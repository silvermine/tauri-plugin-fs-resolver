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

## Install

_This plugin requires a Rust version of at least **1.89.0**_

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

1. **`PathMapping`** — a cross-platform path definition that maps each
   platform to its correct path enum variant. Define the mapping once, then
   call `resolve()` at runtime; the current platform is detected automatically
   and the corresponding resolve function is called. If the current platform
   has no entry in the mapping, an error is returned. This is the recommended
   entry point for most use cases.

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
once with `PathResolver::new()` — no arguments needed — and use that
instance for all resolution.

**TypeScript** exposes individual async functions (`resolveIosPath`,
`resolveMacPath`, `resolveAndroidPath`, `resolveWindowsPath`,
`resolveAndroidPathCollection`) that each call a corresponding Tauri IPC command, plus a
`PathMapping` class for cross-platform resolution. Because Tauri IPC can only invoke flat
commands (not methods on a Rust struct), the TypeScript layer does not mirror the
`PathResolver` struct directly — the individual functions are the natural binding to the
IPC surface.

> **Platform gating:** Both layers check the current OS before making a
> resolve call. The TypeScript functions check `platform()` before calling
> `invoke()` to avoid an unnecessary IPC round trip when running on the wrong
> platform. The Rust side also validates the OS, so the check is enforced
> regardless of how the command is invoked.

#### PathMapping

Use `PathMapping` when your app targets multiple platforms and you want a
single definition that resolves to the right directory on each one. All fields
are optional — only provide entries for the platforms you ship on.

This is the main intended usage for this library. The goal is for
developers to define once what the platform-specific paths should be, and the
library will handle resolving to the path of the current OS.

In other words, callers don't need to write any platform-branching logic
themselves unless they want to.

**JavaScript / TypeScript**

```typescript
import { PathMapping } from '@silvermine/tauri-plugin-fs-resolver';
import {
   IosPath,
   MacPath,
   AndroidPath,
   LinuxPath,
   Win32Path,
} from '@silvermine/tauri-plugin-fs-resolver/types';

const tempDir = new PathMapping({
   android: AndroidPath.CacheDir,
   ios: IosPath.CachesDirectory,
   linux: LinuxPath.UserCacheDirectory,
   macos: MacPath.CachesDirectory,
   windows: { win32: Win32Path.LocalAppData },
});

const downloadsDir = new PathMapping({
   ios: IosPath.DownloadsDirectory,
   macos: MacPath.DownloadsDirectory,
   linux: LinuxPath.UserDownloadDirectory,
   android: AndroidPath.ExternalFilesDirectoryDownloads,
   windows: { win32: Win32Path.Downloads },
});

// These methods are what should actually be called by the rest of the app.
export async function getTempDir() {
   return await tempDir.resolve()
}

export async function getTempDir() {
   return await downloadsDir.resolve()
}
```

**Rust**

```rust
use fs_resolver::{PathResolver, PathMapping, IosPath, LinuxPath, MacPath, AndroidPath, WindowsPath, Win32Path, Error};

let resolver = PathResolver::new();

let temp_dir = PathMapping {
   android: Some(AndroidPath::CacheDir),
   ios: Some(IosPath::CachesDirectory),
   linux: Some(LinuxPath::UserCacheDirectory),
   macos: Some(MacPath::CachesDirectory),
   windows: Some(WindowsPath::Win32(Win32Path::LocalAppData)),
};

let downloads_dir = PathMapping {
   android: Some(AndroidPath::ExternalFilesDirectoryDownloads),
   ios: Some(IosPath::DownloadsDirectory),
   linux: Some(LinuxPath::UserDownloadDirectory),
   macos: Some(MacPath::DownloadsDirectory),
   windows: Some(WindowsPath::Win32(Win32Path::Downloads)),
};

// These methods are what should actually be called by the rest of the app.
pub fn temp_dir() -> Result<PathBuf> {
   resolver.resolve_mapping(&temp_dir)
}

pub fn downloads_dir() -> Result<PathBuf> {
   resolver.resolve_mapping(&temp_dir)
}
```

#### Platform-specific resolve functions

For cases where you only need a single platform, or need finer control than
`PathMapping` provides (e.g. resolving a path collection on Android), use the
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

const data = await resolveLinuxPath(LinuxPath.UserDataDirectory);
const caches = await resolveLinuxPath(LinuxPath.UserCacheDirectory);
const documents = await resolveLinuxPath(LinuxPath.UserDocumentDirectory);
const downloads = await resolveLinuxPath(LinuxPath.UserDownloadDirectory);
```

```typescript
// macOS
import { resolveMacPath } from '@silvermine/tauri-plugin-fs-resolver';
import { MacPath } from '@silvermine/tauri-plugin-fs-resolver/types';

const library = await resolveMacPath(MacPath.LibraryDirectory);
const appSupport = await resolveMacPath(MacPath.ApplicationSupportDirectory);
const caches = await resolveMacPath(MacPath.CachesDirectory);
const documents = await resolveMacPath(MacPath.DocumentDirectory);
const downloads = await resolveMacPath(MacPath.DownloadsDirectory);
```

```typescript
// Windows (Win32 / MSI)
import { resolveWindowsPath } from '@silvermine/tauri-plugin-fs-resolver';
import { Win32Path } from '@silvermine/tauri-plugin-fs-resolver/types';

const appData = await resolveWindowsPath({ win32: Win32Path.RoamingAppData });
const localAppData = await resolveWindowsPath({ win32: Win32Path.LocalAppData });
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
use fs_resolver::{PathResolver, AndroidPath, IosPath, MacPath, Win32Path, WindowsPath};

let resolver = PathResolver::new();

// Android
let cache = resolver.resolve_android(&AndroidPath::CacheDir)?;
let files = resolver.resolve_android(&AndroidPath::FilesDir)?;

// iOS
let library = resolver.resolve_ios(&IosPath::LibraryDirectory)?;
let app_support = resolver.resolve_ios(&IosPath::ApplicationSupportDirectory)?;
let caches = resolver.resolve_ios(&IosPath::CachesDirectory)?;

// Linux
let config = resolver.resolve_linux(&LinuxPath::UserConfigDirectory)?;
let desktop = resolver.resolve_linux(&LinuxPath::UserDesktopDirectory)?;

// MacOS
let library = resolver.resolve_mac(&MacPath::LibraryDirectory)?;
let app_support = resolver.resolve_mac(&MacPath::ApplicationSupportDirectory)?;
let caches = resolver.resolve_mac(&MacPath::CachesDirectory)?;

// Windows
let app_data = resolver.resolve_windows(&WindowsPath::Win32(Win32Path::RoamingAppData))?;
let documents = resolver.resolve_windows(&WindowsPath::Win32(Win32Path::Documents))?;
```

### Examples

Check out the [examples/tauri-app](examples/tauri-app) directory for a working example of
how to use this plugin.

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
