use crate::android_paths::AndroidPath;
use crate::ios_paths::IosPath;
use crate::linux_paths::LinuxPath;
use crate::mac_paths::MacPath;
use crate::windows_paths::WindowsPath;

#[derive(Debug)]
pub struct PathMapping {
   pub android: Option<AndroidPath>,
   pub ios: Option<IosPath>,
   pub macos: Option<MacPath>,
   pub linux: Option<LinuxPath>,
   pub windows: Option<WindowsPath>,
}
