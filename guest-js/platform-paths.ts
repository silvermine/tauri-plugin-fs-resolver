import { invoke } from '@tauri-apps/api/core';
import {
   AndroidPath,
   AndroidPathCollection,
   MacPath,
   IosPath,
   LinuxPath,
   Win32Path,
   WindowsApplicationDataPath,
   FsEnvironment,
} from './types';

let fsEnvironment: FsEnvironment | undefined;

/**
 * Resolves a path on the Android platform.
 *
 * If this function is called on a platform other than Android, it will throw an
 * error.
 *
 * @param path - The path to resolve.
 * @returns The resolved path.
 */
export async function resolveAndroidPath(path: AndroidPath): Promise<string> {
   if (await getFsEnvironment() !== 'android') {
      throw new Error('This function is only available on android');
   }

   return await invoke<string>('plugin:fs-resolver|resolve_android_path', { path });
}

/**
 * Resolves a collection of paths on the Android platform.
 *
 * If this function is called on a platform other than Android, it will throw an
 * error.
 *
 * @param collection - The collection of paths to resolve.
 * @returns The resolved paths.
 */

export async function resolveAndroidPathCollection(collection: AndroidPathCollection): Promise<string[]> {
   if (await getFsEnvironment() !== 'android') {
      throw new Error('This function is only available on android');
   }

   return await invoke<string[]>('plugin:fs-resolver|resolve_android_path_collection', { collection });
}

/**
 * Resolves a path on the iOS platform.
 *
 * If this function is called on a platform other than iOS, it will throw an
 * error.
 *
 * @param path - The path to resolve.
 * @returns The resolved path.
 */
export async function resolveIosPath(path: IosPath): Promise<string> {
   if (await getFsEnvironment() !== 'ios') {
      throw new Error('This function is only available on ios');
   }

   return await invoke<string>('plugin:fs-resolver|resolve_ios_path', { path });
}

/**
 * Resolves a path on the Linux platform.
 *
 * If this function is called on a platform other than Linux, it will throw an
 * error.
 *
 * @param path - The path to resolve.
 * @returns The resolved path.
 */
export async function resolveLinuxPath(path: LinuxPath): Promise<string> {
   if (await getFsEnvironment() !== 'linux') {
      throw new Error('This function is only available on linux');
   }

   return await invoke<string>('plugin:fs-resolver|resolve_linux_path', { path });
}

/**
 * Resolves a path on the macOS platform.
 *
 * If this function is called on a platform other than macOS, it will throw an
 * error.
 *
 * @param path - The path to resolve.
 * @returns The resolved path.
 */
export async function resolveMacPath(path: MacPath): Promise<string> {
   if (await getFsEnvironment() !== 'macos') {
      throw new Error('This function is only available on macos');
   }

   return await invoke<string>('plugin:fs-resolver|resolve_mac_path', { path });
}

/**
 * Resolves a Win32 path on the Windows platform.
 *
 * If this function is called on a platform other than Windows (WinPackaged or Win32),
 * it will throw an error.
 *
 * @param path - The path to resolve.
 * @returns The resolved path.
 */
export async function resolveWin32Path(path: Win32Path): Promise<string> {
   const environment = await getFsEnvironment();

   if (environment !== 'win32' && environment !== 'winpackaged') {
      throw new Error('This function is only available on win32 or winpackaged');
   }

   return await invoke<string>('plugin:fs-resolver|resolve_win32_path', { path });
}

/**
 * Resolves a path on the Windows platform.
 *
 * If this function is called on a platform other than WinPackaged, it will throw an
 * error.
 *
 * @param path - The path to resolve.
 * @returns The resolved path.
 */
export async function resolveWindowsApplicationDataPath(path: WindowsApplicationDataPath): Promise<string> {
   if (await getFsEnvironment() !== 'winpackaged') {
      throw new Error('This function is only available on winpackaged');
   }

   return await invoke<string>('plugin:fs-resolver|resolve_windows_application_data_path', { path });
}

/**
 * Returns the file system environment.
 *
 * @returns The file system environment.
 */
export async function getFsEnvironment(): Promise<FsEnvironment> {
   // This is cached, as it will not change during the lifetime of the application.
   if (fsEnvironment) {
      return fsEnvironment;
   }

   fsEnvironment = await invoke<FsEnvironment>('plugin:fs-resolver|get_fs_environment');

   return fsEnvironment;
}

/**
 * Clears the file system environment cache.
 *
 * This is only meant to be used for tests.
 */
export function clearFsEnvironmentCache(): void {
   fsEnvironment = undefined;
}
