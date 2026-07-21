use crate::error::Result;
use crate::ios_paths::IosPath;
use crate::path_mapping::{
   WinPackagedPathMapping, validate_bundle_identifier, validate_relative_path,
};
use crate::{
   AndroidPathCollection, CrossPlatformMapping, Error, LinuxPath, Win32Path,
   WindowsApplicationDataPath,
};
use std::fmt::Display;
use std::path::PathBuf;

use crate::android_paths::AndroidPath;
use crate::fs_environment::{FsEnvironment, get_fs_environment};
use crate::mac_paths::MacPath;

type AndroidPathResolver = Box<dyn Fn(&AndroidPath) -> Result<PathBuf> + Send + Sync>;
type AndroidPathCollectionResolver =
   Box<dyn Fn(&AndroidPathCollection) -> Result<Vec<PathBuf>> + Send + Sync>;
type IosPathResolver = Box<dyn Fn(&IosPath) -> Result<PathBuf> + Send + Sync>;
type LinuxPathResolver = Box<dyn Fn(&LinuxPath) -> Result<PathBuf> + Send + Sync>;
type MacPathResolver = Box<dyn Fn(&MacPath) -> Result<PathBuf> + Send + Sync>;
type Win32PathResolver = Box<dyn Fn(&Win32Path) -> Result<PathBuf> + Send + Sync>;
type WindowsApplicationDataPathResolver =
   Box<dyn Fn(&WindowsApplicationDataPath) -> Result<PathBuf> + Send + Sync>;

pub struct PathResolver {
   environment: FsEnvironment,
   resolve_android: AndroidPathResolver,
   resolve_android_path_collection: AndroidPathCollectionResolver,
   resolve_ios: IosPathResolver,
   resolve_linux: LinuxPathResolver,
   resolve_mac: MacPathResolver,
   resolve_win32: Win32PathResolver,
   resolve_windows_application_data: WindowsApplicationDataPathResolver,
}

impl PathResolver {
   pub fn new(bundle_identifier: String) -> Result<Self> {
      validate_bundle_identifier(&bundle_identifier)?;
      let environment = get_fs_environment()?;
      let ios_bundle_identifier = bundle_identifier.clone();
      let linux_bundle_identifier = bundle_identifier.clone();
      let mac_bundle_identifier = bundle_identifier.clone();
      let win32_bundle_identifier = bundle_identifier;
      Ok(Self {
         environment,
         resolve_android: Box::new(|_| Err(Error::AndroidPathResolutionNotConfigured)),
         resolve_android_path_collection: Box::new(|_| {
            Err(Error::AndroidPathResolutionNotConfigured)
         }),
         resolve_ios: Box::new(move |path: &IosPath| -> Result<PathBuf> {
            crate::ios_resolve::resolve_ios_path(path, &ios_bundle_identifier)
         }),
         resolve_linux: Box::new(move |path: &LinuxPath| -> Result<PathBuf> {
            crate::linux_resolve::resolve_linux_path(path, &linux_bundle_identifier)
         }),
         resolve_mac: Box::new(move |path: &MacPath| -> Result<PathBuf> {
            crate::mac_resolve::resolve_mac_path(path, &mac_bundle_identifier)
         }),
         resolve_win32: Box::new(move |path: &Win32Path| -> Result<PathBuf> {
            crate::windows_resolve::resolve_win32_path(path, &win32_bundle_identifier)
         }),
         resolve_windows_application_data: Box::new(
            move |path: &WindowsApplicationDataPath| -> Result<PathBuf> {
               crate::windows_resolve::resolve_win_packaged_path(path)
            },
         ),
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

   #[allow(clippy::too_many_arguments)]
   #[cfg(any(test, feature = "test-helpers"))]
   pub fn new_for_test(
      environment: FsEnvironment,
      resolve_android: AndroidPathResolver,
      resolve_android_path_collection: AndroidPathCollectionResolver,
      resolve_ios: IosPathResolver,
      resolve_linux: LinuxPathResolver,
      resolve_mac: MacPathResolver,
      resolve_win32: Win32PathResolver,
      resolve_windows_application_data: WindowsApplicationDataPathResolver,
   ) -> Self {
      Self {
         environment,
         resolve_android,
         resolve_android_path_collection,
         resolve_ios,
         resolve_linux,
         resolve_mac,
         resolve_win32,
         resolve_windows_application_data,
      }
   }

   pub fn resolve_mapping(&self, mapping: &CrossPlatformMapping) -> Result<PathBuf> {
      match self.environment {
         FsEnvironment::Android => {
            if let Some(platform_mapping) = &mapping.android {
               return self.resolve_with_relative_path(
                  || self.resolve_android(&platform_mapping.platform_path),
                  &platform_mapping.relative_path,
               );
            }

            Err(Error::PathMappingUndefined(
               FsEnvironment::Android.to_string(),
            ))
         }
         FsEnvironment::Ios => {
            if let Some(platform_mapping) = &mapping.ios {
               return self.resolve_with_relative_path(
                  || self.resolve_ios(&platform_mapping.platform_path),
                  &platform_mapping.relative_path,
               );
            }

            Err(Error::PathMappingUndefined(FsEnvironment::Ios.to_string()))
         }
         FsEnvironment::Linux => {
            if let Some(platform_mapping) = &mapping.linux {
               return self.resolve_with_relative_path(
                  || self.resolve_linux(&platform_mapping.platform_path),
                  &platform_mapping.relative_path,
               );
            }

            Err(Error::PathMappingUndefined(
               FsEnvironment::Linux.to_string(),
            ))
         }
         FsEnvironment::Macos => {
            if let Some(platform_mapping) = &mapping.macos {
               return self.resolve_with_relative_path(
                  || self.resolve_mac(&platform_mapping.platform_path),
                  &platform_mapping.relative_path,
               );
            }

            Err(Error::PathMappingUndefined(
               FsEnvironment::Macos.to_string(),
            ))
         }
         FsEnvironment::Win32 => {
            if let Some(platform_mapping) = &mapping.win32 {
               return self.resolve_with_relative_path(
                  || self.resolve_win32(&platform_mapping.platform_path),
                  &platform_mapping.relative_path,
               );
            }

            Err(Error::PathMappingUndefined(
               FsEnvironment::Win32.to_string(),
            ))
         }
         FsEnvironment::WinPackaged => {
            if let Some(platform_mapping) = &mapping.win_packaged {
               match platform_mapping {
                  WinPackagedPathMapping::WindowsApplicationDataPath(app_data_path) => {
                     return self.resolve_with_relative_path(
                        || self.resolve_windows_application_data(&app_data_path.platform_path),
                        &app_data_path.relative_path,
                     );
                  }
                  WinPackagedPathMapping::Win32Path(win32_path) => {
                     return self.resolve_with_relative_path(
                        || self.resolve_win32(&win32_path.platform_path),
                        &win32_path.relative_path,
                     );
                  }
               }
            }

            Err(Error::WinPackagedPathMappingUndefined)
         }
      }
   }

   pub fn environment(&self) -> &FsEnvironment {
      &self.environment
   }

   pub fn resolve_android(&self, path: &AndroidPath) -> Result<PathBuf> {
      Self::check_environment(path, &[&FsEnvironment::Android], &self.environment)?;
      (self.resolve_android)(path)
   }

   pub fn resolve_android_path_collection(
      &self,
      collection: &AndroidPathCollection,
   ) -> Result<Vec<PathBuf>> {
      Self::check_environment(collection, &[&FsEnvironment::Android], &self.environment)?;
      (self.resolve_android_path_collection)(collection)
   }

   pub fn resolve_ios(&self, path: &IosPath) -> Result<PathBuf> {
      Self::check_environment(path, &[&FsEnvironment::Ios], &self.environment)?;
      (self.resolve_ios)(path)
   }

   pub fn resolve_linux(&self, path: &LinuxPath) -> Result<PathBuf> {
      Self::check_environment(path, &[&FsEnvironment::Linux], &self.environment)?;
      (self.resolve_linux)(path)
   }

   pub fn resolve_mac(&self, path: &MacPath) -> Result<PathBuf> {
      Self::check_environment(path, &[&FsEnvironment::Macos], &self.environment)?;
      (self.resolve_mac)(path)
   }

   pub fn resolve_win32(&self, path: &Win32Path) -> Result<PathBuf> {
      // This check is intentional; Win32 paths can be resolved from both Win32 and packaged environments
      Self::check_environment(
         path,
         &[&FsEnvironment::Win32, &FsEnvironment::WinPackaged],
         &self.environment,
      )?;
      (self.resolve_win32)(path)
   }

   pub fn resolve_windows_application_data(
      &self,
      path: &WindowsApplicationDataPath,
   ) -> Result<PathBuf> {
      Self::check_environment(path, &[&FsEnvironment::WinPackaged], &self.environment)?;
      (self.resolve_windows_application_data)(path)
   }

   fn check_environment(
      path_string: &dyn Display,
      expected: &[&FsEnvironment],
      actual: &FsEnvironment,
   ) -> Result<()> {
      if !expected.contains(&actual) {
         return Err(Error::IncorrectOS {
            path: path_string.to_string(),
            current_os: actual.to_string(),
            expected_os: expected
               .iter()
               .map(|e| e.to_string())
               .collect::<Vec<String>>()
               .join(", "),
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

   const ENVIRONMENTS: [FsEnvironment; 6] = [
      FsEnvironment::Android,
      FsEnvironment::Ios,
      FsEnvironment::Linux,
      FsEnvironment::Macos,
      FsEnvironment::Win32,
      FsEnvironment::WinPackaged,
   ];

   #[test]
   fn new_rejects_invalid_bundle_identifier() {
      assert!(PathResolver::new(String::new()).is_err());
      assert!(PathResolver::new("../escape".to_string()).is_err());
   }

   #[test]
   fn android_environment_rejects_other_environment_paths() {
      let android_environment = FsEnvironment::Android;

      let android_resolver = create_test_resolver(FsEnvironment::Android);

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
            current_os: android_environment.to_string(),
            expected_os: "ios".to_string(),
         }
      );

      let linux_result = android_resolver.resolve_linux(&LinuxPath::DataHome);
      assert_eq!(
         linux_result.unwrap_err(),
         Error::IncorrectOS {
            path: LinuxPath::DataHome.to_string(),
            current_os: android_environment.to_string(),
            expected_os: "linux".to_string(),
         }
      );

      let mac_result = android_resolver.resolve_mac(&MacPath::ApplicationDirectory);
      assert_eq!(
         mac_result.unwrap_err(),
         Error::IncorrectOS {
            path: MacPath::ApplicationDirectory.to_string(),
            current_os: android_environment.to_string(),
            expected_os: "macos".to_string(),
         }
      );

      let win32_result = android_resolver.resolve_win32(&Win32Path::LocalAppData);
      assert_eq!(
         win32_result.unwrap_err(),
         Error::IncorrectOS {
            path: Win32Path::LocalAppData.to_string(),
            current_os: android_environment.to_string(),
            expected_os: ["win32", "winpackaged"].join(", "),
         }
      );

      let win_packaged_result = android_resolver
         .resolve_windows_application_data(&WindowsApplicationDataPath::LocalFolder);
      assert_eq!(
         win_packaged_result.unwrap_err(),
         Error::IncorrectOS {
            path: WindowsApplicationDataPath::LocalFolder.to_string(),
            current_os: android_environment.to_string(),
            expected_os: "winpackaged".to_string(),
         }
      );
   }

   #[test]
   fn ios_environment_rejects_other_environment_paths() {
      let ios_environment = FsEnvironment::Ios;
      let ios_resolver = create_test_resolver(FsEnvironment::Ios);

      let android_result = ios_resolver.resolve_android(&AndroidPath::DataDir);
      assert_eq!(
         android_result.unwrap_err(),
         Error::IncorrectOS {
            path: AndroidPath::DataDir.to_string(),
            current_os: ios_environment.to_string(),
            expected_os: "android".to_string(),
         }
      );

      let android_collection_result =
         ios_resolver.resolve_android_path_collection(&AndroidPathCollection::ExternalCacheDirs);
      assert_eq!(
         android_collection_result.unwrap_err(),
         Error::IncorrectOS {
            path: AndroidPathCollection::ExternalCacheDirs.to_string(),
            current_os: ios_environment.to_string(),
            expected_os: "android".to_string(),
         }
      );

      let ios_result = ios_resolver.resolve_ios(&IosPath::DocumentDirectory);
      assert_eq!(ios_result.unwrap(), PathBuf::from("ios/documentDirectory"));

      let linux_result = ios_resolver.resolve_linux(&LinuxPath::DataHome);
      assert_eq!(
         linux_result.unwrap_err(),
         Error::IncorrectOS {
            path: LinuxPath::DataHome.to_string(),
            current_os: ios_environment.to_string(),
            expected_os: "linux".to_string(),
         }
      );

      let mac_result = ios_resolver.resolve_mac(&MacPath::ApplicationDirectory);
      assert_eq!(
         mac_result.unwrap_err(),
         Error::IncorrectOS {
            path: MacPath::ApplicationDirectory.to_string(),
            current_os: ios_environment.to_string(),
            expected_os: "macos".to_string(),
         }
      );

      let win32_result = ios_resolver.resolve_win32(&Win32Path::LocalAppData);
      assert_eq!(
         win32_result.unwrap_err(),
         Error::IncorrectOS {
            path: Win32Path::LocalAppData.to_string(),
            current_os: ios_environment.to_string(),
            expected_os: ["win32", "winpackaged"].join(", "),
         }
      );

      let win_packaged_result =
         ios_resolver.resolve_windows_application_data(&WindowsApplicationDataPath::LocalFolder);
      assert_eq!(
         win_packaged_result.unwrap_err(),
         Error::IncorrectOS {
            path: WindowsApplicationDataPath::LocalFolder.to_string(),
            current_os: ios_environment.to_string(),
            expected_os: "winpackaged".to_string(),
         }
      );
   }

   #[test]
   fn linux_environment_rejects_other_environment_paths() {
      let linux_environment = FsEnvironment::Linux;
      let linux_resolver = create_test_resolver(FsEnvironment::Linux);

      let android_result = linux_resolver.resolve_android(&AndroidPath::DataDir);
      assert_eq!(
         android_result.unwrap_err(),
         Error::IncorrectOS {
            path: AndroidPath::DataDir.to_string(),
            current_os: linux_environment.to_string(),
            expected_os: "android".to_string(),
         }
      );

      let android_collection_result =
         linux_resolver.resolve_android_path_collection(&AndroidPathCollection::ExternalCacheDirs);
      assert_eq!(
         android_collection_result.unwrap_err(),
         Error::IncorrectOS {
            path: AndroidPathCollection::ExternalCacheDirs.to_string(),
            current_os: linux_environment.to_string(),
            expected_os: "android".to_string(),
         }
      );

      let ios_result = linux_resolver.resolve_ios(&IosPath::DocumentDirectory);
      assert_eq!(
         ios_result.unwrap_err(),
         Error::IncorrectOS {
            path: IosPath::DocumentDirectory.to_string(),
            current_os: linux_environment.to_string(),
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
            current_os: linux_environment.to_string(),
            expected_os: "macos".to_string(),
         }
      );

      let win32_result = linux_resolver.resolve_win32(&Win32Path::LocalAppData);
      assert_eq!(
         win32_result.unwrap_err(),
         Error::IncorrectOS {
            path: Win32Path::LocalAppData.to_string(),
            current_os: linux_environment.to_string(),
            expected_os: ["win32", "winpackaged"].join(", "),
         }
      );

      let win_packaged_result =
         linux_resolver.resolve_windows_application_data(&WindowsApplicationDataPath::LocalFolder);
      assert_eq!(
         win_packaged_result.unwrap_err(),
         Error::IncorrectOS {
            path: WindowsApplicationDataPath::LocalFolder.to_string(),
            current_os: linux_environment.to_string(),
            expected_os: "winpackaged".to_string(),
         }
      );
   }

   #[test]
   fn macos_environment_rejects_other_environment_paths() {
      let macos_environment = FsEnvironment::Macos;
      let macos_resolver = create_test_resolver(FsEnvironment::Macos);

      let android_result = macos_resolver.resolve_android(&AndroidPath::DataDir);
      assert_eq!(
         android_result.unwrap_err(),
         Error::IncorrectOS {
            path: AndroidPath::DataDir.to_string(),
            current_os: macos_environment.to_string(),
            expected_os: "android".to_string(),
         }
      );

      let android_collection_result =
         macos_resolver.resolve_android_path_collection(&AndroidPathCollection::ExternalCacheDirs);
      assert_eq!(
         android_collection_result.unwrap_err(),
         Error::IncorrectOS {
            path: AndroidPathCollection::ExternalCacheDirs.to_string(),
            current_os: macos_environment.to_string(),
            expected_os: "android".to_string(),
         }
      );

      let ios_result = macos_resolver.resolve_ios(&IosPath::DocumentDirectory);
      assert_eq!(
         ios_result.unwrap_err(),
         Error::IncorrectOS {
            path: IosPath::DocumentDirectory.to_string(),
            current_os: macos_environment.to_string(),
            expected_os: "ios".to_string(),
         }
      );

      let linux_result = macos_resolver.resolve_linux(&LinuxPath::DataHome);
      assert_eq!(
         linux_result.unwrap_err(),
         Error::IncorrectOS {
            path: LinuxPath::DataHome.to_string(),
            current_os: macos_environment.to_string(),
            expected_os: "linux".to_string(),
         }
      );

      let mac_result = macos_resolver.resolve_mac(&MacPath::ApplicationDirectory);
      assert_eq!(
         mac_result.unwrap(),
         PathBuf::from("apple/applicationDirectory")
      );

      let win32_result = macos_resolver.resolve_win32(&Win32Path::LocalAppData);
      assert_eq!(
         win32_result.unwrap_err(),
         Error::IncorrectOS {
            path: Win32Path::LocalAppData.to_string(),
            current_os: macos_environment.to_string(),
            expected_os: ["win32", "winpackaged"].join(", "),
         }
      );

      let win_packaged_result =
         macos_resolver.resolve_windows_application_data(&WindowsApplicationDataPath::LocalFolder);
      assert_eq!(
         win_packaged_result.unwrap_err(),
         Error::IncorrectOS {
            path: WindowsApplicationDataPath::LocalFolder.to_string(),
            current_os: macos_environment.to_string(),
            expected_os: "winpackaged".to_string(),
         }
      );
   }

   #[test]
   fn win32_environment_rejects_other_environment_paths() {
      let win32_environment = FsEnvironment::Win32;
      let win32_resolver = create_test_resolver(FsEnvironment::Win32);

      let android_result = win32_resolver.resolve_android(&AndroidPath::DataDir);
      assert_eq!(
         android_result.unwrap_err(),
         Error::IncorrectOS {
            path: AndroidPath::DataDir.to_string(),
            current_os: win32_environment.to_string(),
            expected_os: "android".to_string(),
         }
      );

      let android_collection_result =
         win32_resolver.resolve_android_path_collection(&AndroidPathCollection::ExternalCacheDirs);
      assert_eq!(
         android_collection_result.unwrap_err(),
         Error::IncorrectOS {
            path: AndroidPathCollection::ExternalCacheDirs.to_string(),
            current_os: win32_environment.to_string(),
            expected_os: "android".to_string(),
         }
      );

      let ios_result = win32_resolver.resolve_ios(&IosPath::DocumentDirectory);
      assert_eq!(
         ios_result.unwrap_err(),
         Error::IncorrectOS {
            path: IosPath::DocumentDirectory.to_string(),
            current_os: win32_environment.to_string(),
            expected_os: "ios".to_string(),
         }
      );

      let linux_result = win32_resolver.resolve_linux(&LinuxPath::DataHome);
      assert_eq!(
         linux_result.unwrap_err(),
         Error::IncorrectOS {
            path: LinuxPath::DataHome.to_string(),
            current_os: win32_environment.to_string(),
            expected_os: "linux".to_string(),
         }
      );

      let mac_result = win32_resolver.resolve_mac(&MacPath::ApplicationDirectory);
      assert_eq!(
         mac_result.unwrap_err(),
         Error::IncorrectOS {
            path: MacPath::ApplicationDirectory.to_string(),
            current_os: win32_environment.to_string(),
            expected_os: "macos".to_string(),
         }
      );

      let win32_result = win32_resolver.resolve_win32(&Win32Path::LocalAppData);
      assert_eq!(win32_result.unwrap(), PathBuf::from("win32/localAppData"));

      let win_packaged_result =
         win32_resolver.resolve_windows_application_data(&WindowsApplicationDataPath::LocalFolder);
      assert_eq!(
         win_packaged_result.unwrap_err(),
         Error::IncorrectOS {
            path: WindowsApplicationDataPath::LocalFolder.to_string(),
            current_os: win32_environment.to_string(),
            expected_os: "winpackaged".to_string(),
         }
      );
   }

   #[test]
   fn win_packaged_environment_rejects_other_environment_paths() {
      let win_packaged_environment = FsEnvironment::WinPackaged;
      let win_packaged_resolver = create_test_resolver(FsEnvironment::WinPackaged);

      let android_result = win_packaged_resolver.resolve_android(&AndroidPath::DataDir);
      assert_eq!(
         android_result.unwrap_err(),
         Error::IncorrectOS {
            path: AndroidPath::DataDir.to_string(),
            current_os: win_packaged_environment.to_string(),
            expected_os: "android".to_string(),
         }
      );

      let android_collection_result = win_packaged_resolver
         .resolve_android_path_collection(&AndroidPathCollection::ExternalCacheDirs);
      assert_eq!(
         android_collection_result.unwrap_err(),
         Error::IncorrectOS {
            path: AndroidPathCollection::ExternalCacheDirs.to_string(),
            current_os: win_packaged_environment.to_string(),
            expected_os: "android".to_string(),
         }
      );

      let ios_result = win_packaged_resolver.resolve_ios(&IosPath::DocumentDirectory);
      assert_eq!(
         ios_result.unwrap_err(),
         Error::IncorrectOS {
            path: IosPath::DocumentDirectory.to_string(),
            current_os: win_packaged_environment.to_string(),
            expected_os: "ios".to_string(),
         }
      );

      let linux_result = win_packaged_resolver.resolve_linux(&LinuxPath::DataHome);
      assert_eq!(
         linux_result.unwrap_err(),
         Error::IncorrectOS {
            path: LinuxPath::DataHome.to_string(),
            current_os: win_packaged_environment.to_string(),
            expected_os: "linux".to_string(),
         }
      );

      let mac_result = win_packaged_resolver.resolve_mac(&MacPath::ApplicationDirectory);
      assert_eq!(
         mac_result.unwrap_err(),
         Error::IncorrectOS {
            path: MacPath::ApplicationDirectory.to_string(),
            current_os: win_packaged_environment.to_string(),
            expected_os: "macos".to_string(),
         }
      );

      // Both Win32 and WinPackaged FsEnvironments can resolve Win32 paths.
      let win32_result = win_packaged_resolver.resolve_win32(&Win32Path::LocalAppData);
      assert_eq!(win32_result.unwrap(), PathBuf::from("win32/localAppData"));

      let win_packaged_result = win_packaged_resolver
         .resolve_windows_application_data(&WindowsApplicationDataPath::LocalFolder);

      assert_eq!(
         win_packaged_result.unwrap(),
         PathBuf::from("windowsApplicationData/localFolder")
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
         win32: Some(PlatformMapping {
            platform_path: Win32Path::LocalAppData,
            relative_path: None,
         }),
         win_packaged: Some(WinPackagedPathMapping::WindowsApplicationDataPath(
            PlatformMapping {
               platform_path: WindowsApplicationDataPath::LocalFolder,
               relative_path: None,
            },
         )),
      };

      for environment in ENVIRONMENTS {
         let resolver = create_test_resolver(environment.clone());
         let resolved = resolver.resolve_mapping(&path_mapping).unwrap();

         match environment {
            FsEnvironment::Android => {
               assert_eq!(resolved, PathBuf::from("android/dataDir"),);
            }
            FsEnvironment::Ios => {
               assert_eq!(resolved, PathBuf::from("ios/documentDirectory"),);
            }
            FsEnvironment::Linux => {
               assert_eq!(resolved, PathBuf::from("linux/dataHome"),);
            }
            FsEnvironment::Macos => {
               assert_eq!(resolved, PathBuf::from("apple/applicationDirectory"),);
            }
            FsEnvironment::Win32 => {
               assert_eq!(resolved, PathBuf::from("win32/localAppData"),);
            }
            FsEnvironment::WinPackaged => {
               assert_eq!(
                  resolved,
                  PathBuf::from("windowsApplicationData/localFolder"),
               );
            }
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
         win32: None,
         win_packaged: None,
      };

      for environment in ENVIRONMENTS {
         let resolver = create_test_resolver(environment.clone());
         let error_str = resolver
            .resolve_mapping(&path_mapping)
            .unwrap_err()
            .to_string();

         match environment {
            FsEnvironment::Android => {
               assert_eq!(
                  error_str,
                  Error::PathMappingUndefined(FsEnvironment::Android.to_string()).to_string()
               );
            }
            FsEnvironment::Ios => {
               assert_eq!(
                  error_str,
                  Error::PathMappingUndefined(FsEnvironment::Ios.to_string()).to_string()
               );
            }
            FsEnvironment::Linux => {
               assert_eq!(
                  error_str,
                  Error::PathMappingUndefined(FsEnvironment::Linux.to_string()).to_string()
               );
            }
            FsEnvironment::Macos => {
               assert_eq!(
                  error_str,
                  Error::PathMappingUndefined(FsEnvironment::Macos.to_string()).to_string()
               );
            }
            FsEnvironment::Win32 => {
               assert_eq!(
                  error_str,
                  Error::PathMappingUndefined(FsEnvironment::Win32.to_string()).to_string()
               );
            }
            FsEnvironment::WinPackaged => {
               assert_eq!(
                  error_str,
                  Error::WinPackagedPathMappingUndefined.to_string()
               );
            }
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
         win32: Some(PlatformMapping {
            platform_path: Win32Path::LocalAppData,
            relative_path: Some("the/win32/path".to_string()),
         }),
         win_packaged: Some(WinPackagedPathMapping::WindowsApplicationDataPath(
            PlatformMapping {
               platform_path: WindowsApplicationDataPath::LocalFolder,
               relative_path: Some("the/windowsApplicationData/path".to_string()),
            },
         )),
      };

      for environment in ENVIRONMENTS {
         let resolver = create_test_resolver(environment.clone());
         let resolved = resolver.resolve_mapping(&path_mapping).unwrap();

         match environment {
            FsEnvironment::Android => {
               assert_eq!(resolved, PathBuf::from("android/dataDir/the/android/path"),);
            }
            FsEnvironment::Ios => {
               assert_eq!(
                  resolved,
                  PathBuf::from("ios/documentDirectory/the/ios/path"),
               );
            }
            FsEnvironment::Linux => {
               assert_eq!(resolved, PathBuf::from("linux/dataHome/the/linux/path"),);
            }
            FsEnvironment::Macos => {
               assert_eq!(
                  resolved,
                  PathBuf::from("apple/applicationDirectory/the/mac/path"),
               );
            }
            FsEnvironment::Win32 => {
               assert_eq!(resolved, PathBuf::from("win32/localAppData/the/win32/path"),);
            }
            FsEnvironment::WinPackaged => {
               assert_eq!(
                  resolved,
                  PathBuf::from(
                     "windowsApplicationData/localFolder/the/windowsApplicationData/path"
                  ),
               );
            }
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
         win32: Some(PlatformMapping {
            platform_path: Win32Path::LocalAppData,
            relative_path: Some("../escape".to_string()),
         }),
         win_packaged: Some(WinPackagedPathMapping::WindowsApplicationDataPath(
            PlatformMapping {
               platform_path: WindowsApplicationDataPath::LocalFolder,
               relative_path: Some("../escape".to_string()),
            },
         )),
      };

      for environment in ENVIRONMENTS {
         let resolver = create_test_resolver(environment);
         let error = resolver.resolve_mapping(&path_mapping).unwrap_err();
         assert_eq!(
            error,
            Error::InvalidPath(
               "Relative path must contain only normal path segments: ../escape".to_string()
            )
         );
      }
   }

   #[test]
   fn resolve_path_mapping_with_both_application_data_path_and_win32_path_for_win_packaged_returns_correct_path()
    {
      let resolver = create_test_resolver(FsEnvironment::WinPackaged);

      let mapping_to_app_data = CrossPlatformMapping {
         android: None,
         ios: None,
         linux: None,
         macos: None,
         win32: None,
         win_packaged: Some(WinPackagedPathMapping::WindowsApplicationDataPath(
            PlatformMapping {
               platform_path: WindowsApplicationDataPath::LocalFolder,
               relative_path: None,
            },
         )),
      };

      let app_data_resolved = resolver.resolve_mapping(&mapping_to_app_data).unwrap();
      assert_eq!(
         app_data_resolved,
         PathBuf::from("windowsApplicationData/localFolder")
      );

      let mapping_to_win32 = CrossPlatformMapping {
         android: None,
         ios: None,
         linux: None,
         macos: None,
         win32: None,
         win_packaged: Some(WinPackagedPathMapping::Win32Path(PlatformMapping {
            platform_path: Win32Path::LocalAppData,
            relative_path: None,
         })),
      };

      let win32_resolved = resolver.resolve_mapping(&mapping_to_win32).unwrap();
      assert_eq!(win32_resolved, PathBuf::from("win32/localAppData"));
   }

   fn create_test_resolver(environment: FsEnvironment) -> PathResolver {
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

      let resolve_win32 = Box::new(|path: &Win32Path| -> Result<PathBuf> {
         Ok(PathBuf::from(format!("win32/{}", path)))
      });

      let resolve_windows_application_data =
         Box::new(|path: &WindowsApplicationDataPath| -> Result<PathBuf> {
            Ok(PathBuf::from(format!("windowsApplicationData/{}", path)))
         });

      PathResolver::new_for_test(
         environment,
         resolve_android,
         resolve_android_path_collection,
         resolve_ios,
         resolve_linux,
         resolve_mac,
         resolve_win32,
         resolve_windows_application_data,
      )
   }
}
