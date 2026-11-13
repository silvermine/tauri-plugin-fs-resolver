// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
   tauri::Builder::default()
      .plugin(tauri_plugin_os::init())
      .plugin(tauri_plugin_fs_resolver::init())
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
   use std::path::PathBuf;
   use tauri_plugin_fs_resolver::fs_resolver::{
      AndroidPath, AndroidPathCollection, FsEnvironment, IosPath, LinuxPath, MacPath, PathResolver,
      Result, Win32Path, WindowsApplicationDataPath, WindowsLibrary,
   };

   // Fails to compile when FsEnvironment gets a new variant, as a reminder to add a test
   // for it below. Nothing checks that the test exists.
   const _: fn(&FsEnvironment) = |environment| match environment {
      FsEnvironment::Android
      | FsEnvironment::Ios
      | FsEnvironment::Linux
      | FsEnvironment::Macos
      | FsEnvironment::Win32
      | FsEnvironment::WinPackaged => {}
   };

   #[test]
   fn test_path_resolver_android() {
      let resolver = create_test_resolver(FsEnvironment::Android);

      let resolved = resolver.resolve_android(&AndroidPath::DataDir);
      assert_eq!(resolved.unwrap(), PathBuf::from("android/dataDir"));
      let resolved =
         resolver.resolve_android_path_collection(&AndroidPathCollection::ExternalCacheDirs);
      assert_eq!(
         resolved.unwrap(),
         vec![PathBuf::from("android/externalCacheDirs")]
      );
   }

   #[test]
   fn test_path_resolver_ios() {
      let resolver = create_test_resolver(FsEnvironment::Ios);

      let resolved = resolver.resolve_ios(&IosPath::DocumentDirectory);
      assert_eq!(resolved.unwrap(), PathBuf::from("ios/documentDirectory"));
   }

   #[test]
   fn test_path_resolver_linux() {
      let resolver = create_test_resolver(FsEnvironment::Linux);

      let resolved = resolver.resolve_linux(&LinuxPath::DataHome);
      assert_eq!(resolved.unwrap(), PathBuf::from("linux/dataHome"));
   }

   #[test]
   fn test_path_resolver_macos() {
      let resolver = create_test_resolver(FsEnvironment::Macos);

      let resolved = resolver.resolve_mac(&MacPath::ApplicationDirectory);
      assert_eq!(
         resolved.unwrap(),
         PathBuf::from("apple/applicationDirectory")
      );
   }

   #[test]
   fn test_path_resolver_win32() {
      let resolver = create_test_resolver(FsEnvironment::Win32);

      let resolved = resolver.resolve_win32(&Win32Path::LocalAppData);
      assert_eq!(resolved.unwrap(), PathBuf::from("win32/localAppData"));
   }

   #[test]
   fn test_path_resolver_win_packaged() {
      let resolver = create_test_resolver(FsEnvironment::WinPackaged);

      let resolved =
         resolver.resolve_windows_application_data(&WindowsApplicationDataPath::LocalFolder);
      assert_eq!(
         resolved.unwrap(),
         PathBuf::from("windowsApplicationData/localFolder")
      );
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
         Box::new(|library: &WindowsLibrary| -> Result<Vec<PathBuf>> {
            Ok(vec![PathBuf::from(format!("windowsLibrary/{}", library))])
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
