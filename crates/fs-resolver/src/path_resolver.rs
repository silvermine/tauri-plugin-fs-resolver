use crate::error::Result;
use crate::ios_paths::IosPath;
use crate::path_mapping::{validate_bundle_identifier, validate_relative_path};
use crate::{AndroidPathCollection, CrossPlatformMapping, Error, LinuxPath};
use std::fmt::Display;
use std::path::PathBuf;

use crate::android_paths::AndroidPath;
use crate::mac_paths::MacPath;
use crate::windows_paths::WindowsPath;

type AndroidPathResolver = Box<dyn Fn(&AndroidPath) -> Result<PathBuf> + Send + Sync>;
type AndroidPathCollectionResolver =
   Box<dyn Fn(&AndroidPathCollection) -> Result<Vec<PathBuf>> + Send + Sync>;
type IosPathResolver = Box<dyn Fn(&IosPath) -> Result<PathBuf> + Send + Sync>;
type LinuxPathResolver = Box<dyn Fn(&LinuxPath) -> Result<PathBuf> + Send + Sync>;
type MacPathResolver = Box<dyn Fn(&MacPath) -> Result<PathBuf> + Send + Sync>;
type WindowsPathResolver = Box<dyn Fn(&WindowsPath) -> Result<PathBuf> + Send + Sync>;

pub struct PathResolver {
   os: String,
   resolve_android: AndroidPathResolver,
   resolve_android_path_collection: AndroidPathCollectionResolver,
   resolve_ios: IosPathResolver,
   resolve_linux: LinuxPathResolver,
   resolve_mac: MacPathResolver,
   resolve_windows: WindowsPathResolver,
}

impl PathResolver {
   pub fn new(bundle_identifier: String) -> Result<Self> {
      validate_bundle_identifier(&bundle_identifier)?;
      let linux_bundle_identifier = bundle_identifier.clone();
      let mac_bundle_identifier = bundle_identifier.clone();
      let win32_bundle_identifier = bundle_identifier;
      Ok(Self {
         os: std::env::consts::OS.to_string(),
         resolve_android: Box::new(|_| Err(Error::AndroidPathResolutionNotConfigured)),
         resolve_android_path_collection: Box::new(|_| {
            Err(Error::AndroidPathResolutionNotConfigured)
         }),
         resolve_ios: Box::new(|path: &IosPath| -> Result<PathBuf> {
            crate::ios_resolve::resolve_ios_path(path)
         }),
         resolve_linux: Box::new(move |path: &LinuxPath| -> Result<PathBuf> {
            crate::linux_resolve::resolve_linux_path(path, &linux_bundle_identifier)
         }),
         resolve_mac: Box::new(move |path: &MacPath| -> Result<PathBuf> {
            crate::mac_resolve::resolve_mac_path(path, &mac_bundle_identifier)
         }),
         resolve_windows: Box::new(move |path: &WindowsPath| -> Result<PathBuf> {
            match path {
               WindowsPath::Win32(path) => {
                  crate::windows_resolve::resolve_win32_path(path, &win32_bundle_identifier)
               }
               WindowsPath::WinMsix(path) => crate::windows_resolve::resolve_win_msix_path(path),
            }
         }),
      })
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

   #[cfg(any(test, feature = "test-helpers"))]
   pub fn new_for_test(
      os: String,
      resolve_android: AndroidPathResolver,
      resolve_android_path_collection: AndroidPathCollectionResolver,
      resolve_ios: IosPathResolver,
      resolve_linux: LinuxPathResolver,
      resolve_mac: MacPathResolver,
      resolve_windows: WindowsPathResolver,
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

   pub fn resolve_mapping(&self, mapping: &CrossPlatformMapping) -> Result<PathBuf> {
      match self.os.as_str() {
         "android" => {
            if let Some(platform_mapping) = &mapping.android {
               return self.resolve_with_relative_path(
                  || self.resolve_android(&platform_mapping.platform_path),
                  &platform_mapping.relative_path,
               );
            }

            Err(Error::PathMappingUndefined("android".to_string()))
         }
         "ios" => {
            if let Some(platform_mapping) = &mapping.ios {
               return self.resolve_with_relative_path(
                  || self.resolve_ios(&platform_mapping.platform_path),
                  &platform_mapping.relative_path,
               );
            }

            Err(Error::PathMappingUndefined("ios".to_string()))
         }
         "linux" => {
            if let Some(platform_mapping) = &mapping.linux {
               return self.resolve_with_relative_path(
                  || self.resolve_linux(&platform_mapping.platform_path),
                  &platform_mapping.relative_path,
               );
            }

            Err(Error::PathMappingUndefined("linux".to_string()))
         }
         "macos" => {
            if let Some(platform_mapping) = &mapping.macos {
               return self.resolve_with_relative_path(
                  || self.resolve_mac(&platform_mapping.platform_path),
                  &platform_mapping.relative_path,
               );
            }

            Err(Error::PathMappingUndefined("macos".to_string()))
         }
         "windows" => {
            if let Some(platform_mapping) = &mapping.windows {
               return self.resolve_with_relative_path(
                  || self.resolve_windows(&platform_mapping.platform_path),
                  &platform_mapping.relative_path,
               );
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

   fn resolve_with_relative_path<F>(
      &self,
      path_resolver: F,
      relative_path: &Option<String>,
   ) -> Result<PathBuf>
   where
      F: FnOnce() -> Result<PathBuf>,
   {
      let resolved_path = path_resolver()?;
      if let Some(relative_path) = relative_path {
         validate_relative_path(relative_path)?;
         Ok(resolved_path.join(relative_path))
      } else {
         Ok(resolved_path)
      }
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   use crate::Error;
   use crate::PlatformMapping;
   use crate::Win32Path;

   const KNOWN_OS: [&str; 5] = ["ios", "macos", "android", "windows", "linux"];

   #[test]
   fn new_rejects_invalid_bundle_identifier() {
      assert!(PathResolver::new(String::new()).is_err());
      assert!(PathResolver::new("../escape".to_string()).is_err());
   }

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

      let linux_result = android_resolver.resolve_linux(&LinuxPath::DataHome);
      assert_eq!(
         linux_result.unwrap_err(),
         Error::IncorrectOS {
            path: LinuxPath::DataHome.to_string(),
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

      let linux_result = ios_resolver.resolve_linux(&LinuxPath::DataHome);
      assert_eq!(
         linux_result.unwrap_err(),
         Error::IncorrectOS {
            path: LinuxPath::DataHome.to_string(),
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

      let linux_result = linux_resolver.resolve_linux(&LinuxPath::DataHome);
      assert_eq!(linux_result.unwrap(), PathBuf::from("linux/dataHome"));

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

      let linux_result = macos_resolver.resolve_linux(&LinuxPath::DataHome);
      assert_eq!(
         linux_result.unwrap_err(),
         Error::IncorrectOS {
            path: LinuxPath::DataHome.to_string(),
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

      let linux_result = windows_resolver.resolve_linux(&LinuxPath::DataHome);
      assert_eq!(
         linux_result.unwrap_err(),
         Error::IncorrectOS {
            path: LinuxPath::DataHome.to_string(),
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
      let path_mapping = CrossPlatformMapping {
         android: Some(PlatformMapping {
            platform_path: AndroidPath::DataDir,
            relative_path: None,
         }),
         ios: Some(PlatformMapping {
            platform_path: IosPath::DocumentDirectory,
            relative_path: None,
         }),
         linux: Some(PlatformMapping {
            platform_path: LinuxPath::DataHome,
            relative_path: None,
         }),
         macos: Some(PlatformMapping {
            platform_path: MacPath::ApplicationDirectory,
            relative_path: None,
         }),
         windows: Some(PlatformMapping {
            platform_path: WindowsPath::Win32(Win32Path::LocalAppData),
            relative_path: None,
         }),
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
               PathBuf::from("linux/dataHome"),
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
      let path_mapping = CrossPlatformMapping {
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

   #[test]
   fn resolve_path_mapping_with_relative_path_returns_correct_path() {
      let path_mapping = CrossPlatformMapping {
         android: Some(PlatformMapping {
            platform_path: AndroidPath::DataDir,
            relative_path: Some("the/android/path".to_string()),
         }),
         ios: Some(PlatformMapping {
            platform_path: IosPath::DocumentDirectory,
            relative_path: Some("the/ios/path".to_string()),
         }),
         linux: Some(PlatformMapping {
            platform_path: LinuxPath::DataHome,
            relative_path: Some("the/linux/path".to_string()),
         }),
         macos: Some(PlatformMapping {
            platform_path: MacPath::ApplicationDirectory,
            relative_path: Some("the/mac/path".to_string()),
         }),
         windows: Some(PlatformMapping {
            platform_path: WindowsPath::Win32(Win32Path::LocalAppData),
            relative_path: Some("the/windows/path".to_string()),
         }),
      };

      for os in KNOWN_OS {
         let resolver = create_test_resolver(os.to_string());
         let resolved = resolver.resolve_mapping(&path_mapping).unwrap();

         if os == "android" {
            assert_eq!(
               resolved,
               PathBuf::from("android/dataDir/the/android/path"),
               "Incorrect path for android with os {}",
               os
            );
         } else if os == "ios" {
            assert_eq!(
               resolved,
               PathBuf::from("ios/documentDirectory/the/ios/path"),
               "Incorrect path for ios with os {}",
               os
            );
         } else if os == "linux" {
            assert_eq!(
               resolved,
               PathBuf::from("linux/dataHome/the/linux/path"),
               "Incorrect path for linux with os {}",
               os
            );
         } else if os == "macos" {
            assert_eq!(
               resolved,
               PathBuf::from("apple/applicationDirectory/the/mac/path"),
               "Incorrect path for macos with os {}",
               os
            );
         } else if os == "windows" {
            assert_eq!(
               resolved,
               PathBuf::from("windows/win32::localAppData/the/windows/path"),
               "Incorrect path for windows with os {}",
               os
            );
         }
      }
   }

   #[test]
   fn resolve_path_mapping_with_invalid_relative_path_returns_error() {
      let path_mapping = CrossPlatformMapping {
         android: Some(PlatformMapping {
            platform_path: AndroidPath::DataDir,
            relative_path: Some("../escape".to_string()),
         }),
         ios: None,
         linux: None,
         macos: None,
         windows: None,
      };

      let resolver = create_test_resolver("android".to_string());
      let error = resolver.resolve_mapping(&path_mapping).unwrap_err();

      assert_eq!(
         error,
         Error::InvalidPath(
            "Relative path must contain only normal path segments: ../escape".to_string()
         )
      );
   }

   #[test]
   fn returns_error_when_resolving_mapping_with_invalid_relative_path() {
      let path_mapping = CrossPlatformMapping {
         android: Some(PlatformMapping {
            platform_path: AndroidPath::DataDir,
            relative_path: Some("../escape".to_string()),
         }),
         ios: Some(PlatformMapping {
            platform_path: IosPath::DocumentDirectory,
            relative_path: Some("../escape".to_string()),
         }),
         linux: Some(PlatformMapping {
            platform_path: LinuxPath::DataHome,
            relative_path: Some("../escape".to_string()),
         }),
         macos: Some(PlatformMapping {
            platform_path: MacPath::ApplicationDirectory,
            relative_path: Some("../escape".to_string()),
         }),
         windows: Some(PlatformMapping {
            platform_path: WindowsPath::Win32(Win32Path::LocalAppData),
            relative_path: Some("../escape".to_string()),
         }),
      };

      for os in KNOWN_OS {
         let resolver = create_test_resolver(os.to_string());
         let error = resolver.resolve_mapping(&path_mapping).unwrap_err();
         assert_eq!(
            error,
            Error::InvalidPath(
               "Relative path must contain only normal path segments: ../escape".to_string()
            )
         );
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

      let resolve_ios = Box::new(|path: &IosPath| -> Result<PathBuf> {
         Ok(PathBuf::from(format!("ios/{}", path)))
      });

      let resolve_linux = Box::new(|path: &LinuxPath| -> Result<PathBuf> {
         Ok(PathBuf::from(format!("linux/{}", path)))
      });

      let resolve_mac = Box::new(|path: &MacPath| -> Result<PathBuf> {
         Ok(PathBuf::from(format!("apple/{}", path)))
      });

      let resolve_windows = Box::new(|path: &WindowsPath| -> Result<PathBuf> {
         Ok(PathBuf::from(format!("windows/{}", path)))
      });

      PathResolver::new_for_test(
         os,
         resolve_android,
         resolve_android_path_collection,
         resolve_ios,
         resolve_linux,
         resolve_mac,
         resolve_windows,
      )
   }
}
