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
      AndroidPath, AndroidPathCollection, IosPath, LinuxPath, MacPath, PathResolver, WindowsPath,
   };
   use fs_resolver::{Result, Win32Path};

   use std::path::PathBuf;

   const AVAILABLE_OS: [&str; 5] = ["android", "ios", "linux", "macos", "windows"];

   #[test]
   fn test_path_resolver() {
      for os in AVAILABLE_OS {
         let resolver = create_test_resolver(os);
         if os == "android" {
            let resolved = resolver.resolve_android(&AndroidPath::DataDir);
            assert_eq!(resolved.unwrap(), PathBuf::from("android/dataDir"));
            let resolved =
               resolver.resolve_android_path_collection(&AndroidPathCollection::ExternalCacheDirs);
            assert_eq!(
               resolved.unwrap(),
               vec![PathBuf::from("android/externalCacheDirs")]
            );
         }

         if os == "ios" {
            let resolved = resolver.resolve_ios(&IosPath::DocumentDirectory);
            assert_eq!(resolved.unwrap(), PathBuf::from("ios/documentDirectory"));
         }

         if os == "linux" {
            let resolved = resolver.resolve_linux(&LinuxPath::DataHome);
            assert_eq!(resolved.unwrap(), PathBuf::from("linux/dataHome"));
         }

         if os == "macos" {
            let resolved = resolver.resolve_mac(&MacPath::ApplicationDirectory);
            assert_eq!(
               resolved.unwrap(),
               PathBuf::from("apple/applicationDirectory")
            );
         }

         if os == "windows" {
            let resolved = resolver.resolve_windows(&WindowsPath::Win32(Win32Path::LocalAppData));
            assert_eq!(
               resolved.unwrap(),
               PathBuf::from("windows/win32::localAppData")
            );
         }
      }
   }

   fn create_test_resolver(platform: &str) -> PathResolver {
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
         platform.to_string(),
         resolve_android,
         resolve_android_path_collection,
         resolve_ios,
         resolve_linux,
         resolve_mac,
         resolve_windows,
      )
   }
}
