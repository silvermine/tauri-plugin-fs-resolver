const COMMANDS: &[&str] = &[
   "get_fs_environment",
   "resolve_android_path",
   "resolve_android_path_collection",
   "resolve_ios_path",
   "resolve_linux_path",
   "resolve_mac_path",
   "resolve_win32_path",
   "resolve_windows_application_data_path",
];

fn main() {
   tauri_plugin::Builder::new(COMMANDS)
      .android_path("android")
      .build();
}
