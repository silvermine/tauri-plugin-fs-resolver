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
   resolveWin32Path,
} from './index';
import {
   AndroidPath,
   AndroidPathCollection,
   FsEnvironment,
   IosPath,
   LinuxPath,
   MacPath,
   Win32Path,
   WindowsApplicationDataPath,
} from './types';
import { clearFsEnvironmentCache, resolveWindowsApplicationDataPath } from './platform-paths';

const resolvedAndroidPath = 'android_path',
      resolvedAndroidPathCollection = [ 'android_path_1', 'android_path_2' ],
      resolvedIosPath = 'ios_path',
      resolvedLinuxPath = 'linux_path',
      resolvedMacPath = 'mac_path',
      resolvedWin32Path = 'win32_path',
      resolvedWindowsApplicationDataPath = 'windows_application_data_path';

let lastCmd = '',
    lastArgs: Record<string, unknown> = {},
    mockFsEnvironment: FsEnvironment | undefined;

function setMockFsEnvironment(environment: FsEnvironment): void {
   clearFsEnvironmentCache();
   mockFsEnvironment = environment;
}

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
      if (cmd === 'plugin:fs-resolver|resolve_win32_path') {
         return resolvedWin32Path;
      }
      if (cmd === 'plugin:fs-resolver|resolve_windows_application_data_path') {
         return resolvedWindowsApplicationDataPath;
      }
      if (cmd === 'plugin:fs-resolver|get_fs_environment') {
         return mockFsEnvironment;
      }

      return undefined;
   });
});

afterEach(() => { return clearMocks(); });

describe('fs-resolver actions map to Tauri commands', () => {
   it('resolveAndroidPath — sends path, returns resolved path', async () => {
      setMockFsEnvironment('android');
      const resolvedPath = await resolveAndroidPath(AndroidPath.DataDir);

      expect(lastCmd).toBe('plugin:fs-resolver|resolve_android_path');
      expect(lastArgs.path).toBe(AndroidPath.DataDir);
      expect(resolvedPath).toBe(resolvedAndroidPath);
   });

   it('resolveAndroidPathCollection — sends collection, returns resolved paths', async () => {
      setMockFsEnvironment('android');
      const resolvedPaths = await resolveAndroidPathCollection(AndroidPathCollection.ExternalCacheDirs);

      expect(lastCmd).toBe('plugin:fs-resolver|resolve_android_path_collection');
      expect(lastArgs.collection).toBe(AndroidPathCollection.ExternalCacheDirs);
      expect(resolvedPaths).toBe(resolvedAndroidPathCollection);
   });

   it('resolveIosPath — sends path, returns resolved path', async () => {
      setMockFsEnvironment('ios');
      const resolvedPath = await resolveIosPath(IosPath.CachesDirectory);

      expect(lastCmd).toBe('plugin:fs-resolver|resolve_ios_path');
      expect(lastArgs.path).toBe(IosPath.CachesDirectory);
      expect(resolvedPath).toBe(resolvedIosPath);
   });

   it('resolveLinuxPath — sends path, returns resolved path', async () => {
      setMockFsEnvironment('linux');
      const resolvedPath = await resolveLinuxPath(LinuxPath.DataHome);

      expect(lastCmd).toBe('plugin:fs-resolver|resolve_linux_path');
      expect(lastArgs.path).toBe(LinuxPath.DataHome);
      expect(resolvedPath).toBe(resolvedLinuxPath);
   });

   it('resolveLinuxPath — supports ForCurrentApp variants', async () => {
      setMockFsEnvironment('linux');
      const resolvedPath = await resolveLinuxPath(LinuxPath.DataHomeForCurrentApp);

      expect(lastCmd).toBe('plugin:fs-resolver|resolve_linux_path');
      expect(lastArgs.path).toBe(LinuxPath.DataHomeForCurrentApp);
      expect(resolvedPath).toBe(resolvedLinuxPath);
   });

   it('resolveMacPath — sends path, returns resolved path', async () => {
      setMockFsEnvironment('macos');
      const resolvedPath = await resolveMacPath(MacPath.CachesDirectory);

      expect(lastCmd).toBe('plugin:fs-resolver|resolve_mac_path');
      expect(lastArgs.path).toBe(MacPath.CachesDirectory);
      expect(resolvedPath).toBe(resolvedMacPath);
   });

   it('resolveMacPath — supports ForCurrentApp variants', async () => {
      setMockFsEnvironment('macos');
      const resolvedPath = await resolveMacPath(MacPath.ApplicationSupportDirectoryForCurrentApp);

      expect(lastCmd).toBe('plugin:fs-resolver|resolve_mac_path');
      expect(lastArgs.path).toBe(MacPath.ApplicationSupportDirectoryForCurrentApp);
      expect(resolvedPath).toBe(resolvedMacPath);
   });

   it('resolveWin32Path — sends Win32 path, returns resolved path', async () => {
      setMockFsEnvironment('win32');
      const resolvedPath = await resolveWin32Path(Win32Path.LocalAppData);

      expect(lastCmd).toBe('plugin:fs-resolver|resolve_win32_path');
      expect(lastArgs.path).toEqual(Win32Path.LocalAppData);
      expect(resolvedPath).toBe(resolvedWin32Path);
   });

   it('resolveWin32Path — supports Win32 ForCurrentApp variants', async () => {
      setMockFsEnvironment('win32');
      const resolvedPath = await resolveWin32Path(Win32Path.RoamingAppDataForCurrentApp);

      expect(lastCmd).toBe('plugin:fs-resolver|resolve_win32_path');
      expect(lastArgs.path).toEqual(Win32Path.RoamingAppDataForCurrentApp);
      expect(resolvedPath).toBe(resolvedWin32Path);
   });

   it('resolveWin32Path when in winpackaged environment — sends Win32 path, returns resolved path', async () => {
      setMockFsEnvironment('winpackaged');
      const resolvedPath = await resolveWin32Path(Win32Path.LocalAppData);

      expect(lastCmd).toBe('plugin:fs-resolver|resolve_win32_path');
      expect(lastArgs.path).toEqual(Win32Path.LocalAppData);
      expect(resolvedPath).toBe(resolvedWin32Path);
   });

   it('resolveWindowsApplicationDataPath — sends WindowsApplicationDataPath, returns resolved path', async () => {
      setMockFsEnvironment('winpackaged');
      const resolvedPath = await resolveWindowsApplicationDataPath(WindowsApplicationDataPath.LocalFolder);

      expect(lastCmd).toBe('plugin:fs-resolver|resolve_windows_application_data_path');
      expect(lastArgs.path).toEqual(WindowsApplicationDataPath.LocalFolder);
      expect(resolvedPath).toBe(resolvedWindowsApplicationDataPath);
   });
});

describe('fs-resolver errors when calling from incorrect platform', () => {
   const androidErrorMsg = 'This function is only available on android',
         iosErrorMsg = 'This function is only available on ios',
         linuxErrorMsg = 'This function is only available on linux',
         macosErrorMsg = 'This function is only available on macos',
         win32ErrorMsg = 'This function is only available on win32 or winpackaged',
         winpackagedErrorMsg = 'This function is only available on winpackaged';

   it('resolveAndroidPath — throws error if not on Android', async () => {
      setMockFsEnvironment('ios');
      await expect(resolveAndroidPath(AndroidPath.DataDir)).rejects.toThrow(androidErrorMsg);

      setMockFsEnvironment('linux');
      await expect(resolveAndroidPath(AndroidPath.DataDir)).rejects.toThrow(androidErrorMsg);

      setMockFsEnvironment('macos');
      await expect(resolveAndroidPath(AndroidPath.DataDir)).rejects.toThrow(androidErrorMsg);

      setMockFsEnvironment('win32');
      await expect(resolveAndroidPath(AndroidPath.DataDir)).rejects.toThrow(androidErrorMsg);

      setMockFsEnvironment('winpackaged');
      await expect(resolveAndroidPath(AndroidPath.DataDir)).rejects.toThrow(androidErrorMsg);
   });

   it('resolveAndroidPathCollection — throws error if not on Android', async () => {
      setMockFsEnvironment('ios');
      await expect(resolveAndroidPathCollection(AndroidPathCollection.ExternalCacheDirs)).rejects.toThrow(androidErrorMsg);

      setMockFsEnvironment('linux');
      await expect(resolveAndroidPathCollection(AndroidPathCollection.ExternalCacheDirs)).rejects.toThrow(androidErrorMsg);

      setMockFsEnvironment('macos');
      await expect(resolveAndroidPathCollection(AndroidPathCollection.ExternalCacheDirs)).rejects.toThrow(androidErrorMsg);

      setMockFsEnvironment('win32');
      await expect(resolveAndroidPathCollection(AndroidPathCollection.ExternalCacheDirs)).rejects.toThrow(androidErrorMsg);

      setMockFsEnvironment('winpackaged');
      await expect(resolveAndroidPathCollection(AndroidPathCollection.ExternalCacheDirs)).rejects.toThrow(androidErrorMsg);
   });

   it('resolveIosPath — throws error if not on iOS', async () => {
      setMockFsEnvironment('android');
      await expect(resolveIosPath(IosPath.CachesDirectory)).rejects.toThrow(iosErrorMsg);

      setMockFsEnvironment('linux');
      await expect(resolveIosPath(IosPath.CachesDirectory)).rejects.toThrow(iosErrorMsg);

      setMockFsEnvironment('macos');
      await expect(resolveIosPath(IosPath.CachesDirectory)).rejects.toThrow(iosErrorMsg);

      setMockFsEnvironment('win32');
      await expect(resolveIosPath(IosPath.CachesDirectory)).rejects.toThrow(iosErrorMsg);

      setMockFsEnvironment('winpackaged');
      await expect(resolveIosPath(IosPath.CachesDirectory)).rejects.toThrow(iosErrorMsg);
   });

   it('resolveLinuxPath — throws error if not on Linux', async () => {
      setMockFsEnvironment('android');
      await expect(resolveLinuxPath(LinuxPath.DataHome)).rejects.toThrow(linuxErrorMsg);

      setMockFsEnvironment('ios');
      await expect(resolveLinuxPath(LinuxPath.DataHome)).rejects.toThrow(linuxErrorMsg);

      setMockFsEnvironment('macos');
      await expect(resolveLinuxPath(LinuxPath.DataHome)).rejects.toThrow(linuxErrorMsg);

      setMockFsEnvironment('win32');
      await expect(resolveLinuxPath(LinuxPath.DataHome)).rejects.toThrow(linuxErrorMsg);

      setMockFsEnvironment('winpackaged');
      await expect(resolveLinuxPath(LinuxPath.DataHome)).rejects.toThrow(linuxErrorMsg);
   });

   it('resolveMacPath — throws error if not on macOS', async () => {
      setMockFsEnvironment('android');
      await expect(resolveMacPath(MacPath.CachesDirectory)).rejects.toThrow(macosErrorMsg);

      setMockFsEnvironment('ios');
      await expect(resolveMacPath(MacPath.CachesDirectory)).rejects.toThrow(macosErrorMsg);

      setMockFsEnvironment('linux');
      await expect(resolveMacPath(MacPath.CachesDirectory)).rejects.toThrow(macosErrorMsg);

      setMockFsEnvironment('win32');
      await expect(resolveMacPath(MacPath.CachesDirectory)).rejects.toThrow(macosErrorMsg);

      setMockFsEnvironment('winpackaged');
      await expect(resolveMacPath(MacPath.CachesDirectory)).rejects.toThrow(macosErrorMsg);
   });

   it('resolveWin32Path — throws error if not on Win32 or WinPackaged', async () => {
      setMockFsEnvironment('android');
      await expect(resolveWin32Path(Win32Path.LocalAppData)).rejects.toThrow(win32ErrorMsg);

      setMockFsEnvironment('ios');
      await expect(resolveWin32Path(Win32Path.LocalAppData)).rejects.toThrow(win32ErrorMsg);

      setMockFsEnvironment('linux');
      await expect(resolveWin32Path(Win32Path.LocalAppData)).rejects.toThrow(win32ErrorMsg);

      setMockFsEnvironment('macos');
      await expect(resolveWin32Path(Win32Path.LocalAppData)).rejects.toThrow(win32ErrorMsg);
   });

   it('resolveWindowsApplicationDataPath — throws error if not on WinPackaged', async () => {
      setMockFsEnvironment('android');
      await expect(resolveWindowsApplicationDataPath(WindowsApplicationDataPath.LocalFolder)).rejects.toThrow(winpackagedErrorMsg);

      setMockFsEnvironment('ios');
      await expect(resolveWindowsApplicationDataPath(WindowsApplicationDataPath.LocalFolder)).rejects.toThrow(winpackagedErrorMsg);

      setMockFsEnvironment('linux');
      await expect(resolveWindowsApplicationDataPath(WindowsApplicationDataPath.LocalFolder)).rejects.toThrow(winpackagedErrorMsg);

      setMockFsEnvironment('macos');
      await expect(resolveWindowsApplicationDataPath(WindowsApplicationDataPath.LocalFolder)).rejects.toThrow(winpackagedErrorMsg);

      setMockFsEnvironment('win32');
      await expect(resolveWindowsApplicationDataPath(WindowsApplicationDataPath.LocalFolder)).rejects.toThrow(winpackagedErrorMsg);
   });
});
