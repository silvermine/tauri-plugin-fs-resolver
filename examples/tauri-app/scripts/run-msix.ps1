# Build and run the Tauri app as a packaged MSIX app for local debugging.
#
# Builds the Rust binary, starts the Vite dev server, registers the debug
# build with MSIX package identity, and launches it via `winapp run`.
# The Vite dev server is automatically stopped when the app exits.
#
# Prerequisites:
#   - winapp CLI (installed automatically if missing via winget).

param(
   # Path to the directory containing the built executable.
   # Default: This is Cargo's debug output directory.
   [string]$TargetDir = "target/debug",

   # Path to the MSIX package manifest.
   [string]$Manifest = "Package.appxmanifest",

   # Name of the executable (without path).
   [string]$Executable = "tauri-app.exe"
)

$ErrorActionPreference = 'Stop'

$AppDir = Resolve-Path (Join-Path $PSScriptRoot '..')
$WorkspaceRoot = Resolve-Path (Join-Path $AppDir '..\..')

# Build the Rust binary. A plain debug `cargo build` (not `tauri build`) produces a
# dev-profile binary that loads the app from the live Vite dev server (`devUrl`,
# http://localhost:1420) instead of the bundled `../dist`, which is what enables
# Vite hot-reload during MSIX debugging.
cargo build -p tauri-app
if ($LASTEXITCODE -ne 0) { throw "cargo build failed with exit code $LASTEXITCODE" }

# Resolve TargetDir relative to the Cargo workspace root.
$TargetDir = Join-Path $WorkspaceRoot $TargetDir

# Ensure winapp CLI is installed.
if (-not (Get-Command winapp -ErrorAction SilentlyContinue)) {
   if (![Environment]::UserInteractive) {
      throw "winapp CLI is required but not installed. Install it with: winget install --id Microsoft.WinAppCli --exact --source winget"
   }
   $install = Read-Host "winapp CLI is required but not installed. Install it now? (y/N)"
   if ($install -match '^[Yy]') {
      winget install --id Microsoft.WinAppCli --exact --source winget
      if ($LASTEXITCODE -ne 0) { throw "Failed to install winapp CLI" }
   } else {
      throw "winapp CLI is required. Install it with: winget install --id Microsoft.WinAppCli --exact --source winget"
   }
}

# Start the Vite dev server in the background. The `$vite` handle points at the `cmd`
# wrapper, not the `node`/vite process it spawns, so it is cleaned up by killing the
# whole process tree (see the finally block) rather than the handle alone.
$vite = Start-Process 'cmd' -ArgumentList '/c', 'npx', 'vite' -WorkingDirectory $AppDir -NoNewWindow -PassThru

try {
   # Register and launch the app with MSIX package identity.
   winapp run $TargetDir --manifest $Manifest --executable $Executable
   if ($LASTEXITCODE -ne 0) { throw "winapp run failed with exit code $LASTEXITCODE" }
} finally {
   # Stop the Vite dev server. Kill the whole process tree (`/T`) so the underlying
   # `node`/vite listener is terminated too, not just the `cmd` wrapper. Otherwise Vite
   # is orphaned and keeps port 1420 bound, breaking the next run.
   if (!$vite.HasExited) { taskkill /T /F /PID $vite.Id *> $null }
}
