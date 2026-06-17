import { describe, it, expect, afterEach } from 'vitest';
import { clearMocks } from '@tauri-apps/api/mocks';

import { AndroidPath, IosPath, LinuxPath, MacPath, Win32Path } from './types';
import { CrossPlatformMapping, resolveMapping } from './path-mapping';

const resolvedAndroidPath = 'android_path',
      resolvedIosPath = 'ios_path',
      resolvedLinuxPath = 'linux_path',
      resolvedMacPath = 'mac_path',
      resolvedWindowsPath = 'windows_path';

let mockPlatform: '' | 'android' | 'ios' | 'linux' | 'macos' | 'windows' = '';

vi.mock('@tauri-apps/plugin-os', () => {
   return {
      platform: () => { return mockPlatform; },
   };
});

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
      resolveWindowsPath: () => { return resolvedWindowsPath; },
   };
});

afterEach(() => { return clearMocks(); });

describe('fs-resolver actions map to Tauri commands', () => {
   it('resolveMapping — sends path, returns resolved path', async () => {

      const pathMapping: CrossPlatformMapping = {
         android: { platform_path: AndroidPath.DataDir },
         ios: { platform_path: IosPath.CachesDirectory },
         linux: { platform_path: LinuxPath.DataHomeForCurrentApp },
         macos: { platform_path: MacPath.ApplicationSupportDirectoryForCurrentApp },
         windows: { platform_path: { win32: Win32Path.RoamingAppDataForCurrentApp } },
      };

      mockPlatform = 'android';
      expect(await resolveMapping(pathMapping)).toBe(resolvedAndroidPath);

      mockPlatform = 'ios';
      expect(await resolveMapping(pathMapping)).toBe(resolvedIosPath);

      mockPlatform = 'linux';
      expect(await resolveMapping(pathMapping)).toBe(resolvedLinuxPath);

      mockPlatform = 'macos';
      expect(await resolveMapping(pathMapping)).toBe(resolvedMacPath);

      mockPlatform = 'windows';
      expect(await resolveMapping(pathMapping)).toBe(resolvedWindowsPath);
   });


   it('resolveMapping — sends path, returns resolved path with relative path', async () => {

      const pathMapping: CrossPlatformMapping = {
         android: { platform_path: AndroidPath.DataDir, relativePath: 'android_relative_path' },
         ios: { platform_path: IosPath.CachesDirectory, relativePath: 'ios_relative_path' },
         linux: { platform_path: LinuxPath.DataHome, relativePath: 'linux_relative_path' },
         macos: { platform_path: MacPath.CachesDirectory, relativePath: 'mac_relative_path' },
         windows: { platform_path: { win32: Win32Path.LocalAppData }, relativePath: 'windows_relative_path' },
      };

      mockPlatform = 'android';
      expect(await resolveMapping(pathMapping)).toBe(`${resolvedAndroidPath}/android_relative_path`);

      mockPlatform = 'ios';
      expect(await resolveMapping(pathMapping)).toBe(`${resolvedIosPath}/ios_relative_path`);

      mockPlatform = 'linux';
      expect(await resolveMapping(pathMapping)).toBe(`${resolvedLinuxPath}/linux_relative_path`);

      mockPlatform = 'macos';
      expect(await resolveMapping(pathMapping)).toBe(`${resolvedMacPath}/mac_relative_path`);

      mockPlatform = 'windows';
      expect(await resolveMapping(pathMapping)).toBe(`${resolvedWindowsPath}/windows_relative_path`);
   });

   it('resolveMapping — rejects invalid relative paths before joining', async () => {
      const baseMapping: CrossPlatformMapping = {
         android: { platform_path: AndroidPath.DataDir },
      };

      mockPlatform = 'android';

      for (const relativePath of [ '', '.', '..', '/absolute', 'foo/..', 'foo/.', 'foo//bar' ]) {
         await expect(
            resolveMapping({
               ...baseMapping,
               android: { platform_path: AndroidPath.DataDir, relativePath },
            }))
            .rejects
            .toThrow(/Relative path must/);
      }
   });

   it('resolveMapping - throws mappings defined with invalid relative paths', async () => {

      const pathMapping: CrossPlatformMapping = {
         android: { platform_path: AndroidPath.DataDir, relativePath: '../escape' },
         ios: { platform_path: IosPath.CachesDirectory, relativePath: '../escape' },
         linux: { platform_path: LinuxPath.DataHome, relativePath: '../escape' },
         macos: { platform_path: MacPath.CachesDirectory, relativePath: '../escape' },
         windows: { platform_path: { win32: Win32Path.LocalAppData }, relativePath: '../escape' },
      };

      mockPlatform = 'android';
      await expect(resolveMapping(pathMapping)).rejects.toThrow(/Relative path must/);

      mockPlatform = 'ios';
      await expect(resolveMapping(pathMapping)).rejects.toThrow(/Relative path must/);

      mockPlatform = 'linux';
      await expect(resolveMapping(pathMapping)).rejects.toThrow(/Relative path must/);

      mockPlatform = 'macos';
      await expect(resolveMapping(pathMapping)).rejects.toThrow(/Relative path must/);

      mockPlatform = 'windows';
      await expect(resolveMapping(pathMapping)).rejects.toThrow(/Relative path must/);
   });
});
