use crate::error::Result;
use crate::ios_paths::IosPath;
use crate::{AndroidPathCollection, Error, LinuxPath, PathMapping};
use std::fmt::Display;
use std::path::PathBuf;

use crate::android_paths::AndroidPath;
use crate::mac_paths::MacPath;
use crate::windows_paths::WindowsPath;

type AndroidPathResolver = Box<dyn Fn(&AndroidPath) -> Result<PathBuf> + Send + Sync>;
type AndroidPathCollectionResolver =
   Box<dyn Fn(&AndroidPathCollection) -> Result<Vec<PathBuf>> + Send + Sync>;

pub struct PathResolver {
   os: String,
   resolve_android: AndroidPathResolver,
   resolve_android_path_collection: AndroidPathCollectionResolver,
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
         resolve_android: Box::new(|_| Err(Error::AndroidPathResolutionNotConfigured)),
         resolve_android_path_collection: Box::new(|_| {
            Err(Error::AndroidPathResolutionNotConfigured)
         }),
         resolve_ios: crate::ios_resolve::resolve_ios_path,
         resolve_linux: crate::linux_resolve::resolve_linux_path,
         resolve_mac: crate::mac_resolve::resolve_mac_path,
         resolve_windows: crate::windows_resolve::resolve_windows_path,
      }
   }

   pub fn configure_android_path_resolution(
      &mut self,
      resolve_android: AndroidPathResolver,
      resolve_android_path_collection: AndroidPathCollectionResolver,
   ) -> &mut Self {
      self.resolve_android = resolve_android;
      self.resolve_android_path_collection = resolve_android_path_collection;
      self
   }

   #[cfg(test)]
   pub fn new_for_test(
      os: String,
      resolve_android: AndroidPathResolver,
      resolve_android_path_collection: AndroidPathCollectionResolver,
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

            Err(Error::PathMappingUndefined("android".to_string()))
         }
         "ios" => {
            if let Some(path) = &mapping.ios {
               return self.resolve_ios(path);
            }

            Err(Error::PathMappingUndefined("ios".to_string()))
         }
         "linux" => {
            if let Some(path) = &mapping.linux {
               return self.resolve_linux(path);
            }

            Err(Error::PathMappingUndefined("linux".to_string()))
         }
         "macos" => {
            if let Some(path) = &mapping.macos {
               return self.resolve_mac(path);
            }

            Err(Error::PathMappingUndefined("macos".to_string()))
         }
         "windows" => {
            if let Some(path) = &mapping.windows {
               return self.resolve_windows(path);
            }

            Err(Error::PathMappingUndefined("windows".to_string()))
         }
         _ => Err(Error::UnsupportedPlatform(self.os.clone())),
      }
   }

   pub fn resolve_android(&self, path: &AndroidPath) -> Result<PathBuf> {
      Self::check_os(path, &["android"], &self.os)?;
      (self.resolve_android)(path)
   }

   pub fn resolve_android_path_collection(
      &self,
      collection: &AndroidPathCollection,
   ) -> Result<Vec<PathBuf>> {
      Self::check_os(collection, &["android"], &self.os)?;
      (self.resolve_android_path_collection)(collection)
   }

   pub fn resolve_ios(&self, path: &IosPath) -> Result<PathBuf> {
      Self::check_os(path, &["ios"], &self.os)?;
      (self.resolve_ios)(path)
   }

   pub fn resolve_linux(&self, path: &LinuxPath) -> Result<PathBuf> {
      Self::check_os(path, &["linux"], &self.os)?;
      (self.resolve_linux)(path)
   }

   pub fn resolve_mac(&self, path: &MacPath) -> Result<PathBuf> {
      Self::check_os(path, &["macos"], &self.os)?;
      (self.resolve_mac)(path)
   }

   pub fn resolve_windows(&self, path: &WindowsPath) -> Result<PathBuf> {
      Self::check_os(path, &["windows"], &self.os)?;
      (self.resolve_windows)(path)
   }

   fn check_os(path_string: &dyn Display, expected: &[&str], actual: &str) -> Result<()> {
      if !expected.contains(&actual) {
         return Err(Error::IncorrectOS {
            path: path_string.to_string(),
            current_os: actual.to_string(),
            expected_os: expected.join(", "),
         });
      }

      Ok(())
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   use crate::Error;
   use crate::Win32Path;

   const KNOWN_OS: [&str; 5] = ["ios", "macos", "android", "windows", "linux"];

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
         Error::IncorrectOS {
            path: IosPath::DocumentDirectory.to_string(),
            current_os: android_string.to_string(),
            expected_os: "ios".to_string(),
         }
      );

      let linux_result = android_resolver.resolve_linux(&LinuxPath::UserHomeDirectory);
      assert_eq!(
         linux_result.unwrap_err(),
         Error::IncorrectOS {
            path: LinuxPath::UserHomeDirectory.to_string(),
            current_os: android_string.to_string(),
            expected_os: "linux".to_string(),
         }
      );

      let mac_result = android_resolver.resolve_mac(&MacPath::ApplicationDirectory);
      assert_eq!(
         mac_result.unwrap_err(),
         Error::IncorrectOS {
            path: MacPath::ApplicationDirectory.to_string(),
            current_os: android_string.to_string(),
            expected_os: "macos".to_string(),
         }
      );

      let windows_result =
         android_resolver.resolve_windows(&WindowsPath::Win32(Win32Path::LocalAppData));
      assert_eq!(
         windows_result.unwrap_err(),
         Error::IncorrectOS {
            path: WindowsPath::Win32(Win32Path::LocalAppData).to_string(),
            current_os: android_string.to_string(),
            expected_os: windows_string.to_string(),
         }
      );

      // iOS
      let ios_resolver = create_test_resolver(ios_string.to_string());

      let android_result = ios_resolver.resolve_android(&AndroidPath::DataDir);
      assert_eq!(
         android_result.unwrap_err(),
         Error::IncorrectOS {
            path: AndroidPath::DataDir.to_string(),
            current_os: ios_string.to_string(),
            expected_os: android_string.to_string(),
         }
      );

      let android_collection_result =
         ios_resolver.resolve_android_path_collection(&AndroidPathCollection::ExternalCacheDirs);
      assert_eq!(
         android_collection_result.unwrap_err(),
         Error::IncorrectOS {
            path: AndroidPathCollection::ExternalCacheDirs.to_string(),
            current_os: ios_string.to_string(),
            expected_os: android_string.to_string(),
         }
      );

      let ios_result = ios_resolver.resolve_ios(&IosPath::DocumentDirectory);
      assert_eq!(ios_result.unwrap(), PathBuf::from("ios/documentDirectory"));

      let linux_result = ios_resolver.resolve_linux(&LinuxPath::UserHomeDirectory);
      assert_eq!(
         linux_result.unwrap_err(),
         Error::IncorrectOS {
            path: LinuxPath::UserHomeDirectory.to_string(),
            current_os: ios_string.to_string(),
            expected_os: "linux".to_string(),
         }
      );

      let mac_result = ios_resolver.resolve_mac(&MacPath::ApplicationDirectory);
      assert_eq!(
         mac_result.unwrap_err(),
         Error::IncorrectOS {
            path: MacPath::ApplicationDirectory.to_string(),
            current_os: ios_string.to_string(),
            expected_os: "macos".to_string(),
         }
      );

      let windows_result =
         ios_resolver.resolve_windows(&WindowsPath::Win32(Win32Path::LocalAppData));
      assert_eq!(
         windows_result.unwrap_err(),
         Error::IncorrectOS {
            path: WindowsPath::Win32(Win32Path::LocalAppData).to_string(),
            current_os: ios_string.to_string(),
            expected_os: windows_string.to_string(),
         }
      );

      // Linux
      let linux_resolver = create_test_resolver(linux_string.to_string());

      let android_result = linux_resolver.resolve_android(&AndroidPath::DataDir);
      assert_eq!(
         android_result.unwrap_err(),
         Error::IncorrectOS {
            path: AndroidPath::DataDir.to_string(),
            current_os: linux_string.to_string(),
            expected_os: android_string.to_string(),
         }
      );

      let android_collection_result =
         linux_resolver.resolve_android_path_collection(&AndroidPathCollection::ExternalCacheDirs);
      assert_eq!(
         android_collection_result.unwrap_err(),
         Error::IncorrectOS {
            path: AndroidPathCollection::ExternalCacheDirs.to_string(),
            current_os: linux_string.to_string(),
            expected_os: android_string.to_string(),
         }
      );

      let ios_result = linux_resolver.resolve_ios(&IosPath::DocumentDirectory);
      assert_eq!(
         ios_result.unwrap_err(),
         Error::IncorrectOS {
            path: IosPath::DocumentDirectory.to_string(),
            current_os: linux_string.to_string(),
            expected_os: "ios".to_string(),
         }
      );

      let linux_result = linux_resolver.resolve_linux(&LinuxPath::UserHomeDirectory);
      assert_eq!(
         linux_result.unwrap(),
         PathBuf::from("linux/userHomeDirectory")
      );

      let mac_result = linux_resolver.resolve_mac(&MacPath::ApplicationDirectory);
      assert_eq!(
         mac_result.unwrap_err(),
         Error::IncorrectOS {
            path: MacPath::ApplicationDirectory.to_string(),
            current_os: linux_string.to_string(),
            expected_os: "macos".to_string(),
         }
      );

      let windows_result =
         linux_resolver.resolve_windows(&WindowsPath::Win32(Win32Path::LocalAppData));
      assert_eq!(
         windows_result.unwrap_err(),
         Error::IncorrectOS {
            path: WindowsPath::Win32(Win32Path::LocalAppData).to_string(),
            current_os: linux_string.to_string(),
            expected_os: windows_string.to_string(),
         }
      );

      // macOS
      let macos_resolver = create_test_resolver(macos_string.to_string());

      let android_result = macos_resolver.resolve_android(&AndroidPath::DataDir);
      assert_eq!(
         android_result.unwrap_err(),
         Error::IncorrectOS {
            path: AndroidPath::DataDir.to_string(),
            current_os: macos_string.to_string(),
            expected_os: android_string.to_string(),
         }
      );

      let android_collection_result =
         macos_resolver.resolve_android_path_collection(&AndroidPathCollection::ExternalCacheDirs);
      assert_eq!(
         android_collection_result.unwrap_err(),
         Error::IncorrectOS {
            path: AndroidPathCollection::ExternalCacheDirs.to_string(),
            current_os: macos_string.to_string(),
            expected_os: android_string.to_string(),
         }
      );

      let ios_result = macos_resolver.resolve_ios(&IosPath::DocumentDirectory);
      assert_eq!(
         ios_result.unwrap_err(),
         Error::IncorrectOS {
            path: IosPath::DocumentDirectory.to_string(),
            current_os: macos_string.to_string(),
            expected_os: "ios".to_string(),
         }
      );

      let linux_result = macos_resolver.resolve_linux(&LinuxPath::UserHomeDirectory);
      assert_eq!(
         linux_result.unwrap_err(),
         Error::IncorrectOS {
            path: LinuxPath::UserHomeDirectory.to_string(),
            current_os: macos_string.to_string(),
            expected_os: "linux".to_string(),
         }
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
         Error::IncorrectOS {
            path: WindowsPath::Win32(Win32Path::LocalAppData).to_string(),
            current_os: macos_string.to_string(),
            expected_os: windows_string.to_string(),
         }
      );

      // Windows
      let windows_resolver = create_test_resolver(windows_string.to_string());

      let android_result = windows_resolver.resolve_android(&AndroidPath::DataDir);
      assert_eq!(
         android_result.unwrap_err(),
         Error::IncorrectOS {
            path: AndroidPath::DataDir.to_string(),
            current_os: windows_string.to_string(),
            expected_os: android_string.to_string(),
         }
      );

      let android_collection_result = windows_resolver
         .resolve_android_path_collection(&AndroidPathCollection::ExternalCacheDirs);
      assert_eq!(
         android_collection_result.unwrap_err(),
         Error::IncorrectOS {
            path: AndroidPathCollection::ExternalCacheDirs.to_string(),
            current_os: windows_string.to_string(),
            expected_os: android_string.to_string(),
         }
      );

      let ios_result = windows_resolver.resolve_ios(&IosPath::DocumentDirectory);
      assert_eq!(
         ios_result.unwrap_err(),
         Error::IncorrectOS {
            path: IosPath::DocumentDirectory.to_string(),
            current_os: windows_string.to_string(),
            expected_os: "ios".to_string(),
         }
      );

      let linux_result = windows_resolver.resolve_linux(&LinuxPath::UserHomeDirectory);
      assert_eq!(
         linux_result.unwrap_err(),
         Error::IncorrectOS {
            path: LinuxPath::UserHomeDirectory.to_string(),
            current_os: windows_string.to_string(),
            expected_os: "linux".to_string(),
         }
      );

      let mac_result = windows_resolver.resolve_mac(&MacPath::ApplicationDirectory);
      assert_eq!(
         mac_result.unwrap_err(),
         Error::IncorrectOS {
            path: MacPath::ApplicationDirectory.to_string(),
            current_os: windows_string.to_string(),
            expected_os: "macos".to_string(),
         }
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
               PathBuf::from("linux/userHomeDirectory"),
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
               Error::PathMappingUndefined("android".to_string()).to_string()
            );
         } else if os == "ios" {
            assert_eq!(
               error_str,
               Error::PathMappingUndefined("ios".to_string()).to_string()
            );
         } else if os == "linux" {
            assert_eq!(
               error_str,
               Error::PathMappingUndefined("linux".to_string()).to_string()
            );
         } else if os == "macos" {
            assert_eq!(
               error_str,
               Error::PathMappingUndefined("macos".to_string()).to_string()
            );
         } else if os == "windows" {
            assert_eq!(
               error_str,
               Error::PathMappingUndefined("windows".to_string()).to_string()
            );
         }
      }
   }

   fn create_test_resolver(os: String) -> PathResolver {
      let resolve_android = Box::new(|path: &AndroidPath| -> Result<PathBuf> {
         Ok(PathBuf::from(format!("android/{}", path)))
      });

      let resolve_android_path_collection = Box::new(
         |collection: &AndroidPathCollection| -> Result<Vec<PathBuf>> {
            Ok(vec![PathBuf::from(format!("android/{}", collection))])
         },
      );

      PathResolver::new_for_test(
         os,
         resolve_android,
         resolve_android_path_collection,
         |path: &IosPath| -> Result<PathBuf> { Ok(PathBuf::from(format!("ios/{}", path))) },
         |path: &LinuxPath| -> Result<PathBuf> { Ok(PathBuf::from(format!("linux/{}", path))) },
         |path: &MacPath| -> Result<PathBuf> { Ok(PathBuf::from(format!("apple/{}", path))) },
         |path: &WindowsPath| -> Result<PathBuf> { Ok(PathBuf::from(format!("windows/{}", path))) },
      )
   }
}
