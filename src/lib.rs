use fs_resolver::PathResolver;
use tauri::Manager;
use tauri::plugin::Builder;
use tauri::{Runtime, plugin::TauriPlugin};

mod commands;

#[cfg(target_os = "android")]
mod android_resolution;

/// Initializes the fs-resolver plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
   Builder::new("fs-resolver")
      // The `_api` value is used when compiling for Android, but not for other platforms.
      // To avoid a clippy error, we need to use `_api` instead of `api`.
      .setup(|app, _api| {

         #[allow(unused_mut)]
         let mut resolver = PathResolver::new(
            app.config().identifier.clone(),
         )?;
         #[cfg(target_os = "android")]
         {
            android_resolution::configure_android_path_resolution(&_api, &mut resolver)?;
         }

         app.manage(resolver);

         Ok(())
      })
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
