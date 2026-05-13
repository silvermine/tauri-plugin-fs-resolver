use serde::Deserialize;
use std::fmt::Display;

// These values are based on the XDG Base Directory Specification and XDG User Directories:
// https://specifications.freedesktop.org/basedir-spec/latest/
// https://www.freedesktop.org/wiki/Software/xdg-user-dirs/
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum LinuxPath {
   // The machine's base configuration directory.
   // $XDG_CONFIG_HOME (~/.config)
   BaseConfigHomeDirectory,

   // The machine's base data directory.
   // $XDG_DATA_HOME (~/.local/share)
   BaseDataHomeDirectory,

   // The machine's base cache directory.
   // $XDG_CACHE_HOME (~/.cache)
   BaseCacheHomeDirectory,

   // The machine's base state directory.
   // $XDG_STATE_HOME (~/.local/state)
   BaseStateHomeDirectory,

   // The machine's runtime directory.
   // $XDG_RUNTIME_DIR (set by pam/systemd)
   BaseRuntimeDirectory,

   // The user's home directory.
   // $HOME (~)
   UserHomeDirectory,

   // The user's configuration directory.
   // $XDG_CONFIG_HOME (~/.config)
   UserConfigDirectory,

   // The user's data directory.
   // $XDG_DATA_HOME (~/.local/share)
   UserDataDirectory,

   // The user's cache directory.
   // $XDG_CACHE_HOME (~/.cache)
   UserCacheDirectory,

   // The user's state directory.
   // $XDG_STATE_HOME (~/.local/state)
   UserStateDirectory,

   // The user's runtime directory.
   // $XDG_RUNTIME_DIR (set by pam/systemd, e.g. /run/user/<uid>)
   UserRuntimeDirectory,

   // The user's executable directory.
   // ~/.local/bin
   UserExecutableDirectory,

   // The user's font directory.
   // $XDG_DATA_HOME/fonts (~/.local/share/fonts)
   UserFontDirectory,

   // The user's desktop directory.
   // $XDG_DESKTOP_DIR (~/Desktop)
   UserDesktopDirectory,

   // The user's document directory.
   // $XDG_DOCUMENTS_DIR (~/Documents)
   UserDocumentDirectory,

   // The user's download directory.
   // $XDG_DOWNLOAD_DIR (~/Downloads)
   UserDownloadDirectory,

   // The user's music directory.
   // $XDG_MUSIC_DIR (~/Music)
   UserMusicDirectory,

   // The user's pictures directory.
   // $XDG_PICTURES_DIR (~/Pictures)
   UserPictureDirectory,

   // The user's videos directory.
   // $XDG_VIDEOS_DIR (~/Videos)
   UserVideoDirectory,

   // The user's templates directory.
   // $XDG_TEMPLATES_DIR (~/Templates)
   UserTemplateDirectory,

   // The user's public share directory.
   // $XDG_PUBLICSHARE_DIR (~/Public)
   UserPublicDirectory,
}

impl Display for LinuxPath {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
         LinuxPath::BaseConfigHomeDirectory => write!(f, "baseConfigHomeDirectory"),
         LinuxPath::BaseDataHomeDirectory => write!(f, "baseDataHomeDirectory"),
         LinuxPath::BaseCacheHomeDirectory => write!(f, "baseCacheHomeDirectory"),
         LinuxPath::BaseStateHomeDirectory => write!(f, "baseStateHomeDirectory"),
         LinuxPath::BaseRuntimeDirectory => write!(f, "baseRuntimeDirectory"),
         LinuxPath::UserHomeDirectory => write!(f, "userHomeDirectory"),
         LinuxPath::UserConfigDirectory => write!(f, "userConfigDirectory"),
         LinuxPath::UserDataDirectory => write!(f, "userDataDirectory"),
         LinuxPath::UserCacheDirectory => write!(f, "userCacheDirectory"),
         LinuxPath::UserStateDirectory => write!(f, "userStateDirectory"),
         LinuxPath::UserRuntimeDirectory => write!(f, "userRuntimeDirectory"),
         LinuxPath::UserExecutableDirectory => write!(f, "userExecutableDirectory"),
         LinuxPath::UserFontDirectory => write!(f, "userFontDirectory"),
         LinuxPath::UserDesktopDirectory => write!(f, "userDesktopDirectory"),
         LinuxPath::UserDocumentDirectory => write!(f, "userDocumentDirectory"),
         LinuxPath::UserDownloadDirectory => write!(f, "userDownloadDirectory"),
         LinuxPath::UserMusicDirectory => write!(f, "userMusicDirectory"),
         LinuxPath::UserPictureDirectory => write!(f, "userPictureDirectory"),
         LinuxPath::UserVideoDirectory => write!(f, "userVideoDirectory"),
         LinuxPath::UserTemplateDirectory => write!(f, "userTemplateDirectory"),
         LinuxPath::UserPublicDirectory => write!(f, "userPublicDirectory"),
      }
   }
}
