# Example App

A minimal Tauri + Vue app that demonstrates `tauri-plugin-fs-resolver`.
It displays a list of platform-specific directory identifiers and resolves
each one to its absolute path when tapped/clicked.

On Android, a radio toggle switches between single-path directories
(e.g. `dataDir`) and collection directories (e.g. `externalCacheDirs`).

## Running

From the **repository root**:

```bash
# Desktop
npm run example:dev

# iOS (first time requires init)
npm run example:init:ios
npm run example:dev:ios

# Android (first time requires init)
npm run example:init:android
npm run example:dev:android
```

See the [main README](../../README.md#ios-setup) for iOS device setup
(development team, device trust).

## Linux (GTK / WebKit rendering)

Tauri on Linux uses GTK3 and WebKit2GTK. On some hardware (notably Raspberry Pi
with VC4/V3D Mesa drivers), WebKit's GPU compositing can corrupt the window —
blurry or static-like artifacts instead of the UI.

Disable compositing for the dev run:

```bash
WEBKIT_DISABLE_COMPOSITING_MODE=1 npm run example:dev
```

If that is not enough, try:

```bash
WEBKIT_DISABLE_DMABUF_RENDERER=1 WEBKIT_DISABLE_COMPOSITING_MODE=1 npm run example:dev
```

Or force software OpenGL (slower, but usually clears remaining artifacts):

```bash
LIBGL_ALWAYS_SOFTWARE=1 npm run example:dev
```

To set this permanently, **export** the variable in your shell profile (a bare
assignment in `~/.bashrc` is not passed to child processes such as `npm`):

```bash
export WEBKIT_DISABLE_COMPOSITING_MODE=1
```

A note regarding the (iOS configuration)[./src-tauri/tauri.ios.conf.json]:
iOS bundle ID [cannot contain underscores](https://developer.apple.com/documentation/bundleresources/information-property-list/cfbundleidentifier#Discussion), so we replace them with hyphens.

Q: Why don't we just use a hyphen in the bundle ID?
A: Because Android supports underscores in the package name, but not hyphens.
So one bundle ID had to be changed, and we chose to change iOS.
