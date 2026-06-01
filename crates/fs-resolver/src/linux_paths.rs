use serde::Deserialize;
use std::fmt::Display;

// XDG Base Directory Specification + XDG User Directories.
// https://specifications.freedesktop.org/basedir-spec/latest/
// https://www.freedesktop.org/wiki/Software/xdg-user-dirs/
//
// The resolver returns base directories; callers append their app identifier
// (e.g. `DataHome` → `~/.local/share/`, app appends `<app-id>/`).
// Flatpak/Snap runtimes automatically remap these XDG vars to sandbox paths.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum LinuxPath {
   // App-specific data: databases, user-generated content.
   // $XDG_DATA_HOME (~/.local/share)
   DataHome,

   // App-specific configuration files.
   // $XDG_CONFIG_HOME (~/.config)
   ConfigHome,

   // Disposable cache data (safe to delete).
   // $XDG_CACHE_HOME (~/.cache)
   CacheHome,

   // Non-portable state: logs, undo history, session state.
   // $XDG_STATE_HOME (~/.local/state)
   StateHome,

   // Ephemeral runtime files: sockets, named pipes. Lifetime bound to login session.
   // $XDG_RUNTIME_DIR (/run/user/<uid>, set by pam/systemd; no fallback)
   RuntimeDir,

   // User home directory.
   // $HOME (~)
   Home,

   // User-specific executables.
   // $XDG_BIN_HOME or ~/.local/bin (de-facto convention, not part of the XDG spec).
   ExecutableDir,

   // User-specific fonts.
   // $XDG_DATA_HOME/fonts (~/.local/share/fonts)
   FontDir,

   // --- XDG User Directories (parsed from ~/.config/user-dirs.dirs) ---

   // ~/Desktop
   DesktopDir,

   // ~/Documents
   DocumentDir,

   // ~/Downloads
   DownloadDir,

   // ~/Music
   MusicDir,

   // ~/Pictures
   PictureDir,

   // ~/Videos
   VideoDir,

   // ~/Templates
   TemplateDir,

   // ~/Public
   PublicDir,
}

impl Display for LinuxPath {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
         LinuxPath::DataHome => write!(f, "dataHome"),
         LinuxPath::ConfigHome => write!(f, "configHome"),
         LinuxPath::CacheHome => write!(f, "cacheHome"),
         LinuxPath::StateHome => write!(f, "stateHome"),
         LinuxPath::RuntimeDir => write!(f, "runtimeDir"),
         LinuxPath::Home => write!(f, "home"),
         LinuxPath::ExecutableDir => write!(f, "executableDir"),
         LinuxPath::FontDir => write!(f, "fontDir"),
         LinuxPath::DesktopDir => write!(f, "desktopDir"),
         LinuxPath::DocumentDir => write!(f, "documentDir"),
         LinuxPath::DownloadDir => write!(f, "downloadDir"),
         LinuxPath::MusicDir => write!(f, "musicDir"),
         LinuxPath::PictureDir => write!(f, "pictureDir"),
         LinuxPath::VideoDir => write!(f, "videoDir"),
         LinuxPath::TemplateDir => write!(f, "templateDir"),
         LinuxPath::PublicDir => write!(f, "publicDir"),
      }
   }
}
