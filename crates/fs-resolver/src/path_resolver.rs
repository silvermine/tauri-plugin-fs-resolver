use crate::error::Result;
use crate::ios_paths::IosPath;
use crate::path_mapping::{
   WinPackagedPathMapping, validate_bundle_identifier, validate_relative_path,
};
use crate::windows_paths::WindowsLibrary;
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
type WindowsLibraryDirsResolver =
   Box<dyn Fn(&WindowsLibrary) -> Result<Vec<PathBuf>> + Send + Sync>;

pub struct PathResolver {
   environment: FsEnvironment,
   resolve_android: AndroidPathResolver,
   resolve_android_path_collection: AndroidPathCollectionResolver,
   resolve_ios: IosPathResolver,
   resolve_linux: LinuxPathResolver,
   resolve_mac: MacPathResolver,
   resolve_win32: Win32PathResolver,
   resolve_windows_application_data: WindowsApplicationDataPathResolver,
   resolve_windows_library_dirs: WindowsLibraryDirsResolver,
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
         resolve_windows_library_dirs: Box::new(
            move |path: &WindowsLibrary| -> Result<Vec<PathBuf>> {
               crate::windows_resolve::resolve_dirs_in_windows_library(path)
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
      resolve_windows_library_dirs: WindowsLibraryDirsResolver,
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
         resolve_windows_library_dirs,
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

   /// Filesystem paths of folders pinned in a Windows shell library ([`WindowsLibrary`]).
   ///
   /// Unlike [`Win32Path`] `*Library` variants (`.library-ms` definition paths via
   /// `SHGetKnownFolderPath`), this uses the Shell Library API (`IShellLibrary`) and
   /// reflects the user's configured library locations.
   ///
   /// # Returns
   ///
   /// * `Ok(vec)` — member folder paths in Explorer order (may be **empty** if the user
   ///   removed every pinned location; the library still exists).
   ///
   /// # Errors
   ///
   /// * [`Error::MissingWindowsLibrary`] — the library definition is absent (not the same
   ///   as an empty member list).
   /// * [`Error::WindowsShellLibraryResolutionFailed`] — COM or shell API failure.
   pub fn resolve_windows_library_dirs(&self, library: &WindowsLibrary) -> Result<Vec<PathBuf>> {
      Self::check_environment(
         library,
         &[&FsEnvironment::Win32, &FsEnvironment::WinPackaged],
         &self.environment,
      )?;
      (self.resolve_windows_library_dirs)(library)
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
   use crate::windows_paths::WindowsLibrary;
   use rstest::rstest;

   // Fails to compile when FsEnvironment gets a new variant, as a reminder to add a case
   // for it to each table below. Nothing checks that the tables have that case.
   const _: fn(&FsEnvironment) = |environment| match environment {
      FsEnvironment::Android
      | FsEnvironment::Ios
      | FsEnvironment::Linux
      | FsEnvironment::Macos
      | FsEnvironment::Win32
      | FsEnvironment::WinPackaged => {}
   };

   #[rstest]
   #[case::empty("")]
   #[case::leading_parent_dir("../escape")]
   fn new_rejects_invalid_bundle_identifier(#[case] bundle_identifier: &str) {
      assert_eq!(
         PathResolver::new(bundle_identifier.to_string()).err(),
         Some(Error::InvalidPath(format!(
            "Bundle identifier must be a single safe path segment: {bundle_identifier}"
         )))
      );
   }

   #[test]
   fn resolve_android_accepts_android_environment() {
      let resolver = create_test_resolver(FsEnvironment::Android);
      let result = resolver.resolve_android(&AndroidPath::DataDir);

      assert_eq!(result.unwrap(), PathBuf::from("android/dataDir"));
   }

   #[rstest]
   #[case::ios(FsEnvironment::Ios)]
   #[case::linux(FsEnvironment::Linux)]
   #[case::macos(FsEnvironment::Macos)]
   #[case::win32(FsEnvironment::Win32)]
   #[case::win_packaged(FsEnvironment::WinPackaged)]
   fn resolve_android_rejects_non_android_environments(#[case] environment: FsEnvironment) {
      let resolver = create_test_resolver(environment.clone());
      let result = resolver.resolve_android(&AndroidPath::DataDir);

      assert_eq!(
         result.unwrap_err(),
         Error::IncorrectOS {
            path: AndroidPath::DataDir.to_string(),
            current_os: environment.to_string(),
            expected_os: "android".to_string(),
         }
      );
   }

   #[test]
   fn resolve_android_path_collection_accepts_android_environment() {
      let resolver = create_test_resolver(FsEnvironment::Android);
      let result =
         resolver.resolve_android_path_collection(&AndroidPathCollection::ExternalCacheDirs);

      assert_eq!(
         result.unwrap(),
         vec![PathBuf::from("android/externalCacheDirs")]
      );
   }

   #[rstest]
   #[case::ios(FsEnvironment::Ios)]
   #[case::linux(FsEnvironment::Linux)]
   #[case::macos(FsEnvironment::Macos)]
   #[case::win32(FsEnvironment::Win32)]
   #[case::win_packaged(FsEnvironment::WinPackaged)]
   fn resolve_android_path_collection_rejects_non_android_environments(
      #[case] environment: FsEnvironment,
   ) {
      let resolver = create_test_resolver(environment.clone());
      let result =
         resolver.resolve_android_path_collection(&AndroidPathCollection::ExternalCacheDirs);

      assert_eq!(
         result.unwrap_err(),
         Error::IncorrectOS {
            path: AndroidPathCollection::ExternalCacheDirs.to_string(),
            current_os: environment.to_string(),
            expected_os: "android".to_string(),
         }
      );
   }

   #[test]
   fn resolve_ios_accepts_ios_environment() {
      let resolver = create_test_resolver(FsEnvironment::Ios);
      let result = resolver.resolve_ios(&IosPath::DocumentDirectory);

      assert_eq!(result.unwrap(), PathBuf::from("ios/documentDirectory"));
   }

   #[rstest]
   #[case::android(FsEnvironment::Android)]
   #[case::linux(FsEnvironment::Linux)]
   #[case::macos(FsEnvironment::Macos)]
   #[case::win32(FsEnvironment::Win32)]
   #[case::win_packaged(FsEnvironment::WinPackaged)]
   fn resolve_ios_rejects_non_ios_environments(#[case] environment: FsEnvironment) {
      let resolver = create_test_resolver(environment.clone());
      let result = resolver.resolve_ios(&IosPath::DocumentDirectory);

      assert_eq!(
         result.unwrap_err(),
         Error::IncorrectOS {
            path: IosPath::DocumentDirectory.to_string(),
            current_os: environment.to_string(),
            expected_os: "ios".to_string(),
         }
      );
   }

   #[test]
   fn resolve_linux_accepts_linux_environment() {
      let resolver = create_test_resolver(FsEnvironment::Linux);
      let result = resolver.resolve_linux(&LinuxPath::DataHome);

      assert_eq!(result.unwrap(), PathBuf::from("linux/dataHome"));
   }

   #[rstest]
   #[case::android(FsEnvironment::Android)]
   #[case::ios(FsEnvironment::Ios)]
   #[case::macos(FsEnvironment::Macos)]
   #[case::win32(FsEnvironment::Win32)]
   #[case::win_packaged(FsEnvironment::WinPackaged)]
   fn resolve_linux_rejects_non_linux_environments(#[case] environment: FsEnvironment) {
      let resolver = create_test_resolver(environment.clone());
      let result = resolver.resolve_linux(&LinuxPath::DataHome);

      assert_eq!(
         result.unwrap_err(),
         Error::IncorrectOS {
            path: LinuxPath::DataHome.to_string(),
            current_os: environment.to_string(),
            expected_os: "linux".to_string(),
         }
      );
   }

   #[test]
   fn resolve_mac_accepts_macos_environment() {
      let resolver = create_test_resolver(FsEnvironment::Macos);
      let result = resolver.resolve_mac(&MacPath::ApplicationDirectory);

      assert_eq!(result.unwrap(), PathBuf::from("apple/applicationDirectory"));
   }

   #[rstest]
   #[case::android(FsEnvironment::Android)]
   #[case::ios(FsEnvironment::Ios)]
   #[case::linux(FsEnvironment::Linux)]
   #[case::win32(FsEnvironment::Win32)]
   #[case::win_packaged(FsEnvironment::WinPackaged)]
   fn resolve_mac_rejects_non_macos_environments(#[case] environment: FsEnvironment) {
      let resolver = create_test_resolver(environment.clone());
      let result = resolver.resolve_mac(&MacPath::ApplicationDirectory);

      assert_eq!(
         result.unwrap_err(),
         Error::IncorrectOS {
            path: MacPath::ApplicationDirectory.to_string(),
            current_os: environment.to_string(),
            expected_os: "macos".to_string(),
         }
      );
   }

   #[rstest]
   #[case::win32(FsEnvironment::Win32)]
   #[case::win_packaged(FsEnvironment::WinPackaged)]
   fn resolve_win32_accepts_windows_environments(#[case] environment: FsEnvironment) {
      let resolver = create_test_resolver(environment);
      let result = resolver.resolve_win32(&Win32Path::LocalAppData);

      assert_eq!(result.unwrap(), PathBuf::from("win32/localAppData"));
   }

   #[rstest]
   #[case::android(FsEnvironment::Android)]
   #[case::ios(FsEnvironment::Ios)]
   #[case::linux(FsEnvironment::Linux)]
   #[case::macos(FsEnvironment::Macos)]
   fn resolve_win32_rejects_non_windows_environments(#[case] environment: FsEnvironment) {
      let resolver = create_test_resolver(environment.clone());
      let result = resolver.resolve_win32(&Win32Path::LocalAppData);

      assert_eq!(
         result.unwrap_err(),
         Error::IncorrectOS {
            path: Win32Path::LocalAppData.to_string(),
            current_os: environment.to_string(),
            expected_os: ["win32", "winpackaged"].join(", "),
         }
      );
   }

   #[test]
   fn resolve_windows_application_data_accepts_win_packaged_environment() {
      let resolver = create_test_resolver(FsEnvironment::WinPackaged);
      let result =
         resolver.resolve_windows_application_data(&WindowsApplicationDataPath::LocalFolder);

      assert_eq!(
         result.unwrap(),
         PathBuf::from("windowsApplicationData/localFolder")
      );
   }

   #[rstest]
   #[case::android(FsEnvironment::Android)]
   #[case::ios(FsEnvironment::Ios)]
   #[case::linux(FsEnvironment::Linux)]
   #[case::macos(FsEnvironment::Macos)]
   #[case::win32(FsEnvironment::Win32)]
   fn resolve_windows_application_data_rejects_non_win_packaged_environments(
      #[case] environment: FsEnvironment,
   ) {
      let resolver = create_test_resolver(environment.clone());
      let result =
         resolver.resolve_windows_application_data(&WindowsApplicationDataPath::LocalFolder);

      assert_eq!(
         result.unwrap_err(),
         Error::IncorrectOS {
            path: WindowsApplicationDataPath::LocalFolder.to_string(),
            current_os: environment.to_string(),
            expected_os: "winpackaged".to_string(),
         }
      );
   }

   #[rstest]
   #[case::win32(FsEnvironment::Win32)]
   #[case::win_packaged(FsEnvironment::WinPackaged)]
   fn resolve_windows_library_dirs_accepts_windows_environments(
      #[case] environment: FsEnvironment,
   ) {
      let resolver = create_test_resolver(environment);
      let result = resolver.resolve_windows_library_dirs(&WindowsLibrary::Documents);

      assert_eq!(result.unwrap(), vec![PathBuf::from("windowsLibraryDirs")]);
   }

   #[rstest]
   #[case::android(FsEnvironment::Android)]
   #[case::ios(FsEnvironment::Ios)]
   #[case::linux(FsEnvironment::Linux)]
   #[case::macos(FsEnvironment::Macos)]
   fn resolve_windows_library_dirs_rejects_non_windows_environments(
      #[case] environment: FsEnvironment,
   ) {
      let resolver = create_test_resolver(environment.clone());
      let result = resolver.resolve_windows_library_dirs(&WindowsLibrary::Documents);

      assert_eq!(
         result.unwrap_err(),
         Error::IncorrectOS {
            path: WindowsLibrary::Documents.to_string(),
            current_os: environment.to_string(),
            expected_os: ["win32", "winpackaged"].join(", "),
         }
      );
   }

   #[rstest]
   #[case::android(FsEnvironment::Android, "android/dataDir")]
   #[case::ios(FsEnvironment::Ios, "ios/documentDirectory")]
   #[case::linux(FsEnvironment::Linux, "linux/dataHome")]
   #[case::macos(FsEnvironment::Macos, "apple/applicationDirectory")]
   #[case::win32(FsEnvironment::Win32, "win32/localAppData")]
   #[case::win_packaged(FsEnvironment::WinPackaged, "windowsApplicationData/localFolder")]
   fn resolve_path_mapping_maps_to_correct_os(
      #[case] environment: FsEnvironment,
      #[case] expected: &str,
   ) {
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

      let resolver = create_test_resolver(environment);
      let resolved = resolver.resolve_mapping(&path_mapping).unwrap();

      assert_eq!(resolved, PathBuf::from(expected));
   }

   #[rstest]
   #[case::android(
      FsEnvironment::Android,
      Error::PathMappingUndefined(FsEnvironment::Android.to_string())
   )]
   #[case::ios(
      FsEnvironment::Ios,
      Error::PathMappingUndefined(FsEnvironment::Ios.to_string())
   )]
   #[case::linux(
      FsEnvironment::Linux,
      Error::PathMappingUndefined(FsEnvironment::Linux.to_string())
   )]
   #[case::macos(
      FsEnvironment::Macos,
      Error::PathMappingUndefined(FsEnvironment::Macos.to_string())
   )]
   #[case::win32(
      FsEnvironment::Win32,
      Error::PathMappingUndefined(FsEnvironment::Win32.to_string())
   )]
   #[case::win_packaged(FsEnvironment::WinPackaged, Error::WinPackagedPathMappingUndefined)]
   fn resolve_path_mapping_with_undefined_path_returns_error(
      #[case] environment: FsEnvironment,
      #[case] expected: Error,
   ) {
      let path_mapping = CrossPlatformMapping {
         android: None,
         ios: None,
         linux: None,
         macos: None,
         win32: None,
         win_packaged: None,
      };

      let resolver = create_test_resolver(environment);
      let error = resolver.resolve_mapping(&path_mapping).unwrap_err();

      assert_eq!(error, expected);
   }

   #[rstest]
   #[case::android(FsEnvironment::Android, "android/dataDir/the/android/path")]
   #[case::ios(FsEnvironment::Ios, "ios/documentDirectory/the/ios/path")]
   #[case::linux(FsEnvironment::Linux, "linux/dataHome/the/linux/path")]
   #[case::macos(FsEnvironment::Macos, "apple/applicationDirectory/the/mac/path")]
   #[case::win32(FsEnvironment::Win32, "win32/localAppData/the/win32/path")]
   #[case::win_packaged(
      FsEnvironment::WinPackaged,
      "windowsApplicationData/localFolder/the/windowsApplicationData/path"
   )]
   fn resolve_path_mapping_with_relative_path_returns_correct_path(
      #[case] environment: FsEnvironment,
      #[case] expected: &str,
   ) {
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

      let resolver = create_test_resolver(environment);
      let resolved = resolver.resolve_mapping(&path_mapping).unwrap();

      assert_eq!(resolved, PathBuf::from(expected));
   }

   #[rstest]
   #[case::android(FsEnvironment::Android)]
   #[case::ios(FsEnvironment::Ios)]
   #[case::linux(FsEnvironment::Linux)]
   #[case::macos(FsEnvironment::Macos)]
   #[case::win32(FsEnvironment::Win32)]
   #[case::win_packaged(FsEnvironment::WinPackaged)]
   fn resolve_path_mapping_with_invalid_relative_path_returns_error(
      #[case] environment: FsEnvironment,
   ) {
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

      let resolver = create_test_resolver(environment);
      let error = resolver.resolve_mapping(&path_mapping).unwrap_err();
      assert_eq!(
         error,
         Error::InvalidPath(
            "Relative path must contain only normal path segments: ../escape".to_string()
         )
      );
   }

   #[rstest]
   #[case::windows_application_data_path(
      WinPackagedPathMapping::WindowsApplicationDataPath(PlatformMapping {
         platform_path: WindowsApplicationDataPath::LocalFolder,
         relative_path: None,
      }),
      "windowsApplicationData/localFolder"
   )]
   #[case::win32_path(
      WinPackagedPathMapping::Win32Path(PlatformMapping {
         platform_path: Win32Path::LocalAppData,
         relative_path: None,
      }),
      "win32/localAppData"
   )]
   fn resolve_path_mapping_for_win_packaged_returns_correct_path(
      #[case] win_packaged: WinPackagedPathMapping,
      #[case] expected: &str,
   ) {
      let resolver = create_test_resolver(FsEnvironment::WinPackaged);

      let path_mapping = CrossPlatformMapping {
         android: None,
         ios: None,
         linux: None,
         macos: None,
         win32: None,
         win_packaged: Some(win_packaged),
      };

      let resolved = resolver.resolve_mapping(&path_mapping).unwrap();
      assert_eq!(resolved, PathBuf::from(expected));
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

      let resolve_windows_library_dirs =
         Box::new(|_path: &WindowsLibrary| -> Result<Vec<PathBuf>> {
            Ok(vec![PathBuf::from("windowsLibraryDirs")])
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
         resolve_windows_library_dirs,
      )
   }
}
