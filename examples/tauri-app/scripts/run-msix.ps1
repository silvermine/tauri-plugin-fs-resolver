$ErrorActionPreference = 'Stop'

$AppDir = Resolve-Path (Join-Path $PSScriptRoot '..')
Set-Location $AppDir

$RepoRoot = Resolve-Path (Join-Path $AppDir '..\..')
$TargetDebug = Join-Path $RepoRoot 'target\debug'
$Manifest = Join-Path $AppDir 'Package.appxmanifest'
$Exe = Join-Path $TargetDebug 'tauri-app.exe'

# Use tauri build (not cargo build) so the webview loads frontendDist instead of devUrl.
npm run tauri -- build --debug --no-bundle

if (-not (Test-Path $Exe)) {
    throw "Expected exe not found: $Exe"
}

winapp run $TargetDebug --manifest $Manifest --executable tauri-app.exe
