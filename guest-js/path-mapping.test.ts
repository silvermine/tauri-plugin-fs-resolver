import { describe, it, expect, afterEach } from 'vitest';
import { clearMocks } from '@tauri-apps/api/mocks';

import { AndroidPath, IosPath, MacPath, Win32Path } from './types';
import { PathMapping } from './path-mapping';

const resolvedAndroidPath = 'android_path',
      resolvedMacPath = 'mac_path',
      resolvedIosPath = 'ios_path',
      resolvedWindowsPath = 'windows_path';

let mockPlatform: '' | 'android' | 'ios' | 'macos' | 'windows' = '';

vi.mock('@tauri-apps/plugin-os', () => {
   return {
      platform: () => { return mockPlatform; },
   };
});

vi.mock('./index', () => {
   return {
      resolveAndroidPath: () => { return resolvedAndroidPath; },
      resolveIosPath: () => { return resolvedIosPath; },
      resolveMacPath: () => { return resolvedMacPath; },
      resolveWindowsPath: () => { return resolvedWindowsPath; },
   };
});

afterEach(() => { return clearMocks(); });

describe('fs-resolver actions map to Tauri commands', () => {
   it('resolveAndroidPath — sends path, returns resolved path', async () => {

      const pathMapping = new PathMapping({
         android: AndroidPath.DataDir,
         ios: IosPath.CachesDirectory,
         macos: MacPath.CachesDirectory,
         windows: { win32: Win32Path.LocalAppData },
      });

      mockPlatform = 'android';
      expect(await pathMapping.resolve()).toBe(resolvedAndroidPath);

      mockPlatform = 'windows';
      expect(await pathMapping.resolve()).toBe(resolvedWindowsPath);

      mockPlatform = 'ios';
      expect(await pathMapping.resolve()).toBe(resolvedIosPath);

      mockPlatform = 'macos';
      expect(await pathMapping.resolve()).toBe(resolvedMacPath);
   });
});
