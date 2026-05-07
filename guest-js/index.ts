import { invoke } from '@tauri-apps/api/core';
import { platform } from '@tauri-apps/plugin-os';
import { AndroidPath, AndroidPathCollection, MacPath, IosPath, WindowsPath, LinuxPath } from './types';

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
   if (platform() !== 'android') {
      throw new Error('This function is only available on Android');
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
   if (platform() !== 'android') {
      throw new Error('This function is only available on Android');
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
   if (platform() !== 'ios') {
      throw new Error('This function is only available on iOS');
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
   if (platform() !== 'linux') {
      throw new Error('This function is only available on Linux');
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
   if (platform() !== 'macos') {
      throw new Error('This function is only available on macOS');
   }

   return await invoke<string>('plugin:fs-resolver|resolve_mac_path', { path });
}

/**
 * Resolves a Win32 path on the Windows platform.
 *
 * If this function is called on a platform other than Windows, it will throw an
 * error.
 *
 * @param path - The path to resolve.
 * @returns The resolved path.
 */
export async function resolveWindowsPath(path: WindowsPath): Promise<string> {
   if (platform() !== 'windows') {
      throw new Error('This function is only available on Windows');
   }

   return await invoke<string>('plugin:fs-resolver|resolve_windows_path', { path });
}
