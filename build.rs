const COMMANDS: &[&str] = &[
   "resolve_android_path",
   "resolve_android_path_collection",
   "resolve_ios_path",
   "resolve_linux_path",
   "resolve_mac_path",
   "resolve_windows_path",
];

fn main() {
   tauri_plugin::Builder::new(COMMANDS).build();
}
