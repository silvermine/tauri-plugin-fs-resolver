use tauri::plugin::Builder;
use tauri::{Runtime, plugin::TauriPlugin};

mod commands;

/// Initializes the fs-resolver plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
   Builder::new("fs-resolver")
      .invoke_handler(tauri::generate_handler![
         commands::resolve_android_path,
         commands::resolve_android_path_collection,
         commands::resolve_ios_path,
         commands::resolve_linux_path,
         commands::resolve_mac_path,
         commands::resolve_windows_path,
      ])
      .build()
}
