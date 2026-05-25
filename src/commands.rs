use fs_resolver::{
   AndroidPath, AndroidPathCollection, IosPath, LinuxPath, MacPath, PathResolver, Result,
   WindowsPath,
};
use std::path::PathBuf;
use std::sync::LazyLock;
use tauri::{AppHandle, Runtime};

static RESOLVER: LazyLock<PathResolver> = LazyLock::new(PathResolver::new);
fn get_resolver() -> &'static PathResolver {
   &RESOLVER
}

#[tauri::command]
pub(crate) async fn resolve_android_path<R: Runtime>(
   _app: AppHandle<R>,
   path: AndroidPath,
) -> Result<PathBuf> {
   get_resolver().resolve_android(&path)
}

#[tauri::command]
pub(crate) async fn resolve_android_path_collection<R: Runtime>(
   _app: AppHandle<R>,
   path: AndroidPathCollection,
) -> Result<Vec<PathBuf>> {
   get_resolver().resolve_android_path_collection(&path)
}

#[tauri::command]
pub(crate) async fn resolve_ios_path<R: Runtime>(
   _app: AppHandle<R>,
   path: IosPath,
) -> Result<PathBuf> {
   get_resolver().resolve_ios(&path)
}

#[tauri::command]
pub(crate) async fn resolve_linux_path<R: Runtime>(
   _app: AppHandle<R>,
   path: LinuxPath,
) -> Result<PathBuf> {
   get_resolver().resolve_linux(&path)
}

#[tauri::command]
pub(crate) async fn resolve_mac_path<R: Runtime>(
   _app: AppHandle<R>,
   path: MacPath,
) -> Result<PathBuf> {
   get_resolver().resolve_mac(&path)
}

#[tauri::command]
pub(crate) async fn resolve_windows_path<R: Runtime>(
   _app: AppHandle<R>,
   path: WindowsPath,
) -> Result<PathBuf> {
   get_resolver().resolve_windows(&path)
}
