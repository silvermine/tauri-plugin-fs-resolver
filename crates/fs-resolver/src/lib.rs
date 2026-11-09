mod android_paths;
#[cfg(any(target_os = "ios", target_os = "macos"))]
mod backup_exclusion;
mod error;
mod fs_environment;
mod ios_paths;
mod ios_resolve;
mod linux_paths;
mod linux_resolve;
mod mac_paths;
mod mac_resolve;
mod path_mapping;
mod path_resolver;
mod windows_paths;
mod windows_resolve;

pub use android_paths::{AndroidPath, AndroidPathCollection};
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use backup_exclusion::{is_excluded_from_backup, set_excluded_from_backup};
pub use error::{Error, Result};
pub use fs_environment::FsEnvironment;
pub use ios_paths::IosPath;
pub use linux_paths::LinuxPath;
pub use mac_paths::MacPath;
pub use path_mapping::{CrossPlatformMapping, PlatformMapping, WinPackagedPathMapping};
pub use path_resolver::PathResolver;
pub use windows_paths::{Win32Path, WindowsApplicationDataPath, WindowsLibrary};
