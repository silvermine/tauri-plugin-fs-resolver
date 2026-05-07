import { platform } from '@tauri-apps/plugin-os';
import { resolveAndroidPath, resolveIosPath, resolveMacPath, resolveWindowsPath } from './index';
import { AndroidPath, IosPath, LinuxPath, MacPath, WindowsPath } from './types';

export class PathMapping {
   private _ios?: IosPath;
   private _macos?: MacPath;
   private _linux?: LinuxPath;
   private _android?: AndroidPath;
   private _windows?: WindowsPath;

   public constructor(paths: {
      android?: AndroidPath;
      ios?: IosPath;
      linux?: LinuxPath;
      macos?: MacPath;
      windows?: WindowsPath;
   }) {
      this._android = paths.android;
      this._ios = paths.ios;
      this._linux = paths.linux;
      this._macos = paths.macos;
      this._windows = paths.windows;
   }

   public async resolve(): Promise<string> {
      const os = platform();

      switch (os) {
         case 'android': {
            if (this._android) {
               return await resolveAndroidPath(this._android);
            }

            throw new Error('No path defined for Android');
         }

         case 'ios': {
            if (this._ios) {
               return await resolveIosPath(this._ios);
            }

            throw new Error('No path defined for iOS');
         }

         case 'linux': {
            if (this._linux) {
               return await resolveLinuxPath(this._linux);
            }

            throw new Error('No path defined for Linux');
         }

         case 'macos': {
            if (this._macos) {
               return await resolveMacPath(this._macos);
            }

            throw new Error('No path defined for macOS');
         }

         case 'windows': {
            if (this._windows) {
               return await resolveWindowsPath(this._windows);
            }

            throw new Error('No path defined for Windows');
         }

         default: {
            throw new Error(`Unsupported platform: ${os}`);
         }
      }
   }
}
