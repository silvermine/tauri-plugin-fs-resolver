import {
   getFsEnvironment,
   resolveAndroidPath,
   resolveIosPath,
   resolveLinuxPath,
   resolveMacPath,
   resolveWin32Path,
   resolveWindowsApplicationDataPath,
} from './platform-paths';
import {
   AndroidPath,
   IosPath,
   LinuxPath,
   MacPath,
   Win32Path,
   WindowsApplicationDataPath,
} from './types';
import { join } from '@tauri-apps/api/path';

/**
 * Windows path mapping for WinPackaged apps.
 *
 * In a packaged Windows app, either an ApplicationDataPath or a Win32Path can be used.
 * With this in mind, we need to allow a CrossPlatformMapping to support both
 * of these paths
 * when defining the mapping used for a packaged app.
 */
export type WinPackagedPathMapping = | {
   kind: 'windowsApplicationDataPath';
   mapping: PlatformMapping<WindowsApplicationDataPath>;
} | {
   kind: 'win32';
   mapping: PlatformMapping<Win32Path>;
};

/**
 * Per-platform path mapping: a platform path enum plus an optional relative suffix.
 *
 * These types are resolved in-process on the frontend. They are not sent across IPC;
 * only individual path enums cross the boundary via per-platform resolve commands.
 */
export type PlatformMapping<T extends AndroidPath | IosPath | LinuxPath | MacPath | Win32Path | WindowsApplicationDataPath> = {
   platformPath: T;
   relativePath?: string;
};

/**
 * Cross-platform path definition with optional per-OS mappings.
 *
 * Parallel to the Rust `CrossPlatformMapping` type. Use `resolveMapping()` here or
 * `PathResolver::resolve_mapping()` in Rust — same semantics, separate implementations.
 */
export type CrossPlatformMapping = {
   android?: PlatformMapping<AndroidPath>;
   ios?: PlatformMapping<IosPath>;
   linux?: PlatformMapping<LinuxPath>;
   macos?: PlatformMapping<MacPath>;
   win32?: PlatformMapping<Win32Path>;
   winPackaged?: WinPackagedPathMapping;
};

/**
 * Resolves a cross-platform mapping for the current OS.
 *
 * Picks the mapping entry for the current platform, invokes the corresponding
 * per-platform resolve helper (IPC), then optionally joins `relativePath`.
 */
export async function resolveMapping(mapping: CrossPlatformMapping): Promise<string> {
   const environment = await getFsEnvironment();

   switch (environment) {
      case 'android': {
         const android = mapping.android;

         if (android) {
            return await resolveWithRelativePath(
               () => { return resolveAndroidPath(android.platformPath); },
               android.relativePath
            );
         }

         throw new Error('No path defined for Android');
      }

      case 'ios': {
         const ios = mapping.ios;

         if (ios) {
            return await resolveWithRelativePath(
               () => { return resolveIosPath(ios.platformPath); },
               ios.relativePath
            );
         }

         throw new Error('No path defined for iOS');
      }

      case 'linux': {
         const linux = mapping.linux;

         if (linux) {
            return await resolveWithRelativePath(
               () => { return resolveLinuxPath(linux.platformPath); },
               linux.relativePath
            );
         }

         throw new Error('No path defined for Linux');
      }

      case 'macos': {
         const macos = mapping.macos;

         if (macos) {
            return await resolveWithRelativePath(
               () => { return resolveMacPath(macos.platformPath); },
               macos.relativePath
            );
         }

         throw new Error('No path defined for macOS');
      }

      case 'win32': {
         const win32 = mapping.win32;

         if (win32) {
            return await resolveWithRelativePath(
               () => { return resolveWin32Path(win32.platformPath); },
               win32.relativePath
            );
         }

         throw new Error('No path defined for Win32');
      }

      case 'winpackaged': {
         const winPackaged = mapping.winPackaged;

         if (winPackaged) {
            if (winPackaged.kind === 'win32') {
               return await resolveWithRelativePath(
                  () => { return resolveWin32Path(winPackaged.mapping.platformPath); },
                  winPackaged.mapping.relativePath
               );
            } else if (winPackaged.kind === 'windowsApplicationDataPath') {
               return await resolveWithRelativePath(
                  () => { return resolveWindowsApplicationDataPath(winPackaged.mapping.platformPath); },
                  winPackaged.mapping.relativePath
               );
            }
         }

         throw new Error(`winPackaged mapping is missing while running as WinPackaged.
resolveMapping does not fall back to win32. Set winPackaged explicitly:
{ kind: 'windowsApplicationDataPath', mapping: { ... } } or
{ kind: 'win32', mapping: { ... } } (same known folder as unpackaged if that is intentional).
Note: resolveWin32Path still works under WinPackaged; only cross-platform mapping has this specific requirement.`);
      }

      default: {
         throw new Error(`Unsupported platform: ${environment}`);
      }
   }
}
function validateRelativePath(relativePath: string): void {
   if (relativePath.length === 0) {
      throw new Error('Relative path must not be empty');
   }

   if (relativePath.startsWith('/') || /^[a-zA-Z]:/.test(relativePath)) {
      throw new Error(`Relative path must contain only normal path segments: ${relativePath}`);
   }

   for (const segment of relativePath.split(/[/\\]/)) {
      if (segment === '' || segment === '.' || segment === '..') {
         throw new Error(`Relative path must contain only normal path segments: ${relativePath}`);
      }
   }
}

async function resolveWithRelativePath(resolveBasePath: () => Promise<string>, relativePath?: string): Promise<string> {
   const path = await resolveBasePath();

   if (relativePath !== undefined) {
      validateRelativePath(relativePath);
      return join(path, relativePath);
   }

   return path;
}
