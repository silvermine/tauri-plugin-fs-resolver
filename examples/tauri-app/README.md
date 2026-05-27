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

A note regarding the (iOS configuration)[./src-tauri/tauri.ios.conf.json]:
iOS bundle ID [cannot contain underscores](https://developer.apple.com/documentation/bundleresources/information-property-list/cfbundleidentifier#Discussion), so we replace them with hyphens.

Q: Why don't we just use a hyphen in the bundle ID?
A: Because Android supports underscores in the package name, but not hyphens.
So one bundle ID had to be changed, and we chose to change iOS.
