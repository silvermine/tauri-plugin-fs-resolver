use std::fmt::Display;

use serde::Serialize;

use crate::error::{Error, Result};

/// The filesystem environment of the current operating system.
///
/// This is used to determine the appropriate filesystem path resolution strategy.
///
/// Q: Why do we need this enum instead of the string for std::env::consts::OS?
/// A: This solves a specific Windows problem, where Windows can be either Win32 or WinPackaged.
/// The os string alone is not enough to determine the environment.
/// Win32 paths can be resolved on both Win32 and WinPackaged, but WinPackaged paths cannot be resolved on Win32.
/// Having a declaritive enum is the most robust solution for this problem,
/// which allows us to determine the environment once at runtime and hold the value for the lifetime of the application.
#[derive(Debug, PartialEq, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FsEnvironment {
   Android,
   Ios,
   Linux,
   Macos,
   Win32,
   WinPackaged,
}

impl Display for FsEnvironment {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
         FsEnvironment::Android => write!(f, "android"),
         FsEnvironment::Ios => write!(f, "ios"),
         FsEnvironment::Linux => write!(f, "linux"),
         FsEnvironment::Macos => write!(f, "macos"),
         FsEnvironment::Win32 => write!(f, "win32"),
         FsEnvironment::WinPackaged => write!(f, "winpackaged"),
      }
   }
}

pub(crate) fn get_fs_environment() -> Result<FsEnvironment> {
   let os_string = std::env::consts::OS.to_string();
   get_fs_environment_inner(&os_string, is_windows_packaged)
}

/// Inner implementation used for testing.
fn get_fs_environment_inner(
   os_string: &str,
   check_is_windows_packaged: impl FnOnce() -> Result<bool>,
) -> Result<FsEnvironment> {
   match os_string {
      "android" => Ok(FsEnvironment::Android),
      "ios" => Ok(FsEnvironment::Ios),
      "linux" => Ok(FsEnvironment::Linux),
      "macos" => Ok(FsEnvironment::Macos),
      "windows" => {
         let is_packaged = check_is_windows_packaged()?;
         if is_packaged {
            Ok(FsEnvironment::WinPackaged)
         } else {
            Ok(FsEnvironment::Win32)
         }
      }
      _ => Err(Error::UnsupportedEnvironment(os_string.to_string())),
   }
}

fn is_windows_packaged() -> Result<bool> {
   #[cfg(target_os = "windows")]
   {
      use windows_sys::Win32::{
         Foundation::{APPMODEL_ERROR_NO_PACKAGE, ERROR_INSUFFICIENT_BUFFER},
         Storage::Packaging::Appx::GetCurrentPackageFullName,
      };

      // See reference implementation here:
      // https://learn.microsoft.com/en-us/windows/msix/detect-package-identity
      let mut length = 0u32;
      let return_code = unsafe { GetCurrentPackageFullName(&mut length, std::ptr::null_mut()) };
      match return_code {
         APPMODEL_ERROR_NO_PACKAGE => Ok(false),

         // This error is expected, as we have purposely passed a zero length buffer.
         // ERROR_INSUFFICIENT_BUFFER on a null/zero-length probe means the process has
         // package identity; the API is reporting how large a buffer the name needs.
         // We do not allocate or call again — identity alone is enough for packaged vs Win32.
         // If we do need to actually read the package name, we can call to
         // GetCurrentPackageFullName again with a non-zero length buffer.
         ERROR_INSUFFICIENT_BUFFER => Ok(true),
         _ => Err(Error::CouldNotDetermineWindowsPackagingEnvironment(
            format!("GetCurrentPackageFullName returned: {}", return_code),
         )),
      }
   }
   #[cfg(not(target_os = "windows"))]
   {
      Err(Error::AttemptedDeterminingWindowsPackagingEnvironmentOnNonWindowsPlatform)
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   use test_case::test_case;

   #[test_case("android", FsEnvironment::Android)]
   #[test_case("ios", FsEnvironment::Ios)]
   #[test_case("linux", FsEnvironment::Linux)]
   #[test_case("macos", FsEnvironment::Macos)]
   fn test_get_non_windows_fs_environment(os: &str, expected: FsEnvironment) {
      assert_eq!(
         get_fs_environment_inner(os, || Err(
            Error::AttemptedDeterminingWindowsPackagingEnvironmentOnNonWindowsPlatform
         ))
         .unwrap(),
         expected
      );
   }

   #[test_case(false, FsEnvironment::Win32)]
   #[test_case(true, FsEnvironment::WinPackaged)]
   fn test_get_windows_fs_environment(is_packaged: bool, expected: FsEnvironment) {
      assert_eq!(
         get_fs_environment_inner("windows", || Ok(is_packaged)).unwrap(),
         expected
      );
   }
}
