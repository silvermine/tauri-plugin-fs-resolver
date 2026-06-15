import { platform } from '@tauri-apps/plugin-os';
import { resolveAndroidPath, resolveIosPath, resolveLinuxPath, resolveMacPath, resolveWindowsPath } from './platform-paths';
import { AndroidPath, IosPath, LinuxPath, MacPath, WindowsPath } from './types';
import { join } from '@tauri-apps/api/path';

/**
 * Per-platform path mapping: a platform path enum plus an optional relative suffix.
 *
 * These types are resolved in-process on the frontend. They are not sent across IPC;
 * only individual path enums cross the boundary via per-platform resolve commands.
 */
export type PlatformMapping<T extends AndroidPath | IosPath | LinuxPath | MacPath | WindowsPath> = {
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
   windows?: PlatformMapping<WindowsPath>;
};

/**
 * Resolves a cross-platform mapping for the current OS.
 *
 * Picks the mapping entry for the current platform, invokes the corresponding
 * per-platform resolve helper (IPC), then optionally joins `relativePath`.
 */
export async function resolveMapping(mapping: CrossPlatformMapping): Promise<string> {
   const os = platform();

   switch (os) {
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

      case 'windows': {
         const windows = mapping.windows;

         if (windows) {
            return await resolveWithRelativePath(
               () => { return resolveWindowsPath(windows.platformPath); },
               windows.relativePath
            );
         }

         throw new Error('No path defined for Windows');
      }

      default: {
         throw new Error(`Unsupported platform: ${os}`);
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
