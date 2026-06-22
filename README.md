# Tauri Plugin FS Resolver

[![CI][ci-badge]][ci-url]

Platform-specific file system paths for Tauri 2.x apps.

[ci-badge]: https://github.com/silvermine/tauri-plugin-fs-resolver/actions/workflows/ci.yml/badge.svg
[ci-url]: https://github.com/silvermine/tauri-plugin-fs-resolver/actions/workflows/ci.yml

## Features

Provides platform-specific file system path resolution with 1:1 parity to each
OS's native directory APIs.

Tauri's built-in path APIs (and the underlying [`dirs`][dirs-rs] crate) apply a
cross-platform abstraction that can produce [incorrect or inconsistent paths on
mobile][tauri-12276]. For example, on iOS the `dirs` crate applies macOS
conventions — appending `/Library/Application Support/<bundle-id>` to the
sandbox root — rather than using `FileManager` APIs to resolve the correct
sandbox-relative path. Similar inconsistencies appear on Android, where four
distinct directory types can resolve to the same location.

This matters because mobile platforms are opinionated about directory structure
within the app sandbox, and the correct choice of directory has real
consequences: on iOS, files in `Documents/` are included in device backups and
visible in the Files app, while `Library/Application Support/` is backed up but
hidden. On Android, scoped storage rules and manufacturer-specific behavior
affect where media and app data should be persisted.

This plugin bypasses the abstraction layer and resolves paths directly through
each platform's native APIs (`FileManager` on Apple, `Context` on Android,
Known Folders / `ApplicationData` on Windows), so the resolved path always
matches what the OS itself would return.

This plugin also supports paths for Windows apps packaged as both MSI and MSIX.
MSI paths are supported with Win32Paths, and MSIX paths with WindowsApplicationDataPaths.

[dirs-rs]: https://github.com/dirs-dev/dirs-rs
[tauri-12276]: https://github.com/tauri-apps/tauri/issues/12276

| Platform  | Supported |
| --------- | --------- |
| Linux     | ✓         |
| Windows   | ✓         |
| macOS     | ✓         |
| Android¹  | ✓         |
| iOS²      | ✓         |

## Getting Started

### Installation

1. Install NPM dependencies:

   ```bash
   npm install
   ```

2. Build the TypeScript bindings:

   ```bash
   npm run build
   ```

3. Build the Rust plugin:

   ```bash
   cargo build
   ```

### Tests

Run Rust tests:

```bash
cargo test
```

Run Typescript tests:

```ts
npm run test
```

#### Testing mappings in your app (Rust)

If your app defines `CrossPlatformMapping` values and resolves them through
`PathResolver`, you can unit-test that logic without calling real OS path APIs.
The `fs-resolver` crate exposes `PathResolver::new_for_test` behind the
optional `test-helpers` feature. Pass a synthetic OS string and stub resolve
functions; the resolver then behaves as if it were running on that platform.

`new_for_test` is compiled only when `test-helpers` is enabled or when running
`fs-resolver`'s own unit tests (`#[cfg(test)]`). Consumer crates do not get
`cfg(test)` when built as dependencies, so enable the feature explicitly.

Add `fs-resolver` with the feature as a `dev-dependency` in your
`src-tauri/Cargo.toml`. Because Cargo feature unification is additive across the
whole build graph, putting `test-helpers` under `[dependencies]` would compile
`new_for_test` into release builds. Use `[dev-dependencies]` even when tests
live in `#[cfg(test)]` modules in the same crate (as in the example app). Only
use a regular dependency if production code also needs `fs-resolver` directly
(rare):

```toml
[dev-dependencies]
fs-resolver = { path = "../crates/fs-resolver", features = ["test-helpers"] }
```

Construct a resolver with stub closures and assert on `resolve_mapping` or the
per-platform methods:

```rust
use fs_resolver::{
   AndroidPath, AndroidPathCollection, CrossPlatformMapping, IosPath,
   LinuxPath, MacPath, PathResolver, PlatformMapping, Result, Win32Path,
   WindowsPath,
};
use std::path::PathBuf;

fn create_test_resolver(platform: &str) -> PathResolver {
   let resolve_android = Box::new(|path: &AndroidPath| -> Result<PathBuf> {
      Ok(PathBuf::from(format!("android/{}", path)))
   });
   let resolve_android_path_collection = Box::new(
      |collection: &AndroidPathCollection| -> Result<Vec<PathBuf>> {
         Ok(vec![PathBuf::from(format!("android/{}", collection))])
      },
   );
   let resolve_ios = Box::new(|path: &IosPath| -> Result<PathBuf> {
      Ok(PathBuf::from(format!("ios/{}", path)))
   });
   let resolve_linux = Box::new(|path: &LinuxPath| -> Result<PathBuf> {
      Ok(PathBuf::from(format!("linux/{}", path)))
   });
   let resolve_mac = Box::new(|path: &MacPath| -> Result<PathBuf> {
      Ok(PathBuf::from(format!("apple/{}", path)))
   });
   let resolve_windows = Box::new(|path: &WindowsPath| -> Result<PathBuf> {
      Ok(PathBuf::from(format!("windows/{}", path)))
   });

   PathResolver::new_for_test(
      platform.to_string(),
      resolve_android,
      resolve_android_path_collection,
      resolve_ios,
      resolve_linux,
      resolve_mac,
      resolve_windows,
   )
}

#[test]
fn resolves_mapping_on_android() {
   let resolver = create_test_resolver("android");
   let mapping = CrossPlatformMapping {
      android: Some(PlatformMapping {
         platform_path: AndroidPath::DataDir,
         relative_path: Some("data".to_string()),
      }),
      ios: None,
      linux: None,
      macos: None,
      windows: None,
   };

   assert_eq!(
      resolver.resolve_mapping(&mapping).unwrap(),
      PathBuf::from("android/dataDir/data"),
   );
}
```

See [examples/tauri-app/src-tauri/src/lib.rs](examples/tauri-app/src-tauri/src/lib.rs)
for a fuller example that exercises per-platform resolve methods across all
supported OS values.

Do not enable `test-helpers` in release builds of apps that ship to users — it
is intended for test and development use only.

## Install

_This plugin requires a Rust version of at least **1.94.0**_

### Rust

Add the plugin to your `Cargo.toml`:

`src-tauri/Cargo.toml`

```toml
[dependencies]
tauri-plugin-fs-resolver = { git = "https://github.com/silvermine/tauri-plugin-fs-resolver" }
```

### JavaScript/TypeScript

Install the JavaScript bindings:

```sh
npm install @silvermine/tauri-plugin-fs-resolver
```

## Usage

### Prerequisites

Initialize the plugin in your `tauri::Builder`:

```rust
fn main() {
   tauri::Builder::default()
      .plugin(tauri_plugin_fs_resolver::init())
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
```

### API

The plugin exposes two levels of API:

1. **`CrossPlatformMapping`** — a cross-platform path definition that maps each
   platform to its correct path enum variant. Define the mapping once, then
   call `resolveMapping()` (TypeScript) or `PathResolver::resolve_mapping()` (Rust) at
   runtime; the current platform is detected automatically and the
   corresponding resolve function is called. If the current platform has no
   entry in the mapping, an error is returned. This is the recommended entry
   point for most use cases.

2. **Platform-specific resolve functions** — each function targets a single
   platform and accepts that platform's path enum. These are the low-level
   building blocks. Calling a resolve function on the wrong platform throws an
   error (TypeScript) or returns an `Err` (Rust).

#### Architecture

The Rust and TypeScript APIs are shaped differently to fit each language's
idioms while sharing the same IPC commands underneath.

**Rust** exposes a single `PathResolver` struct. It owns the current OS
string, holds the platform-specific resolve functions, and provides methods
for both cross-platform mapping resolution (`resolve_mapping`) and direct
per-platform resolution (`resolve_ios`, `resolve_mac`, `resolve_android`,
`resolve_windows`, `resolve_android_path_collection`). Callers construct it
once with `PathResolver::new(bundle_identifier)?` (typically Tauri's
`config().identifier`) and use that instance for all resolution.

**TypeScript** exposes individual async functions (`resolveIosPath`,
`resolveMacPath`, `resolveAndroidPath`, `resolveWindowsPath`,
`resolveAndroidPathCollection`) that each call a corresponding Tauri IPC command, plus a
`resolveMapping()` helper and `CrossPlatformMapping` type for cross-platform resolution.
Because Tauri IPC can only invoke flat commands (not methods on a Rust struct), the
TypeScript layer does not mirror the `PathResolver` struct directly — the individual
functions are the natural binding to the IPC surface.

> **Platform gating:** Both layers check the current OS before making a
> resolve call. The TypeScript functions check `platform()` before calling
> `invoke()` to avoid an unnecessary IPC round trip when running on the wrong
> platform. The Rust side also validates the OS, so the check is enforced
> regardless of how the command is invoked.

#### CrossPlatformMapping

Use `CrossPlatformMapping` when your app targets multiple platforms and you want a
single definition that resolves to the right directory on each one. Each platform
entry is a `PlatformMapping` with a required `platform_path` and an optional
`relativePath` in TypeScript (or `relative_path` in Rust). Top-level platform
fields are optional — only provide entries for the platforms you ship on.

Mapping types are **not** IPC payloads. Only individual path enums cross the plugin
boundary via the per-platform resolve commands. TypeScript `resolveMapping()` and Rust
`PathResolver::resolve_mapping()` are parallel in-process helpers with the same
semantics — each picks the current platform's entry, resolves the base path enum,
then optionally joins the relative segment. The shared field name for the platform
enum is `platform_path` on both sides; relative segments follow each language's
casing convention (`relativePath` / `relative_path`).

This is the main intended usage for this library. The goal is for developers to
define once what the platform-specific paths should be, and the library will
handle resolving to the path of the current OS. Callers don't need to write
platform-branching logic themselves unless they want to.

**Relative paths:** Set `relativePath` when your mapping should resolve to a
subfolder under the platform base directory. For example, if the app always
stores user data in a `data/` folder and the mapping should answer "what is the
parent directory of `data/`?", set the base path enum on `platform_path` and
`relativePath: 'data'`. The resolver joins the resolved base path with the
relative segment (via Tauri's `join` in TypeScript, `PathBuf::join` in Rust).

**JavaScript / TypeScript**

```typescript
import { resolveMapping } from '@silvermine/tauri-plugin-fs-resolver';
import type { CrossPlatformMapping } from '@silvermine/tauri-plugin-fs-resolver';
import {
   IosPath,
   MacPath,
   AndroidPath,
   LinuxPath,
   Win32Path,
} from '@silvermine/tauri-plugin-fs-resolver/types';

const tempDir: CrossPlatformMapping = {
   android: { platform_path: AndroidPath.CacheDir },
   ios: { platform_path: IosPath.CachesDirectory },
   linux: { platform_path: LinuxPath.CacheHomeForCurrentApp },
   macos: { platform_path: MacPath.CachesDirectory },
   windows: { platform_path: { win32: Win32Path.LocalAppDataForCurrentApp } },
};

const downloadsDir: CrossPlatformMapping = {
   ios: { platform_path: IosPath.DownloadsDirectory },
   macos: { platform_path: MacPath.DownloadsDirectory },
   linux: { platform_path: LinuxPath.DownloadDir },
   android: { platform_path: AndroidPath.ExternalFilesDirectoryDownloads },
   windows: { platform_path: { win32: Win32Path.Downloads } },
};

const dataDir: CrossPlatformMapping = {
   android: { platform_path: AndroidPath.FilesDir, relativePath: 'data' },
   ios: { platform_path: IosPath.ApplicationSupportDirectory, relativePath: 'data' },
   linux: { platform_path: LinuxPath.DataHomeForCurrentApp, relativePath: 'data' },
   macos: { platform_path: MacPath.ApplicationSupportDirectoryForCurrentApp, relativePath: 'data' },
   windows: { platform_path: { win32: Win32Path.LocalAppDataForCurrentApp }, relativePath: 'data' },
};

// These functions are what should actually be called by the rest of the app.
export async function getTempDir() {
   return await resolveMapping(tempDir);
}

export async function getDownloadsDir() {
   return await resolveMapping(downloadsDir);
}

export async function getDataDir() {
   return await resolveMapping(dataDir);
}
```

**Rust**

```rust
use fs_resolver::{
   PathResolver, CrossPlatformMapping, PlatformMapping,
   IosPath, LinuxPath, MacPath, AndroidPath, WindowsPath, Win32Path,
};

let resolver = PathResolver::new("com.example.app".to_string())?;

let temp_dir = CrossPlatformMapping {
   android: Some(PlatformMapping {
      platform_path: AndroidPath::CacheDir,
      relative_path: None,
   }),
   ios: Some(PlatformMapping {
      platform_path: IosPath::CachesDirectory,
      relative_path: None,
   }),
   linux: Some(PlatformMapping {
      platform_path: LinuxPath::CacheHomeForCurrentApp,
      relative_path: None,
   }),
   macos: Some(PlatformMapping {
      platform_path: MacPath::CachesDirectory,
      relative_path: None,
   }),
   windows: Some(PlatformMapping {
      platform_path: WindowsPath::Win32(Win32Path::LocalAppDataForCurrentApp),
      relative_path: None,
   }),
};

let downloads_dir = CrossPlatformMapping {
   android: Some(PlatformMapping {
      platform_path: AndroidPath::ExternalFilesDirectoryDownloads,
      relative_path: None,
   }),
   ios: Some(PlatformMapping {
      platform_path: IosPath::DownloadsDirectory,
      relative_path: None,
   }),
   linux: Some(PlatformMapping {
      platform_path: LinuxPath::DownloadDir,
      relative_path: None,
   }),
   macos: Some(PlatformMapping {
      platform_path: MacPath::DownloadsDirectory,
      relative_path: None,
   }),
   windows: Some(PlatformMapping {
      platform_path: WindowsPath::Win32(Win32Path::Downloads),
      relative_path: None,
   }),
};

let data_dir = CrossPlatformMapping {
   android: Some(PlatformMapping {
      platform_path: AndroidPath::FilesDir,
      relative_path: Some("data".to_string()),
   }),
   ios: Some(PlatformMapping {
      platform_path: IosPath::ApplicationSupportDirectory,
      relative_path: Some("data".to_string()),
   }),
   linux: Some(PlatformMapping {
      platform_path: LinuxPath::DataHomeForCurrentApp,
      relative_path: Some("data".to_string()),
   }),
   macos: Some(PlatformMapping {
      platform_path: MacPath::ApplicationSupportDirectoryForCurrentApp,
      relative_path: Some("data".to_string()),
   }),
   windows: Some(PlatformMapping {
      platform_path: WindowsPath::Win32(Win32Path::LocalAppDataForCurrentApp),
      relative_path: Some("data".to_string()),
   }),
};

// These methods are what should actually be called by the rest of the app.
pub fn temp_dir() -> Result<PathBuf> {
   resolver.resolve_mapping(&temp_dir)
}

pub fn downloads_dir() -> Result<PathBuf> {
   resolver.resolve_mapping(&downloads_dir)
}

pub fn data_dir() -> Result<PathBuf> {
   resolver.resolve_mapping(&data_dir)
}
```

#### Platform-specific resolve functions

For cases where you only need a single platform, or need finer control than
`CrossPlatformMapping` provides (e.g. resolving a path collection on Android), use the
resolve functions directly.

**JavaScript / TypeScript**

All resolve functions are async and platform-gated — calling one on the wrong
platform throws an error immediately without an IPC round trip.

```typescript
// Android
import {
   resolveAndroidPath,
   resolveAndroidPathCollection,
} from '@silvermine/tauri-plugin-fs-resolver';
import {
   AndroidPath,
   AndroidPathCollection,
} from '@silvermine/tauri-plugin-fs-resolver/types';

const cacheDir = await resolveAndroidPath(AndroidPath.CacheDir);
const filesDir = await resolveAndroidPath(AndroidPath.FilesDir);
const pictures = await resolveAndroidPath(AndroidPath.ExternalFilesDirectoryPictures);

const allExternalCaches = await resolveAndroidPathCollection(
   AndroidPathCollection.ExternalCacheDirs,
);
const allMediaDirs = await resolveAndroidPathCollection(
   AndroidPathCollection.ExternalMediaDirs,
);
```

```typescript
// iOS
import { resolveIosPath } from '@silvermine/tauri-plugin-fs-resolver';
import { IosPath } from '@silvermine/tauri-plugin-fs-resolver/types';

const library = await resolveIosPath(IosPath.LibraryDirectory);
const appSupport = await resolveIosPath(IosPath.ApplicationSupportDirectory);
const caches = await resolveIosPath(IosPath.CachesDirectory);
const documents = await resolveIosPath(IosPath.DocumentDirectory);
const downloads = await resolveIosPath(IosPath.DownloadsDirectory);
```

```typescript
// Linux
import { resolveLinuxPath } from '@silvermine/tauri-plugin-fs-resolver';
import { LinuxPath } from '@silvermine/tauri-plugin-fs-resolver/types';

const data = await resolveLinuxPath(LinuxPath.DataHomeForCurrentApp);
const caches = await resolveLinuxPath(LinuxPath.CacheHomeForCurrentApp);
const documents = await resolveLinuxPath(LinuxPath.DocumentDir);
const downloads = await resolveLinuxPath(LinuxPath.DownloadDir);
```

```typescript
// macOS
import { resolveMacPath } from '@silvermine/tauri-plugin-fs-resolver';
import { MacPath } from '@silvermine/tauri-plugin-fs-resolver/types';

const library = await resolveMacPath(MacPath.LibraryDirectory);
const appSupport = await resolveMacPath(MacPath.ApplicationSupportDirectoryForCurrentApp);
const caches = await resolveMacPath(MacPath.CachesDirectoryForCurrentApp);
const documents = await resolveMacPath(MacPath.DocumentDirectory);
const downloads = await resolveMacPath(MacPath.DownloadsDirectory);
```

```typescript
// Windows (Win32 / MSI)
import { resolveWindowsPath } from '@silvermine/tauri-plugin-fs-resolver';
import { Win32Path } from '@silvermine/tauri-plugin-fs-resolver/types';

const appData = await resolveWindowsPath({ win32: Win32Path.RoamingAppDataForCurrentApp });
const localAppData = await resolveWindowsPath({ win32: Win32Path.LocalAppDataForCurrentApp });
const documents = await resolveWindowsPath({ win32: Win32Path.Documents });
```

```typescript
// Windows (MSIX)
import { resolveWindowsPath } from '@silvermine/tauri-plugin-fs-resolver';
import { WindowsApplicationDataPath } from '@silvermine/tauri-plugin-fs-resolver/types';

const localFolder = await resolveWindowsPath({ winMsix: WindowsApplicationDataPath.LocalFolder });
const roamingFolder = await resolveWindowsPath({ winMsix: WindowsApplicationDataPath.RoamingFolder });
const tempFolder = await resolveWindowsPath({ winMsix: WindowsApplicationDataPath.TemporaryFolder });
```

**Rust**

All resolution goes through a `PathResolver` instance. Each method validates
that the current OS matches the target platform and returns `Result<PathBuf>`.

```rust
use fs_resolver::{PathResolver, AndroidPath, IosPath, LinuxPath, MacPath, Win32Path, WindowsApplicationDataPath, WindowsPath};

let resolver = PathResolver::new("com.example.app".to_string())?;

// Android
let cache = resolver.resolve_android(&AndroidPath::CacheDir)?;
let files = resolver.resolve_android(&AndroidPath::FilesDir)?;

// iOS
let library = resolver.resolve_ios(&IosPath::LibraryDirectory)?;
let app_support = resolver.resolve_ios(&IosPath::ApplicationSupportDirectory)?;
let caches = resolver.resolve_ios(&IosPath::CachesDirectory)?;

// Linux
let config = resolver.resolve_linux(&LinuxPath::ConfigHomeForCurrentApp)?;
let desktop = resolver.resolve_linux(&LinuxPath::DesktopDir)?;

// MacOS
let library = resolver.resolve_mac(&MacPath::LibraryDirectory)?;
let app_support = resolver.resolve_mac(&MacPath::ApplicationSupportDirectoryForCurrentApp)?;
let caches = resolver.resolve_mac(&MacPath::CachesDirectoryForCurrentApp)?;

// Windows (Win32 / MSI)
let app_data = resolver.resolve_windows(&WindowsPath::Win32(Win32Path::RoamingAppDataForCurrentApp))?;
let documents = resolver.resolve_windows(&WindowsPath::Win32(Win32Path::Documents))?;

// Windows (MSIX)
let local_folder = resolver.resolve_windows(&WindowsPath::WinMsix(WindowsApplicationDataPath::LocalFolder))?;
let temp_folder = resolver.resolve_windows(&WindowsPath::WinMsix(WindowsApplicationDataPath::TemporaryFolder))?;
```

### Implementation

| Platform | Resolution strategy                                              |
|----------|------------------------------------------------------------------|
| macOS    | Native calls via `objc2-foundation`                              |
| iOS      | Native calls via `objc2-foundation`                              |
| Linux    | Rust `std::env` and XDG conventions                              |
| Windows  | `SHGetKnownFolderPath` (Win32) or WinRT `ApplicationData` (MSIX) |
| Android  | JNI bridge to Kotlin via Tauri `PluginHandle`                    |

On all platforms except Android, paths are resolved directly in Rust
using native bindings or standard library APIs. No Tauri dependency is
required at the resolver level for these platforms.

Android requires crossing the JNI boundary to call `Context` methods
(e.g. `getFilesDir()`, `getExternalCacheDirs()`) that are only
available in the Kotlin runtime. At plugin initialization, the Tauri
`PluginHandle` is captured in closures and injected into the
`PathResolver`, keeping the public API free of Tauri types on all
platforms.

#### Windows paths

Windows path resolution is a **tagged union** (`WindowsPath`) with two variants.
Callers choose the variant that matches how the app is packaged.
If the incorrect variant is used in the app, an exception is thrown.
For example, if the user attempts to resolve a `Win32Path` in an MSIX packaged
app (or vice versa), an exception will be thrown.

| Variant | Serde / TypeScript tag | Enum | Resolution API |
| ------- | ---------------------- | ---- | -------------- |
| Unpackaged Win32 / MSI | `win32` | `Win32Path` | `SHGetKnownFolderPath` ([KNOWNFOLDERID](https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid)) |
| MSIX (package identity) | `winMsix` | `WindowsApplicationDataPath` | WinRT [`ApplicationData`](https://learn.microsoft.com/en-us/uwp/api/windows.storage.applicationdata) |

**`Win32Path`** covers standard desktop known folders (`Documents`, `LocalAppData`,
`Downloads`, etc.). Use this for unpackaged apps, MSI installs, and any Win32
process without package identity. Windows has no API to detect MSI specifically;
all unpackaged Win32 processes (including MSI) lack package identity and share
the same path model.

**`WindowsApplicationDataPath`** covers the five app-container folders exposed
by `ApplicationData::Current()` (`LocalFolder`, `RoamingFolder`,
`LocalCacheFolder`, `TemporaryFolder`, `SharedLocalFolder`). Use this when the
app ships as MSIX (Microsoft Store, sideloaded `.msix`, or loose-layout debug
runs with package identity). These paths live under
`AppData\Local\Packages\<package-id>\…` and are the correct locations for
app-private data in a sandboxed package.

At runtime the resolver validates that the chosen variant matches the process
context:

   * Resolving a `win32` path while running inside an MSIX package returns
     `Win32PathInvokedFromMsixPackagedContext`.
   * Resolving a `winMsix` path outside a package returns
     `WindowsApplicationDataPathInvokedFromWin32Context`.

Package identity is detected by whether `ApplicationData::Current()` succeeds —
the same signal described in Microsoft's [winapp CLI + Tauri
guide](https://learn.microsoft.com/en-us/windows/apps/dev-tools/winapp-cli/guides/tauri).

**TypeScript**

```typescript
import { Win32Path, WindowsApplicationDataPath } from '@silvermine/tauri-plugin-fs-resolver/types';

// Unpackaged / MSI — known folders
const downloads = { win32: Win32Path.Downloads };

// MSIX — app-container folders
const appData = { winMsix: WindowsApplicationDataPath.LocalFolder };
```

**Rust**

```rust
use fs_resolver::{WindowsPath, Win32Path, WindowsApplicationDataPath};

let downloads = WindowsPath::Win32(Win32Path::Downloads);
let app_data = WindowsPath::WinMsix(WindowsApplicationDataPath::LocalFolder);
```

#### Linux paths

Linux path resolution follows the [XDG Base Directory
Specification](https://specifications.freedesktop.org/basedir-spec/latest/) and
[XDG User Directories](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/).

#### Desktop app-scoped directories

On desktop platforms, `*ForCurrentApp` variants append your app identifier
(Tauri `config().identifier`) to the shared base directory, matching Tauri's
`app_data_dir`, `app_config_dir`, and `app_cache_dir` behavior where each
variant maps to the corresponding base (for example, Windows
`LocalAppDataForCurrentApp` matches `app_local_data_dir`; Tauri's
`app_cache_dir` is `<app-id>/cache` under that base).

1. **macOS:** `MacPath::ApplicationSupportDirectoryForCurrentApp` resolves to
   `~/Library/Application Support/<bundle-id>` and
   `MacPath::CachesDirectoryForCurrentApp` resolves to `~/Library/Caches/<bundle-id>`.
   Source: Apple's `SearchPathDirectory` documentation
   ([ApplicationSupportDirectory][apple-app-support-dir],
   [CachesDirectory][apple-caches-dir]).
1. **Linux:** `LinuxPath::*ForCurrentApp` variants resolve to
   `$XDG_{DATA,CONFIG,CACHE,STATE}_HOME/<app-id>`, per the XDG Base Directory spec
   ([variables][xdg-variables]).
1. **Windows (Win32 / MSI):** `Win32Path::{RoamingAppData,LocalAppData}ForCurrentApp`
   resolve to `%APPDATA%/<app-id>` and `%LOCALAPPDATA%/<app-id>`, using KNOWNFOLDERID
   ([Known Folder IDs][win32-known-folder-ids]).

<!-- markdownlint-disable MD013 -->
[apple-app-support-dir]: https://developer.apple.com/documentation/foundation/filemanager/searchpathdirectory/applicationsupportdirectory
[apple-caches-dir]: https://developer.apple.com/documentation/foundation/filemanager/searchpathdirectory/cachesdirectory
[xdg-variables]: https://specifications.freedesktop.org/basedir-spec/latest/#variables
[win32-known-folder-ids]: https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid
<!-- markdownlint-enable MD013 -->

`LinuxPath` base variants return shared XDG roots (e.g. `DataHome` → `~/.local/share`).
If you want an app-specific directory, use the `*ForCurrentApp` variants (e.g.
`DataHomeForCurrentApp` → `~/.local/share/<app-id>`), or append your identifier
manually.

| Category | Variants | Source |
| -------- | -------- | ------ |
| XDG base | `DataHome`, `DataHomeForCurrentApp`, `ConfigHome`, `ConfigHomeForCurrentApp`, `CacheHome`, `CacheHomeForCurrentApp`, `StateHome`, `StateHomeForCurrentApp`, `RuntimeDir`, `Home`, `ExecutableDir`, `FontDir` | `$XDG_*` env vars with spec-defined fallbacks |
| User dirs | `DesktopDir`, `DocumentDir`, `DownloadDir`, `MusicDir`, `PictureDir`, `VideoDir`, `TemplateDir`, `PublicDir` | `~/.config/user-dirs.dirs` |

`RuntimeDir` (`$XDG_RUNTIME_DIR`) has no fallback — it must be set by
pam/systemd. Missing required variables return `LinuxEnvironmentMissing`.

Flatpak and Snap runtimes remap `$XDG_*` variables to sandbox paths
automatically; no special handling is needed in the plugin.

### Examples

Check out the [examples/tauri-app](examples/tauri-app) directory for a working example of
how to use this plugin.

To run the example app:

```bash
# Desktop
npm run example:dev

# Windows MSIX (package identity via winapp CLI)
npm run example:dev:msix

# iOS
npm run example:init:ios   # first time only
npm run example:dev:ios

# Android
npm run example:init:android   # first time only
npm run example:dev:android
```

#### Known Linux Issue

On some Linux devices (e.g. Raspberry Pi), the example window may show WebKit
GPU corruption (blurry or static-like UI). Set `WEBKIT_DISABLE_COMPOSITING_MODE=1`
when running the dev server. See
[Linux (GTK / WebKit rendering)](examples/tauri-app/README.md#linux-gtk--webkit-rendering)
in the example app README for fallbacks and how to export the variable in your
shell.

#### Windows MSIX testing

The example app includes a `Package.appxmanifest` and a `dev:msix` script
that uses the [winapp CLI][winapp-tauri] to build the app and launch it with
package identity (`winapp run`). From the repo root:

```bash
npm run example:dev:msix
```

This runs `examples/tauri-app/scripts/run-msix.ps1`, which debug-builds the
example, then registers and launches it as a loose-layout MSIX package. In the
app UI, switch the **WinMsix** radio button to exercise
`WindowsApplicationDataPath` variants; **Win32** covers known-folder paths for
unpackaged runs (`npm run example:dev`).

Prerequisites: Windows 11, [winapp CLI][winapp-tauri]
(`winget install microsoft.winappcli --source winget`), and PowerShell.

[winapp-tauri]: https://learn.microsoft.com/en-us/windows/apps/dev-tools/winapp-cli/guides/tauri

#### iOS Setup

Before running on iOS, you must set your Apple Development Team ID in
`examples/tauri-app/src-tauri/gen/apple/tauri-app.xcodeproj/project.pbxproj`.
The best way to do this is to open this file in Xcode and set the Team in `Signing &
Capabilities`.

When deploying to a physical iOS device, you may also need to trust the developer
certificate on the device: go to **Settings > General > VPN & Device Management**,
select your developer profile, and tap **Trust**.

## Development Standards

This project follows the
[Silvermine standardization](https://github.com/silvermine/standardization)
guidelines. Key standards include:

   * **EditorConfig**: Consistent editor settings across the team
   * **Markdownlint**: Markdown linting for documentation
   * **Commitlint**: Conventional commit message format
   * **Code Style**: 3-space indentation, LF line endings

### Running Standards Checks

```bash
npm run standards
```

## License

MIT

## Contributing

Contributions are welcome! Please follow the established coding standards and commit
message conventions.
