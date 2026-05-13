use crate::error::Result;
use crate::ios_paths::IosPath;
use crate::{AndroidPathCollection, Error, LinuxPath, PathMapping, check_os};
use std::path::PathBuf;

use crate::android_paths::AndroidPath;
use crate::mac_paths::MacPath;
use crate::windows_paths::WindowsPath;

pub struct PathResolver {
   os: String,
   resolve_android: fn(&AndroidPath) -> Result<PathBuf>,
   resolve_android_path_collection: fn(&AndroidPathCollection) -> Result<Vec<PathBuf>>,
   resolve_ios: fn(&IosPath) -> Result<PathBuf>,
   resolve_linux: fn(&LinuxPath) -> Result<PathBuf>,
   resolve_mac: fn(&MacPath) -> Result<PathBuf>,
   resolve_windows: fn(&WindowsPath) -> Result<PathBuf>,
}

impl Default for PathResolver {
   fn default() -> Self {
      Self::new()
   }
}

impl PathResolver {
   pub fn new() -> Self {
      Self {
         os: std::env::consts::OS.to_string(),
         resolve_android: crate::android_resolve::resolve_android_path,
         resolve_android_path_collection: crate::android_resolve::resolve_android_path_collection,
         resolve_ios: crate::ios_resolve::resolve_ios_path,
         resolve_linux: crate::linux_resolve::resolve_linux_path,
         resolve_mac: crate::mac_resolve::resolve_mac_path,
         resolve_windows: crate::windows_resolve::resolve_windows_path,
      }
   }

   #[cfg(test)]
   pub fn new_for_test(
      os: String,
      resolve_android: fn(&AndroidPath) -> Result<PathBuf>,
      resolve_android_path_collection: fn(&AndroidPathCollection) -> Result<Vec<PathBuf>>,
      resolve_ios: fn(&IosPath) -> Result<PathBuf>,
      resolve_linux: fn(&LinuxPath) -> Result<PathBuf>,
      resolve_mac: fn(&MacPath) -> Result<PathBuf>,
      resolve_windows: fn(&WindowsPath) -> Result<PathBuf>,
   ) -> Self {
      Self {
         os,
         resolve_android,
         resolve_android_path_collection,
         resolve_ios,
         resolve_linux,
         resolve_mac,
         resolve_windows,
      }
   }

   pub fn resolve_mapping(&self, mapping: &PathMapping) -> Result<PathBuf> {
      match self.os.as_str() {
         "android" => {
            if let Some(path) = &mapping.android {
               return self.resolve_android(path);
            }

            Err(Error::InvalidPath("Android path not defined".to_string()))
         }
         "ios" => {
            if let Some(path) = &mapping.ios {
               return self.resolve_ios(path);
            }

            Err(Error::InvalidPath("iOS path not defined".to_string()))
         }
         "linux" => {
            if let Some(path) = &mapping.linux {
               return self.resolve_linux(path);
            }

            Err(Error::InvalidPath("Linux path not defined".to_string()))
         }
         "macos" => {
            if let Some(path) = &mapping.macos {
               return self.resolve_mac(path);
            }

            Err(Error::InvalidPath("macOS path not defined".to_string()))
         }
         "windows" => {
            if let Some(path) = &mapping.windows {
               return self.resolve_windows(path);
            }

            Err(Error::InvalidPath("Windows path not defined".to_string()))
         }
         _ => Err(Error::UnsupportedPlatform(self.os.clone())),
      }
   }

   pub fn resolve_android(&self, path: &AndroidPath) -> Result<PathBuf> {
      check_os(&["android"], &self.os)?;
      (self.resolve_android)(path)
   }

   pub fn resolve_android_path_collection(
      &self,
      collection: &AndroidPathCollection,
   ) -> Result<Vec<PathBuf>> {
      check_os(&["android"], &self.os)?;
      (self.resolve_android_path_collection)(collection)
   }

   pub fn resolve_ios(&self, path: &IosPath) -> Result<PathBuf> {
      check_os(&["ios"], &self.os)?;
      (self.resolve_ios)(path)
   }

   pub fn resolve_linux(&self, path: &LinuxPath) -> Result<PathBuf> {
      check_os(&["linux"], &self.os)?;
      (self.resolve_linux)(path)
   }

   pub fn resolve_mac(&self, path: &MacPath) -> Result<PathBuf> {
      check_os(&["macos"], &self.os)?;
      (self.resolve_mac)(path)
   }

   pub fn resolve_windows(&self, path: &WindowsPath) -> Result<PathBuf> {
      check_os(&["windows"], &self.os)?;
      (self.resolve_windows)(path)
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   use crate::Error;
   use crate::Win32Path;

   const KNOWN_OS: [&str; 4] = ["ios", "macos", "android", "windows"];

   #[test]
   fn resolve_handles_invoking_os() {
      let android_string = "android";
      let ios_string = "ios";
      let linux_string = "linux";
      let macos_string = "macos";
      let windows_string = "windows";

      // Android
      let android_resolver = create_test_resolver(android_string.to_string());

      let android_result = android_resolver.resolve_android(&AndroidPath::DataDir);
      assert_eq!(android_result.unwrap(), PathBuf::from("android/dataDir"));

      let android_collection_result = android_resolver
         .resolve_android_path_collection(&AndroidPathCollection::ExternalCacheDirs);
      assert_eq!(
         android_collection_result.unwrap(),
         vec![PathBuf::from("android/externalCacheDirs")]
      );

      let ios_result = android_resolver.resolve_ios(&IosPath::DocumentDirectory);
      assert_eq!(
         ios_result.unwrap_err(),
         Error::IncorrectOS(android_string.to_string(), "ios".to_string())
      );

      let linux_result = android_resolver.resolve_linux(&LinuxPath::UserHomeDirectory);
      assert_eq!(
         linux_result.unwrap_err(),
         Error::IncorrectOS(android_string.to_string(), "linux".to_string())
      );

      let mac_result = android_resolver.resolve_mac(&MacPath::ApplicationDirectory);
      assert_eq!(
         mac_result.unwrap_err(),
         Error::IncorrectOS(android_string.to_string(), "macos".to_string())
      );

      let windows_result =
         android_resolver.resolve_windows(&WindowsPath::Win32(Win32Path::LocalAppData));
      assert_eq!(
         windows_result.unwrap_err(),
         Error::IncorrectOS(android_string.to_string(), windows_string.to_string())
      );

      // iOS
      let ios_resolver = create_test_resolver(ios_string.to_string());

      let android_result = ios_resolver.resolve_android(&AndroidPath::DataDir);
      assert_eq!(
         android_result.unwrap_err(),
         Error::IncorrectOS(ios_string.to_string(), android_string.to_string())
      );

      let android_collection_result =
         ios_resolver.resolve_android_path_collection(&AndroidPathCollection::ExternalCacheDirs);
      assert_eq!(
         android_collection_result.unwrap_err(),
         Error::IncorrectOS(ios_string.to_string(), android_string.to_string())
      );

      let ios_result = ios_resolver.resolve_ios(&IosPath::DocumentDirectory);
      assert_eq!(ios_result.unwrap(), PathBuf::from("ios/documentDirectory"));

      let linux_result = ios_resolver.resolve_linux(&LinuxPath::UserHomeDirectory);
      assert_eq!(
         linux_result.unwrap_err(),
         Error::IncorrectOS(ios_string.to_string(), "linux".to_string())
      );

      let mac_result = ios_resolver.resolve_mac(&MacPath::ApplicationDirectory);
      assert_eq!(
         mac_result.unwrap_err(),
         Error::IncorrectOS(ios_string.to_string(), "macos".to_string())
      );

      let windows_result =
         ios_resolver.resolve_windows(&WindowsPath::Win32(Win32Path::LocalAppData));
      assert_eq!(
         windows_result.unwrap_err(),
         Error::IncorrectOS(ios_string.to_string(), windows_string.to_string())
      );

      // Linux
      let linux_resolver = create_test_resolver(linux_string.to_string());

      let android_result = linux_resolver.resolve_android(&AndroidPath::DataDir);
      assert_eq!(
         android_result.unwrap_err(),
         Error::IncorrectOS(linux_string.to_string(), android_string.to_string())
      );

      let android_collection_result =
         linux_resolver.resolve_android_path_collection(&AndroidPathCollection::ExternalCacheDirs);
      assert_eq!(
         android_collection_result.unwrap_err(),
         Error::IncorrectOS(linux_string.to_string(), android_string.to_string())
      );

      let ios_result = linux_resolver.resolve_ios(&IosPath::DocumentDirectory);
      assert_eq!(
         ios_result.unwrap_err(),
         Error::IncorrectOS(linux_string.to_string(), "ios".to_string())
      );

      let linux_result = linux_resolver.resolve_linux(&LinuxPath::UserHomeDirectory);
      assert_eq!(
         linux_result.unwrap(),
         PathBuf::from("linux/userHomeDirectory")
      );

      let mac_result = linux_resolver.resolve_mac(&MacPath::ApplicationDirectory);
      assert_eq!(
         mac_result.unwrap_err(),
         Error::IncorrectOS(linux_string.to_string(), "macos".to_string())
      );

      let windows_result =
         linux_resolver.resolve_windows(&WindowsPath::Win32(Win32Path::LocalAppData));
      assert_eq!(
         windows_result.unwrap_err(),
         Error::IncorrectOS(linux_string.to_string(), windows_string.to_string())
      );

      // macOS
      let macos_resolver = create_test_resolver(macos_string.to_string());

      let android_result = macos_resolver.resolve_android(&AndroidPath::DataDir);
      assert_eq!(
         android_result.unwrap_err(),
         Error::IncorrectOS(macos_string.to_string(), android_string.to_string())
      );

      let android_collection_result =
         macos_resolver.resolve_android_path_collection(&AndroidPathCollection::ExternalCacheDirs);
      assert_eq!(
         android_collection_result.unwrap_err(),
         Error::IncorrectOS(macos_string.to_string(), android_string.to_string())
      );

      let ios_result = macos_resolver.resolve_ios(&IosPath::DocumentDirectory);
      assert_eq!(
         ios_result.unwrap_err(),
         Error::IncorrectOS(macos_string.to_string(), "ios".to_string())
      );

      let linux_result = macos_resolver.resolve_linux(&LinuxPath::UserHomeDirectory);
      assert_eq!(
         linux_result.unwrap_err(),
         Error::IncorrectOS(macos_string.to_string(), "linux".to_string())
      );

      let mac_result = macos_resolver.resolve_mac(&MacPath::ApplicationDirectory);
      assert_eq!(
         mac_result.unwrap(),
         PathBuf::from("apple/applicationDirectory")
      );

      let windows_result =
         macos_resolver.resolve_windows(&WindowsPath::Win32(Win32Path::LocalAppData));
      assert_eq!(
         windows_result.unwrap_err(),
         Error::IncorrectOS(macos_string.to_string(), windows_string.to_string())
      );

      // Windows
      let windows_resolver = create_test_resolver(windows_string.to_string());

      let android_result = windows_resolver.resolve_android(&AndroidPath::DataDir);
      assert_eq!(
         android_result.unwrap_err(),
         Error::IncorrectOS(windows_string.to_string(), android_string.to_string())
      );

      let android_collection_result = windows_resolver
         .resolve_android_path_collection(&AndroidPathCollection::ExternalCacheDirs);
      assert_eq!(
         android_collection_result.unwrap_err(),
         Error::IncorrectOS(windows_string.to_string(), android_string.to_string())
      );

      let ios_result = windows_resolver.resolve_ios(&IosPath::DocumentDirectory);
      assert_eq!(
         ios_result.unwrap_err(),
         Error::IncorrectOS(windows_string.to_string(), "ios".to_string())
      );

      let linux_result = windows_resolver.resolve_linux(&LinuxPath::UserHomeDirectory);
      assert_eq!(
         linux_result.unwrap_err(),
         Error::IncorrectOS(windows_string.to_string(), "linux".to_string())
      );

      let mac_result = windows_resolver.resolve_mac(&MacPath::ApplicationDirectory);
      assert_eq!(
         mac_result.unwrap_err(),
         Error::IncorrectOS(windows_string.to_string(), "macos".to_string())
      );

      let windows_result =
         windows_resolver.resolve_windows(&WindowsPath::Win32(Win32Path::LocalAppData));
      assert_eq!(
         windows_result.unwrap(),
         PathBuf::from("windows/win32::localAppData")
      );
   }

   #[test]
   fn resolve_path_mapping_maps_to_correct_os() {
      let path_mapping = PathMapping {
         android: Some(AndroidPath::DataDir),
         ios: Some(IosPath::DocumentDirectory),
         linux: Some(LinuxPath::UserHomeDirectory),
         macos: Some(MacPath::ApplicationDirectory),
         windows: Some(WindowsPath::Win32(Win32Path::LocalAppData)),
      };

      for os in KNOWN_OS {
         let resolver = create_test_resolver(os.to_string());
         let resolved = resolver.resolve_mapping(&path_mapping).unwrap();

         if os == "android" {
            assert_eq!(
               resolved,
               PathBuf::from("android/dataDir"),
               "Incorrect path for android with os {}",
               os
            );
         } else if os == "ios" {
            assert_eq!(
               resolved,
               PathBuf::from("ios/documentDirectory"),
               "Incorrect path for ios with os {}",
               os
            );
         } else if os == "linux" {
            assert_eq!(
               resolved,
               PathBuf::from("linux/homeDirectory"),
               "Incorrect path for linux with os {}",
               os
            );
         } else if os == "macos" {
            assert_eq!(
               resolved,
               PathBuf::from("apple/applicationDirectory"),
               "Incorrect path for macos with os {}",
               os
            );
         } else if os == "windows" {
            assert_eq!(
               resolved,
               PathBuf::from("windows/win32::localAppData"),
               "Incorrect path for windows with os {}",
               os
            );
         }
      }
   }

   #[test]
   fn resolve_path_mapping_with_undefined_path_returns_error() {
      let path_mapping = PathMapping {
         android: None,
         ios: None,
         linux: None,
         macos: None,
         windows: None,
      };

      for os in KNOWN_OS {
         let resolver = create_test_resolver(os.to_string());
         let error_str = resolver
            .resolve_mapping(&path_mapping)
            .unwrap_err()
            .to_string();

         if os == "android" {
            assert_eq!(
               error_str,
               Error::InvalidPath("Android path not defined".to_string()).to_string()
            );
         } else if os == "ios" {
            assert_eq!(
               error_str,
               Error::InvalidPath("iOS path not defined".to_string()).to_string()
            );
         } else if os == "linux" {
            assert_eq!(
               error_str,
               Error::InvalidPath("Linux path not defined".to_string()).to_string()
            );
         } else if os == "macos" {
            assert_eq!(
               error_str,
               Error::InvalidPath("macOS path not defined".to_string()).to_string()
            );
         } else if os == "windows" {
            assert_eq!(
               error_str,
               Error::InvalidPath("Windows path not defined".to_string()).to_string()
            );
         }
      }
   }

   fn create_test_resolver(os: String) -> PathResolver {
      PathResolver::new_for_test(
         os,
         |path: &AndroidPath| -> Result<PathBuf> { Ok(PathBuf::from(format!("android/{}", path))) },
         |collection: &AndroidPathCollection| -> Result<Vec<PathBuf>> {
            Ok(vec![PathBuf::from(format!("android/{}", collection))])
         },
         |path: &IosPath| -> Result<PathBuf> { Ok(PathBuf::from(format!("ios/{}", path))) },
         |path: &LinuxPath| -> Result<PathBuf> { Ok(PathBuf::from(format!("linux/{}", path))) },
         |path: &MacPath| -> Result<PathBuf> { Ok(PathBuf::from(format!("apple/{}", path))) },
         |path: &WindowsPath| -> Result<PathBuf> { Ok(PathBuf::from(format!("windows/{}", path))) },
      )
   }
}
