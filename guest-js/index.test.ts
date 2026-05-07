/**
 * Sanity check to test the bridge between Typescript and the Tauri commands.
 */
import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { mockIPC, clearMocks } from '@tauri-apps/api/mocks';
import {
   resolveAndroidPath,
   resolveAndroidPathCollection,
   resolveIosPath,
   resolveLinuxPath,
   resolveMacPath,
   resolveWindowsPath,
} from './index';
import { AndroidPath, AndroidPathCollection, IosPath, LinuxPath, MacPath, Win32Path, WindowsApplicationDataPath } from './types';

const resolvedAndroidPath = 'android_path',
      resolvedAndroidPathCollection = [ 'android_path_1', 'android_path_2' ],
      resolvedIosPath = 'ios_path',
      resolvedLinuxPath = 'linux_path',
      resolvedMacPath = 'mac_path',
      resolvedWin32Path = 'win32_path',
      resolvedWindowsApplicationDataPath = 'windows_application_data_path';

let lastCmd = '',
    lastArgs: Record<string, unknown> = {},
    mockPlatform: '' | 'android' | 'ios' | 'linux' | 'macos' | 'windows' = '';

vi.mock('@tauri-apps/plugin-os', () => {
   return {
      platform: () => { return mockPlatform; },
   };
});

beforeEach(() => {
   mockIPC((cmd, args) => {
      lastCmd = cmd;
      lastArgs = args as Record<string, unknown>;

      if (cmd === 'plugin:fs-resolver|resolve_android_path') {
         return resolvedAndroidPath;
      }
      if (cmd === 'plugin:fs-resolver|resolve_android_path_collection') {
         return resolvedAndroidPathCollection;
      }
      if (cmd === 'plugin:fs-resolver|resolve_ios_path') {
         return resolvedIosPath;
      }
      if (cmd === 'plugin:fs-resolver|resolve_linux_path') {
         return resolvedLinuxPath;
      }
      if (cmd === 'plugin:fs-resolver|resolve_mac_path') {
         return resolvedMacPath;
      }
      if (cmd === 'plugin:fs-resolver|resolve_windows_path') {
         const pathObj = lastArgs.path as Record<string, unknown>;

         if ('win32' in pathObj) {
            return resolvedWin32Path;
         }
         if ('winMsix' in pathObj) {
            return resolvedWindowsApplicationDataPath;
         }
         return undefined;
      }
      return undefined;
   });
});

afterEach(() => { return clearMocks(); });

describe('fs-resolver actions map to Tauri commands', () => {
   it('resolveAndroidPath — sends path, returns resolved path', async () => {
      mockPlatform = 'android';
      const resolvedPath = await resolveAndroidPath(AndroidPath.DataDir);

      expect(lastCmd).toBe('plugin:fs-resolver|resolve_android_path');
      expect(lastArgs.path).toBe(AndroidPath.DataDir);
      expect(resolvedPath).toBe(resolvedAndroidPath);
   });

   it('resolveAndroidPathCollection — sends collection, returns resolved paths', async () => {
      mockPlatform = 'android';
      const resolvedPaths = await resolveAndroidPathCollection(AndroidPathCollection.ExternalCacheDirs);

      expect(lastCmd).toBe('plugin:fs-resolver|resolve_android_path_collection');
      expect(lastArgs.collection).toBe(AndroidPathCollection.ExternalCacheDirs);
      expect(resolvedPaths).toBe(resolvedAndroidPathCollection);
   });

   it('resolveIosPath — sends path, returns resolved path', async () => {
      mockPlatform = 'ios';
      const resolvedPath = await resolveIosPath(IosPath.CachesDirectory);

      expect(lastCmd).toBe('plugin:fs-resolver|resolve_ios_path');
      expect(lastArgs.path).toBe(IosPath.CachesDirectory);
      expect(resolvedPath).toBe(resolvedIosPath);
   });

   it('resolveLinuxPath — sends path, returns resolved path', async () => {
      mockPlatform = 'linux';
      const resolvedPath = await resolveLinuxPath(LinuxPath.UserHomeDirectory);

      expect(lastCmd).toBe('plugin:fs-resolver|resolve_linux_path');
      expect(lastArgs.path).toBe(LinuxPath.UserHomeDirectory);
      expect(resolvedPath).toBe(resolvedLinuxPath);
   });

   it('resolveMacPath — sends path, returns resolved path', async () => {
      mockPlatform = 'macos';
      const resolvedPath = await resolveMacPath(MacPath.CachesDirectory);

      expect(lastCmd).toBe('plugin:fs-resolver|resolve_mac_path');
      expect(lastArgs.path).toBe(MacPath.CachesDirectory);
      expect(resolvedPath).toBe(resolvedMacPath);
   });

   it('resolveWindowsPath — sends Win32 path, returns resolved path', async () => {
      mockPlatform = 'windows';
      const resolvedPath = await resolveWindowsPath({ win32: Win32Path.LocalAppData });

      expect(lastCmd).toBe('plugin:fs-resolver|resolve_windows_path');
      expect(lastArgs.path).toEqual({ win32: Win32Path.LocalAppData });
      expect(resolvedPath).toBe(resolvedWin32Path);
   });

   it('resolveWindowsPath — sends WindowsApplicationDataPath path, returns resolved path', async () => {
      mockPlatform = 'windows';
      const resolvedPath = await resolveWindowsPath({ winMsix: WindowsApplicationDataPath.LocalFolder });

      expect(lastCmd).toBe('plugin:fs-resolver|resolve_windows_path');
      expect(lastArgs.path).toEqual({ winMsix: WindowsApplicationDataPath.LocalFolder });
      expect(resolvedPath).toBe(resolvedWindowsApplicationDataPath);
   });
});

describe('fs-resolver errors when calling from incorrect platform', () => {
   const androidErrorMsg = 'This function is only available on Android',
         iosErrorMsg = 'This function is only available on iOS',
         linuxErrorMsg = 'This function is only available on Linux',
         macosErrorMsg = 'This function is only available on macOS',
         windowsErrorMsg = 'This function is only available on Windows';

   it('resolveAndroidPath — throws error if not on Android', async () => {
      mockPlatform = 'ios';
      await expect(resolveAndroidPath(AndroidPath.DataDir)).rejects.toThrow(androidErrorMsg);

      mockPlatform = 'linux';
      await expect(resolveAndroidPath(AndroidPath.DataDir)).rejects.toThrow(androidErrorMsg);

      mockPlatform = 'macos';
      await expect(resolveAndroidPath(AndroidPath.DataDir)).rejects.toThrow(androidErrorMsg);

      mockPlatform = 'windows';
      await expect(resolveAndroidPath(AndroidPath.DataDir)).rejects.toThrow(androidErrorMsg);
   });

   it('resolveAndroidPathCollection — throws error if not on Android', async () => {
      mockPlatform = 'ios';
      await expect(resolveAndroidPathCollection(AndroidPathCollection.ExternalCacheDirs)).rejects.toThrow(androidErrorMsg);

      mockPlatform = 'linux';
      await expect(resolveAndroidPathCollection(AndroidPathCollection.ExternalCacheDirs)).rejects.toThrow(androidErrorMsg);

      mockPlatform = 'macos';
      await expect(resolveAndroidPathCollection(AndroidPathCollection.ExternalCacheDirs)).rejects.toThrow(androidErrorMsg);

      mockPlatform = 'windows';
      await expect(resolveAndroidPathCollection(AndroidPathCollection.ExternalCacheDirs)).rejects.toThrow(androidErrorMsg);
   });

   it('resolveIosPath — throws error if not on iOS', async () => {
      mockPlatform = 'android';
      await expect(resolveIosPath(IosPath.CachesDirectory)).rejects.toThrow(iosErrorMsg);

      mockPlatform = 'linux';
      await expect(resolveIosPath(IosPath.CachesDirectory)).rejects.toThrow(iosErrorMsg);

      mockPlatform = 'macos';
      await expect(resolveIosPath(IosPath.CachesDirectory)).rejects.toThrow(iosErrorMsg);

      mockPlatform = 'windows';
      await expect(resolveIosPath(IosPath.CachesDirectory)).rejects.toThrow(iosErrorMsg);
   });

   it('resolveLinuxPath — throws error if not on Linux', async () => {
      mockPlatform = 'android';
      await expect(resolveLinuxPath(LinuxPath.UserHomeDirectory)).rejects.toThrow(linuxErrorMsg);

      mockPlatform = 'ios';
      await expect(resolveLinuxPath(LinuxPath.UserHomeDirectory)).rejects.toThrow(linuxErrorMsg);

      mockPlatform = 'macos';
      await expect(resolveLinuxPath(LinuxPath.UserHomeDirectory)).rejects.toThrow(linuxErrorMsg);

      mockPlatform = 'windows';
      await expect(resolveLinuxPath(LinuxPath.UserHomeDirectory)).rejects.toThrow(linuxErrorMsg);
   });

   it('resolveMacPath — throws error if not on macOS', async () => {
      mockPlatform = 'android';
      await expect(resolveMacPath(MacPath.CachesDirectory)).rejects.toThrow(macosErrorMsg);

      mockPlatform = 'ios';
      await expect(resolveMacPath(MacPath.CachesDirectory)).rejects.toThrow(macosErrorMsg);

      mockPlatform = 'linux';
      await expect(resolveMacPath(MacPath.CachesDirectory)).rejects.toThrow(macosErrorMsg);

      mockPlatform = 'windows';
      await expect(resolveMacPath(MacPath.CachesDirectory)).rejects.toThrow(macosErrorMsg);
   });

   it('resolveWindowsPath — throws error if not on Windows', async () => {
      mockPlatform = 'android';
      await expect(resolveWindowsPath({ win32: Win32Path.LocalAppData })).rejects.toThrow(windowsErrorMsg);

      mockPlatform = 'ios';
      await expect(resolveWindowsPath({ win32: Win32Path.LocalAppData })).rejects.toThrow(windowsErrorMsg);

      mockPlatform = 'linux';
      await expect(resolveWindowsPath({ win32: Win32Path.LocalAppData })).rejects.toThrow(windowsErrorMsg);

      mockPlatform = 'macos';
      await expect(resolveWindowsPath({ win32: Win32Path.LocalAppData })).rejects.toThrow(windowsErrorMsg);
   });
});
