import { describe, it, expect, afterEach } from 'vitest';
import { clearMocks } from '@tauri-apps/api/mocks';

import { AndroidPath, FsEnvironment, IosPath, LinuxPath, MacPath, Win32Path, WindowsApplicationDataPath } from './types';
import { CrossPlatformMapping, resolveMapping } from './path-mapping';

const resolvedAndroidPath = 'android_path',
      resolvedIosPath = 'ios_path',
      resolvedLinuxPath = 'linux_path',
      resolvedMacPath = 'mac_path',
      resolvedWin32Path = 'win32_path',
      resolvedWindowsApplicationDataPath = 'windows_application_data_path';

const winPackagedMissingMappingErrorMessage = `winPackaged mapping is missing while running as WinPackaged.
resolveMapping does not fall back to win32. Set winPackaged explicitly:
{ kind: 'windowsApplicationDataPath', mapping: { ... } } or
{ kind: 'win32', mapping: { ... } } (same known folder as unpackaged if that is intentional).
Note: resolveWin32Path still works under WinPackaged; only cross-platform mapping has this specific requirement.`;

let mockFsEnvironment: FsEnvironment | undefined;

vi.mock('@tauri-apps/api/path', () => {
   return {
      join: (path: string, relativePath: string) => { return `${path}/${relativePath}`; },
   };
});

vi.mock('./platform-paths', () => {
   return {
      resolveAndroidPath: () => { return resolvedAndroidPath; },
      resolveIosPath: () => { return resolvedIosPath; },
      resolveLinuxPath: () => { return resolvedLinuxPath; },
      resolveMacPath: () => { return resolvedMacPath; },
      resolveWin32Path: () => { return resolvedWin32Path; },
      resolveWindowsApplicationDataPath: () => { return resolvedWindowsApplicationDataPath; },
      getFsEnvironment: () => { return mockFsEnvironment; },
   };
});

afterEach(() => { return clearMocks(); });

describe('fs-resolver actions map to Tauri commands', () => {
   it('resolveMapping — sends path, returns resolved path', async () => {

      const pathMapping: CrossPlatformMapping = {
         android: { platformPath: AndroidPath.DataDir },
         ios: { platformPath: IosPath.CachesDirectory },
         linux: { platformPath: LinuxPath.DataHomeForCurrentApp },
         macos: { platformPath: MacPath.ApplicationSupportDirectoryForCurrentApp },
         win32: { platformPath: Win32Path.RoamingAppDataForCurrentApp },
         winPackaged: { kind: 'windowsApplicationDataPath', mapping: { platformPath: WindowsApplicationDataPath.LocalFolder } },
      };

      mockFsEnvironment = 'android';
      expect(await resolveMapping(pathMapping)).toBe(resolvedAndroidPath);

      mockFsEnvironment = 'ios';
      expect(await resolveMapping(pathMapping)).toBe(resolvedIosPath);

      mockFsEnvironment = 'linux';
      expect(await resolveMapping(pathMapping)).toBe(resolvedLinuxPath);

      mockFsEnvironment = 'macos';
      expect(await resolveMapping(pathMapping)).toBe(resolvedMacPath);

      mockFsEnvironment = 'win32';
      expect(await resolveMapping(pathMapping)).toBe(resolvedWin32Path);

      mockFsEnvironment = 'winpackaged';
      expect(await resolveMapping(pathMapping)).toBe(resolvedWindowsApplicationDataPath);
   });


   it('resolveMapping — sends path, returns resolved path with relative path', async () => {

      const pathMapping: CrossPlatformMapping = {
         android: { platformPath: AndroidPath.DataDir, relativePath: 'android_relative_path' },
         ios: { platformPath: IosPath.CachesDirectory, relativePath: 'ios_relative_path' },
         linux: { platformPath: LinuxPath.DataHome, relativePath: 'linux_relative_path' },
         macos: { platformPath: MacPath.CachesDirectory, relativePath: 'mac_relative_path' },
         win32: { platformPath: Win32Path.LocalAppData, relativePath: 'win32_relative_path' },
         winPackaged: {
            kind: 'windowsApplicationDataPath',
            mapping: {
               platformPath: WindowsApplicationDataPath.LocalFolder,
               relativePath: 'windows_application_data_relative_path',
            },
         },
      };

      mockFsEnvironment = 'android';
      expect(await resolveMapping(pathMapping)).toBe(`${resolvedAndroidPath}/android_relative_path`);

      mockFsEnvironment = 'ios';
      expect(await resolveMapping(pathMapping)).toBe(`${resolvedIosPath}/ios_relative_path`);

      mockFsEnvironment = 'linux';
      expect(await resolveMapping(pathMapping)).toBe(`${resolvedLinuxPath}/linux_relative_path`);

      mockFsEnvironment = 'macos';
      expect(await resolveMapping(pathMapping)).toBe(`${resolvedMacPath}/mac_relative_path`);

      mockFsEnvironment = 'win32';
      expect(await resolveMapping(pathMapping)).toBe(`${resolvedWin32Path}/win32_relative_path`);

      mockFsEnvironment = 'winpackaged';
      expect(await resolveMapping(pathMapping)).toBe(`${resolvedWindowsApplicationDataPath}/windows_application_data_relative_path`);
   });

   it('resolveMapping — rejects invalid relative paths before joining', async () => {
      const baseMapping: CrossPlatformMapping = {
         android: { platformPath: AndroidPath.DataDir },
      };

      mockFsEnvironment = 'android';

      for (const relativePath of [ '', '.', '..', '/absolute', 'foo/..', 'foo/.', 'foo//bar' ]) {
         await expect(
            resolveMapping({
               ...baseMapping,
               android: { platformPath: AndroidPath.DataDir, relativePath },
            }))
            .rejects
            .toThrow(/Relative path must/);
      }
   });

   it('resolveMapping - throws mappings defined with invalid relative paths', async () => {

      const pathMapping: CrossPlatformMapping = {
         android: { platformPath: AndroidPath.DataDir, relativePath: '../escape' },
         ios: { platformPath: IosPath.CachesDirectory, relativePath: '../escape' },
         linux: { platformPath: LinuxPath.DataHome, relativePath: '../escape' },
         macos: { platformPath: MacPath.CachesDirectory, relativePath: '../escape' },
         win32: { platformPath: Win32Path.LocalAppData, relativePath: '../escape' },
         winPackaged: {
            kind: 'windowsApplicationDataPath',
            mapping: {
               platformPath: WindowsApplicationDataPath.LocalFolder,
               relativePath: '../escape',
            },
         },
      };

      mockFsEnvironment = 'android';
      await expect(resolveMapping(pathMapping)).rejects.toThrow(/Relative path must/);

      mockFsEnvironment = 'ios';
      await expect(resolveMapping(pathMapping)).rejects.toThrow(/Relative path must/);

      mockFsEnvironment = 'linux';
      await expect(resolveMapping(pathMapping)).rejects.toThrow(/Relative path must/);

      mockFsEnvironment = 'macos';
      await expect(resolveMapping(pathMapping)).rejects.toThrow(/Relative path must/);

      mockFsEnvironment = 'win32';
      await expect(resolveMapping(pathMapping)).rejects.toThrow(/Relative path must/);

      mockFsEnvironment = 'winpackaged';
      await expect(resolveMapping(pathMapping)).rejects.toThrow(/Relative path must/);
   });

   it('resolveMapping - resolve mapping for winpackaged environment does not fall back to win32', async () => {
      const pathMapping: CrossPlatformMapping = {
         win32: { platformPath: Win32Path.LocalAppData },
      };

      mockFsEnvironment = 'winpackaged';
      await expect(resolveMapping(pathMapping)).rejects.toThrow(winPackagedMissingMappingErrorMessage);
   });

   it('resolveMapping - resolve mapping for win32 environment does not fall back to winpackaged', async () => {
      const pathMapping: CrossPlatformMapping = {
         winPackaged: { kind: 'windowsApplicationDataPath', mapping: { platformPath: WindowsApplicationDataPath.LocalFolder } },
      };

      mockFsEnvironment = 'win32';
      await expect(resolveMapping(pathMapping)).rejects.toThrow('No path defined for Win32');
   });

   it('resolve-mapping: winPackaged supports win32 paths when defined', async () => {
      const pathMapping: CrossPlatformMapping = {
         winPackaged: { kind: 'win32', mapping: { platformPath: Win32Path.LocalAppData } },
      };

      mockFsEnvironment = 'winpackaged';
      expect(await resolveMapping(pathMapping)).toBe(resolvedWin32Path);
   });

   it('throws when mapping not present for current environment', async () => {
      const pathMapping: CrossPlatformMapping = {};

      mockFsEnvironment = 'android';
      await expect(resolveMapping(pathMapping)).rejects.toThrow('No path defined for Android');

      mockFsEnvironment = 'ios';
      await expect(resolveMapping(pathMapping)).rejects.toThrow('No path defined for iOS');

      mockFsEnvironment = 'linux';
      await expect(resolveMapping(pathMapping)).rejects.toThrow('No path defined for Linux');

      mockFsEnvironment = 'macos';
      await expect(resolveMapping(pathMapping)).rejects.toThrow('No path defined for macOS');

      mockFsEnvironment = 'win32';
      await expect(resolveMapping(pathMapping)).rejects.toThrow('No path defined for Win32');

      mockFsEnvironment = 'winpackaged';
      await expect(resolveMapping(pathMapping)).rejects.toThrow(winPackagedMissingMappingErrorMessage);
   });
});
