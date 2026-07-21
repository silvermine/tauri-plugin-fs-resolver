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
   use fs_resolver::{
      AndroidPath, AndroidPathCollection, FsEnvironment, IosPath, LinuxPath, MacPath, PathResolver,
      Result, Win32Path, WindowsApplicationDataPath,
   };
   use std::path::PathBuf;
   use test_case::test_case;

   #[test_case(FsEnvironment::Android)]
   #[test_case(FsEnvironment::Ios)]
   #[test_case(FsEnvironment::Linux)]
   #[test_case(FsEnvironment::Macos)]
   #[test_case(FsEnvironment::Win32)]
   #[test_case(FsEnvironment::WinPackaged)]
   fn test_path_resolver(environment: FsEnvironment) {
      let resolver = create_test_resolver(environment.clone());

      match environment {
         FsEnvironment::Android => {
            let resolved = resolver.resolve_android(&AndroidPath::DataDir);
            assert_eq!(resolved.unwrap(), PathBuf::from("android/dataDir"));
            let resolved =
               resolver.resolve_android_path_collection(&AndroidPathCollection::ExternalCacheDirs);
            assert_eq!(
               resolved.unwrap(),
               vec![PathBuf::from("android/externalCacheDirs")]
            );
         }
         FsEnvironment::Ios => {
            let resolved = resolver.resolve_ios(&IosPath::DocumentDirectory);
            assert_eq!(resolved.unwrap(), PathBuf::from("ios/documentDirectory"));
         }
         FsEnvironment::Linux => {
            let resolved = resolver.resolve_linux(&LinuxPath::DataHome);
            assert_eq!(resolved.unwrap(), PathBuf::from("linux/dataHome"));
         }
         FsEnvironment::Macos => {
            let resolved = resolver.resolve_mac(&MacPath::ApplicationDirectory);
            assert_eq!(
               resolved.unwrap(),
               PathBuf::from("apple/applicationDirectory")
            );
         }
         FsEnvironment::Win32 => {
            let resolved = resolver.resolve_win32(&Win32Path::LocalAppData);
            assert_eq!(resolved.unwrap(), PathBuf::from("win32/localAppData"));
         }
         FsEnvironment::WinPackaged => {
            let resolved =
               resolver.resolve_windows_application_data(&WindowsApplicationDataPath::LocalFolder);
            assert_eq!(
               resolved.unwrap(),
               PathBuf::from("windowsApplicationData/localFolder")
            );
         }
      }
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
