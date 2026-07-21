use fs_resolver::{
   AndroidPath, AndroidPathCollection, FsEnvironment, IosPath, LinuxPath, MacPath, PathResolver,
   Result, Win32Path, WindowsApplicationDataPath,
};
use std::path::PathBuf;
use tauri::State;

#[tauri::command]
pub(crate) async fn resolve_android_path(
   resolver: State<'_, PathResolver>,
   path: AndroidPath,
) -> Result<PathBuf> {
   resolver.resolve_android(&path)
}

#[tauri::command]
pub(crate) async fn resolve_android_path_collection(
   resolver: State<'_, PathResolver>,
   collection: AndroidPathCollection,
) -> Result<Vec<PathBuf>> {
   resolver.resolve_android_path_collection(&collection)
}

#[tauri::command]
pub(crate) async fn resolve_ios_path(
   resolver: State<'_, PathResolver>,
   path: IosPath,
) -> Result<PathBuf> {
   resolver.resolve_ios(&path)
}

#[tauri::command]
pub(crate) async fn resolve_linux_path(
   resolver: State<'_, PathResolver>,
   path: LinuxPath,
) -> Result<PathBuf> {
   resolver.resolve_linux(&path)
}

#[tauri::command]
pub(crate) async fn resolve_mac_path(
   resolver: State<'_, PathResolver>,
   path: MacPath,
) -> Result<PathBuf> {
   resolver.resolve_mac(&path)
}

#[tauri::command]
pub(crate) async fn resolve_win32_path(
   resolver: State<'_, PathResolver>,
   path: Win32Path,
) -> Result<PathBuf> {
   resolver.resolve_win32(&path)
}

#[tauri::command]
pub(crate) async fn resolve_windows_application_data_path(
   resolver: State<'_, PathResolver>,
   path: WindowsApplicationDataPath,
) -> Result<PathBuf> {
   resolver.resolve_windows_application_data(&path)
}

#[tauri::command]
pub(crate) async fn get_fs_environment(resolver: State<'_, PathResolver>) -> Result<FsEnvironment> {
   Ok(resolver.environment().clone())
}
