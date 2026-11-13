use fs_resolver::PathResolver;
use tauri::Manager;
use tauri::plugin::Builder;
use tauri::{Runtime, plugin::TauriPlugin};

mod commands;

#[cfg(target_os = "android")]
mod android_resolution;

/// The resolver API used by this plugin, including its managed [`fs_resolver::PathResolver`].
///
/// Import resolver types through this re-export to use the same types as the plugin's
/// managed state without a separate dependency on `fs-resolver`. The re-exported crate's
/// public API is part of the plugin's public API.
pub use fs_resolver;

/// Re-exported so a consumer can name the error and return types, rather than falling back
/// on inference and `to_string()`.
pub use fs_resolver::{Error, Result};

/// Re-exported so consumers reach backup exclusion through the plugin rather than needing
/// a second direct dependency on `fs-resolver`. Unlike the resolve commands, these have no
/// IPC equivalent, so this is the only route to them.
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use fs_resolver::{is_excluded_from_backup, set_excluded_from_backup};

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
         commands::get_fs_environment,
         commands::resolve_android_path,
         commands::resolve_android_path_collection,
         commands::resolve_ios_path,
         commands::resolve_linux_path,
         commands::resolve_mac_path,
         commands::resolve_win32_path,
         commands::resolve_windows_application_data_path,
         commands::resolve_windows_library_dirs,
      ])
      .build()
}
