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

This plugin also supports Windows unpackaged (`Win32Path`) and packaged
(`WindowsApplicationDataPath`) paths. On a `CrossPlatformMapping`, define both
`win32` and `winPackaged` entries when the same app may run unpackaged or with
package identity; `resolveMapping` / `resolve_mapping` picks which one to use
from `FsEnvironment`.

[dirs-rs]: https://github.com/dirs-dev/dirs-rs
[tauri-12276]: https://github.com/tauri-apps/tauri/issues/12276

| Platform | Supported |
| -------- | --------- |
| Linux    | ✓         |
| Windows  | ✓         |
| macOS    | ✓         |
| Android  | ✓¹        |
| iOS      | ✓¹        |

¹ Mobile requires Tauri mobile targets. See [Examples](#examples) for init and
dev scripts.

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

Run TypeScript tests:

```bash
npm run test
```

#### Testing mappings in your app (Rust)

If your app defines `CrossPlatformMapping` values and resolves them through
`PathResolver`, you can unit-test that logic without calling real OS path APIs.
The `fs-resolver` crate exposes `PathResolver::new_for_test` behind the
optional `test-helpers` feature. Pass a synthetic `FsEnvironment` and stub
resolve functions; the resolver then behaves as if it were running in that
environment.

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
   AndroidPath, AndroidPathCollection, CrossPlatformMapping, FsEnvironment,
   IosPath, LinuxPath, MacPath, PathResolver, PlatformMapping, Result, Win32Path,
   WindowsApplicationDataPath,
};
use std::path::PathBuf;

fn create_test_resolver(environment: FsEnvironment) -> PathResolver {
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
   let resolve_win32 = Box::new(|path: &Win32Path| -> Result<PathBuf> {
      Ok(PathBuf::from(format!("win32/{}", path)))
   });
   let resolve_windows_application_data =
      Box::new(|path: &WindowsApplicationDataPath| -> Result<PathBuf> {
         Ok(PathBuf::from(format!("windowsApplicationData/{}", path)))
      });

   PathResolver::new_for_test(
      environment,
      resolve_android,
      resolve_android_path_collection,
      resolve_ios,
      resolve_linux,
      resolve_mac,
      resolve_win32,
      resolve_windows_application_data,
   )
}

#[test]
fn resolves_mapping_on_android() {
   let resolver = create_test_resolver(FsEnvironment::Android);
   let mapping = CrossPlatformMapping {
      android: Some(PlatformMapping {
         platform_path: AndroidPath::DataDir,
         relative_path: Some("data".to_string()),
      }),
      ios: None,
      linux: None,
      macos: None,
      win32: None,
      win_packaged: None,
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
npm install tauri-plugin-fs-resolver
```

All functions, types, and enums are exported from the package root (no `/types`
subpath).

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

Grant the plugin permission in your Tauri capabilities file (for example
`src-tauri/capabilities/default.json`):

```json
{
  "permissions": [
    "core:default",
    "fs-resolver:default"
  ]
}
```

`fs-resolver:default` enables all resolve commands. For a granular allow/deny
list, see the
[permissions reference](permissions/autogenerated/reference.md).

### API

The plugin exposes two levels of API:

1. **`CrossPlatformMapping`** — a cross-platform path definition that maps each
   platform to its correct path enum variant. Define the mapping once, then
   call `resolveMapping()` (TypeScript) or `PathResolver::resolve_mapping()` (Rust) at
   runtime; the current environment is detected automatically and the
   corresponding resolve function is called. If the current environment has no
   entry in the mapping, an error is returned. This is the recommended entry
   point for most use cases.

2. **Platform-specific resolve functions** — each function targets a single
   platform and accepts that platform's path enum. These are the low-level
   building blocks. Calling a resolve function on the wrong platform throws an
   error (TypeScript) or returns an `Err` (Rust).

#### Architecture

The Rust and TypeScript APIs are shaped differently to fit each language's
idioms while sharing the same IPC commands underneath.

**Rust** exposes a single `PathResolver` struct. It owns the current
`FsEnvironment`, holds the platform-specific resolve functions, and provides
methods for both cross-platform mapping resolution (`resolve_mapping`) and
direct per-platform resolution (`resolve_ios`, `resolve_mac`, `resolve_linux`,
`resolve_android`, `resolve_win32`, `resolve_windows_application_data`,
`resolve_android_path_collection`, `environment`). Callers construct it once
with `PathResolver::new(bundle_identifier)?` (typically Tauri's
`config().identifier`) and use that instance for all resolution.

**TypeScript** exposes individual async functions (`resolveIosPath`,
`resolveMacPath`, `resolveLinuxPath`, `resolveAndroidPath`, `resolveWin32Path`,
`resolveWindowsApplicationDataPath`, `resolveAndroidPathCollection`,
`getFsEnvironment`) that each call a corresponding Tauri IPC command, plus a
`resolveMapping()` helper and `CrossPlatformMapping` type for cross-platform
resolution. On Windows, `resolveMapping()` uses `getFsEnvironment()` to choose
between `win32` and `winPackaged`. Because Tauri IPC can only invoke flat
commands (not methods on a Rust struct), the TypeScript layer does not mirror
the `PathResolver` struct directly — the individual functions are the natural
binding to the IPC surface.

> **Platform gating:** Both layers check the current `FsEnvironment` before
> making a resolve call. The TypeScript functions call `getFsEnvironment()`
> before `invoke()` to avoid an unnecessary IPC round trip when running in the
> wrong environment. The Rust side also validates the environment, so the check
> is enforced regardless of how the command is invoked.

#### CrossPlatformMapping

Use `CrossPlatformMapping` when your app targets multiple platforms and you want a
single definition that resolves to the right directory on each one. Each platform
entry is a `PlatformMapping` with a required `platformPath` (TypeScript) or
`platform_path` (Rust) and an optional `relativePath` / `relative_path`.
Top-level platform fields are optional — only provide entries for the platforms
you ship on. On Windows, use separate `win32` and `winPackaged` /
`win_packaged` fields when the app may run unpackaged or packaged.

Mapping types are **not** IPC payloads. Only individual path enums cross the plugin
boundary via the per-platform resolve commands. TypeScript `resolveMapping()` and Rust
`PathResolver::resolve_mapping()` are parallel in-process helpers with the same
semantics — each picks the current environment's entry, resolves the base path enum,
then optionally joins the relative segment. Relative segments follow each language's
casing convention (`relativePath` / `relative_path`).
This is the main intended usage for this library. The goal is for developers to
define once what the platform-specific paths should be, and the library will
handle resolving to the path of the current OS. Callers don't need to write
platform-branching logic themselves unless they want to.

**Relative paths:** Set `relativePath` when your mapping should resolve to a
subfolder under the platform base directory. For example, if the app always
stores user data in a `data/` folder and the mapping should answer "what is the
parent directory of `data/`?", set the base path enum on `platformPath` /
`platform_path` and `relativePath: 'data'`. The resolver joins the resolved base
path with the relative segment (via Tauri's `join` in TypeScript,
`PathBuf::join` in Rust).

**JavaScript / TypeScript**

```typescript
import {
   resolveMapping,
   type CrossPlatformMapping,
   IosPath,
   MacPath,
   AndroidPath,
   LinuxPath,
   Win32Path,
   WindowsApplicationDataPath,
} from 'tauri-plugin-fs-resolver';

const tempDir: CrossPlatformMapping = {
   android: { platformPath: AndroidPath.CacheDir },
   ios: { platformPath: IosPath.CachesDirectory },
   linux: { platformPath: LinuxPath.CacheHomeForCurrentApp },
   macos: { platformPath: MacPath.CachesDirectory },
   // Separate unpackaged / packaged entries; resolveMapping picks by FsEnvironment.
   win32: { platformPath: Win32Path.LocalAppDataForCurrentApp },
   winPackaged: {
      kind: 'windowsApplicationData',
      mapping: { platformPath: WindowsApplicationDataPath.LocalCacheFolder },
   },
};

const downloadsDir: CrossPlatformMapping = {
   ios: { platformPath: IosPath.DownloadsDirectory },
   macos: { platformPath: MacPath.DownloadsDirectory },
   linux: { platformPath: LinuxPath.DownloadDir },
   android: { platformPath: AndroidPath.ExternalFilesDirectoryDownloads },
   win32: { platformPath: Win32Path.Downloads },
   // Packaged mappings may use Win32 known folders (e.g. user Downloads), not
   // only ApplicationData — resolveWin32Path is allowed under package identity.
   winPackaged: {
      kind: 'win32',
      mapping: { platformPath: Win32Path.Downloads },
   },
};

const dataDir: CrossPlatformMapping = {
   android: { platformPath: AndroidPath.FilesDir, relativePath: 'data' },
   ios: { platformPath: IosPath.ApplicationSupportDirectory, relativePath: 'data' },
   linux: { platformPath: LinuxPath.DataHomeForCurrentApp, relativePath: 'data' },
   macos: { platformPath: MacPath.ApplicationSupportDirectoryForCurrentApp, relativePath: 'data' },
   win32: { platformPath: Win32Path.LocalAppDataForCurrentApp, relativePath: 'data' },
   winPackaged: {
      kind: 'windowsApplicationData',
      mapping: { platformPath: WindowsApplicationDataPath.LocalFolder, relativePath: 'data' },
   },
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
   PathResolver, CrossPlatformMapping, PlatformMapping, WinPackagedPathMapping,
   IosPath, LinuxPath, MacPath, AndroidPath, Win32Path, WindowsApplicationDataPath,
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
   // Separate unpackaged / packaged entries; resolve_mapping picks by FsEnvironment.
   win32: Some(PlatformMapping {
      platform_path: Win32Path::LocalAppDataForCurrentApp,
      relative_path: None,
   }),
   win_packaged: Some(WinPackagedPathMapping::ApplicationDataPath(
      PlatformMapping {
         platform_path: WindowsApplicationDataPath::LocalCacheFolder,
         relative_path: None,
      },
   )),
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
   win32: Some(PlatformMapping {
      platform_path: Win32Path::Downloads,
      relative_path: None,
   }),
   // Packaged mappings may use Win32 known folders (e.g. user Downloads), not
   // only ApplicationData — resolve_win32 is allowed under package identity.
   // See below for more information.
   win_packaged: Some(WinPackagedPathMapping::Win32Path(PlatformMapping {
      platform_path: Win32Path::Downloads,
      relative_path: None,
   })),
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
   win32: Some(PlatformMapping {
      platform_path: Win32Path::LocalAppDataForCurrentApp,
      relative_path: Some("data".to_string()),
   }),
   win_packaged: Some(WinPackagedPathMapping::ApplicationDataPath(
      PlatformMapping {
         platform_path: WindowsApplicationDataPath::LocalFolder,
         relative_path: Some("data".to_string()),
      },
   )),
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
   AndroidPath,
   AndroidPathCollection,
} from 'tauri-plugin-fs-resolver';

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
import { resolveIosPath, IosPath } from 'tauri-plugin-fs-resolver';

const library = await resolveIosPath(IosPath.LibraryDirectory);
const appSupport = await resolveIosPath(IosPath.ApplicationSupportDirectory);
const caches = await resolveIosPath(IosPath.CachesDirectory);
const documents = await resolveIosPath(IosPath.DocumentDirectory);
const downloads = await resolveIosPath(IosPath.DownloadsDirectory);
```

```typescript
// Linux
import { resolveLinuxPath, LinuxPath } from 'tauri-plugin-fs-resolver';

const data = await resolveLinuxPath(LinuxPath.DataHomeForCurrentApp);
const caches = await resolveLinuxPath(LinuxPath.CacheHomeForCurrentApp);
const documents = await resolveLinuxPath(LinuxPath.DocumentDir);
const downloads = await resolveLinuxPath(LinuxPath.DownloadDir);
```

```typescript
// macOS
import { resolveMacPath, MacPath } from 'tauri-plugin-fs-resolver';

const library = await resolveMacPath(MacPath.LibraryDirectory);
const appSupport = await resolveMacPath(MacPath.ApplicationSupportDirectoryForCurrentApp);
const caches = await resolveMacPath(MacPath.CachesDirectoryForCurrentApp);
const documents = await resolveMacPath(MacPath.DocumentDirectory);
const downloads = await resolveMacPath(MacPath.DownloadsDirectory);
```

```typescript
// Windows (unpackaged)
import { resolveWin32Path, Win32Path } from 'tauri-plugin-fs-resolver';

const appData = await resolveWin32Path(Win32Path.RoamingAppDataForCurrentApp);
const localAppData = await resolveWin32Path(Win32Path.LocalAppDataForCurrentApp);
const documents = await resolveWin32Path(Win32Path.Documents);
```

```typescript
// Windows (packaged)
import {
   resolveWindowsApplicationDataPath,
   WindowsApplicationDataPath,
} from 'tauri-plugin-fs-resolver';

const localFolder = await resolveWindowsApplicationDataPath(
   WindowsApplicationDataPath.LocalFolder,
);
const roamingFolder = await resolveWindowsApplicationDataPath(
   WindowsApplicationDataPath.RoamingFolder,
);
const tempFolder = await resolveWindowsApplicationDataPath(
   WindowsApplicationDataPath.TemporaryFolder,
);
```

**Rust**

All resolution goes through a `PathResolver` instance. Each method validates
that the current environment matches the target platform and returns
`Result<PathBuf>`.

```rust
use fs_resolver::{
   PathResolver, AndroidPath, IosPath, LinuxPath, MacPath, Win32Path,
   WindowsApplicationDataPath,
};

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

// macOS
let library = resolver.resolve_mac(&MacPath::LibraryDirectory)?;
let app_support = resolver.resolve_mac(&MacPath::ApplicationSupportDirectoryForCurrentApp)?;
let caches = resolver.resolve_mac(&MacPath::CachesDirectoryForCurrentApp)?;

// Windows (unpackaged)
let app_data = resolver.resolve_win32(&Win32Path::RoamingAppDataForCurrentApp)?;
let documents = resolver.resolve_win32(&Win32Path::Documents)?;

// Windows (packaged)
let local_folder = resolver.resolve_windows_application_data(
   &WindowsApplicationDataPath::LocalFolder,
)?;
let temp_folder = resolver.resolve_windows_application_data(
   &WindowsApplicationDataPath::TemporaryFolder,
)?;
```

### Implementation

| Platform | Resolution strategy                                              |
|----------|------------------------------------------------------------------|
| macOS    | Native calls via `objc2-foundation`                              |
| iOS      | Native calls via `objc2-foundation`                              |
| Linux    | Rust `std::env` and XDG conventions                              |
| Windows  | `SHGetKnownFolderPath` (Win32) or WinRT `ApplicationData` (packaged) |
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

Windows unpackaged and packaged paths are first-class: use `Win32Path` and
`WindowsApplicationDataPath` directly via `resolveWin32Path` /
`resolveWindowsApplicationDataPath` (or the Rust equivalents). On a
`CrossPlatformMapping`, define separate top-level `win32` and `winPackaged` /
`win_packaged` entries when the same app may run unpackaged or packaged (e.g.
local `cargo run` vs packaged debug with package identity).

`resolveMapping` / `resolve_mapping` detects package identity via
`GetCurrentPackageFullName` and stores the result as `FsEnvironment`
(`win32` or `winpackaged`), then uses the matching mapping field. If that
entry is missing, resolution fails with `PathMappingUndefined`. Call the
per-path resolve functions directly when you already know the packaging
model.

**Win32 paths in packaged apps:** `resolve_win32` / `resolveWin32Path` do
**not** fail just because the process has package identity. Windows still
serves `SHGetKnownFolderPath` in packaged desktop apps. User folders such as
`Documents` and `Downloads` typically resolve to the real user locations. For
app-private storage in a package, prefer `winPackaged` with
`WindowsApplicationDataPath` (or a `WinPackagedPathMapping` Win32 entry when
you intentionally want a known folder inside the packaged process).

Package identity is detected with `GetCurrentPackageFullName` as described in
Microsoft's [detect package identity][detect-package-identity] guide (same
signal used by the [winapp CLI + Tauri guide][winapp-tauri]).

[detect-package-identity]: https://learn.microsoft.com/en-us/windows/msix/detect-package-identity

| Mapping field | Shape | Resolution API |
| ------------- | ----- | -------------- |
| `win32` | `PlatformMapping<Win32Path>` | `SHGetKnownFolderPath` ([KNOWNFOLDERID](https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid)) |
| `winPackaged` / `win_packaged` | `WinPackagedPathMapping` (`win32` or `windowsApplicationData`) | Win32 known folders or WinRT [`ApplicationData`](https://learn.microsoft.com/en-us/uwp/api/windows.storage.applicationdata) |

**`Win32Path`** covers standard desktop known folders (`Documents`,
`LocalAppData`, `Downloads`, etc.). Use this for unpackaged apps, MSI
installs, and any Win32 process without package identity. Windows has no API
to detect MSI specifically; all unpackaged Win32 processes (including MSI)
lack package identity and share the same path model. `resolveWin32Path` is
also allowed when `FsEnvironment` is `winpackaged`.

**`WindowsApplicationDataPath`** covers the five app-container folders
exposed by `ApplicationData::Current()` (`LocalFolder`, `RoamingFolder`,
`LocalCacheFolder`, `TemporaryFolder`, `SharedLocalFolder`). Use this when
the process has package identity (e.g. MSIX Store, sideloaded `.msix`, or
loose-layout debug runs). These paths live under
`AppData\Local\Packages\<package-id>\…` and are the correct locations for
app-private data in a sandboxed package. Resolving ApplicationData outside a
packaged environment fails with an incorrect-environment error.

**`WinPackagedPathMapping`** lets a packaged mapping choose either
ApplicationData (typical) or a Win32 known folder:

   * TypeScript:
     `{ kind: 'windowsApplicationData', mapping: { platformPath, relativePath? } }`
     or `{ kind: 'win32', mapping: { platformPath, relativePath? } }`
   * Rust: `WinPackagedPathMapping::ApplicationDataPath(...)` or
     `WinPackagedPathMapping::Win32Path(...)`

**TypeScript**

```typescript
import {
   resolveWin32Path,
   resolveWindowsApplicationDataPath,
   Win32Path,
   WindowsApplicationDataPath,
   type CrossPlatformMapping,
} from 'tauri-plugin-fs-resolver';

// Direct resolve when packaging model is known
const downloads = await resolveWin32Path(Win32Path.Downloads);
const applicationDataLocal = await resolveWindowsApplicationDataPath(
   WindowsApplicationDataPath.LocalFolder,
);

// Mapping — resolveMapping selects win32 vs winPackaged by FsEnvironment
const appData: CrossPlatformMapping = {
   win32: { platformPath: Win32Path.LocalAppDataForCurrentApp },
   winPackaged: {
      kind: 'windowsApplicationData',
      mapping: { platformPath: WindowsApplicationDataPath.LocalFolder },
   },
};
```

**Rust**

```rust
use fs_resolver::{
   CrossPlatformMapping, PathResolver, PlatformMapping, Win32Path,
   WinPackagedPathMapping, WindowsApplicationDataPath,
};

let resolver = PathResolver::new("com.example.app".to_string())?;

// Direct resolve when packaging model is known
let downloads = resolver.resolve_win32(&Win32Path::Downloads)?;
let application_data_local = resolver.resolve_windows_application_data(
   &WindowsApplicationDataPath::LocalFolder,
)?;

// Mapping — resolve_mapping selects win32 vs win_packaged by FsEnvironment
let app_data = CrossPlatformMapping {
   android: None,
   ios: None,
   linux: None,
   macos: None,
   win32: Some(PlatformMapping {
      platform_path: Win32Path::LocalAppDataForCurrentApp,
      relative_path: None,
   }),
   win_packaged: Some(WinPackagedPathMapping::ApplicationDataPath(
      PlatformMapping {
         platform_path: WindowsApplicationDataPath::LocalFolder,
         relative_path: None,
      },
   )),
};
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
1. **Windows (unpackaged Win32):** `Win32Path::{RoamingAppData,LocalAppData}ForCurrentApp`
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

# Packaged Windows (package identity via winapp CLI / MSIX loose-layout)
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

#### Packaged Windows testing

The example app includes a `Package.appxmanifest` and a `dev:msix` script
that uses the [winapp CLI][winapp-tauri] to build the app and launch it with
package identity (`winapp run`). From the repo root:

```bash
npm run example:dev:msix
```

This runs `examples/tauri-app/scripts/run-msix.ps1`, which debug-builds the
example, then registers and launches it as a loose-layout MSIX package (one
way to get package identity for local testing). In the app UI, switch the
**WinPackaged** radio button to exercise `WindowsApplicationDataPath`
variants; **Win32** covers known-folder paths for unpackaged runs
(`npm run example:dev`). For `CrossPlatformMapping`, define separate `win32`
and `winPackaged` entries; `resolveMapping` picks the matching one from
`FsEnvironment`.

> **Note:** Packaged ApplicationData resolution tries `Current()` then may fall
> back to `CreateForPackageFamily` (see `windows_resolve::get_application_data`).
> Which branch runs is a runtime WinRT decision. The sample MSIX is already
> MediumIL (`packagedClassicApp`) and often succeeds on `Current()`, so there
> is no known deterministic way—via manifest or `dev:msix` alone—to force and
> verify the fallback branch.

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
