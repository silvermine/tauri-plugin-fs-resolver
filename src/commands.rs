use fs_resolver::{
   AndroidPath, AndroidPathCollection, IosPath, LinuxPath, MacPath, PathResolver, Result,
   WindowsPath,
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
pub(crate) async fn resolve_windows_path(
   resolver: State<'_, PathResolver>,
   path: WindowsPath,
) -> Result<PathBuf> {
   resolver.resolve_windows(&path)
}
