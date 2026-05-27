use crate::error::Error;
use crate::error::Result;
use crate::mac_paths::MacPath;
use std::path::PathBuf;

pub(crate) fn resolve_mac_path(path: &MacPath) -> Result<PathBuf> {
   #[cfg(target_os = "macos")]
   {
      resolve_mac_path_inner(path)
   }

   #[cfg(not(target_os = "macos"))]
   {
      Err(Error::IncorrectOS {
         path: path.to_string(),
         current_os: std::env::consts::OS.to_string(),
         expected_os: "macos".to_string(),
      })
   }
}

#[cfg(target_os = "macos")]
pub(crate) fn resolve_mac_path_inner(path: &MacPath) -> Result<PathBuf> {
   use objc2_foundation::{
      NSSearchPathDirectory, NSSearchPathDomainMask, NSSearchPathForDirectoriesInDomains,
   };

   fn resolve_search_path_directory(directory: NSSearchPathDirectory) -> Result<PathBuf> {
      let arr = NSSearchPathForDirectoriesInDomains(directory, NS_USER_DOMAIN_MASK, true);
      let first = arr
         .firstObject()
         .ok_or_else(|| Error::InvalidPath(format!("{:?} not found", directory)))?;

      Ok(PathBuf::from(first.to_string()))
   }

   const NS_USER_DOMAIN_MASK: NSSearchPathDomainMask = NSSearchPathDomainMask::UserDomainMask;

   match path {
      // Supported applications (/Applications).
      MacPath::ApplicationDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::ApplicationDirectory)
      }

      // Unsupported applications and demonstration versions.
      MacPath::DemoApplicationDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::DemoApplicationDirectory)
      }

      // Developer applications (/Developer/Applications).
      MacPath::DeveloperApplicationDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::DeveloperApplicationDirectory)
      }

      // System and network administration applications.
      MacPath::AdminApplicationDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::AdminApplicationDirectory)
      }

      // Various user-visible documentation, support, and configuration files (/Library).
      MacPath::LibraryDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::LibraryDirectory)
      }

      // Developer resources (/Developer).
      MacPath::DeveloperDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::DeveloperDirectory)
      }

      // User home directories (/Users).
      MacPath::UserDirectory => resolve_search_path_directory(NSSearchPathDirectory::UserDirectory),

      // Documentation.
      MacPath::DocumentationDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::DocumentationDirectory)
      }

      // Document directory.
      MacPath::DocumentDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::DocumentDirectory)
      }

      // Core services (System/Library/CoreServices).
      MacPath::CoreServiceDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::CoreServiceDirectory)
      }

      // The user’s autosaved documents (Library/Autosave Information).
      MacPath::AutosavedInformationDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::AutosavedInformationDirectory)
      }

      // The user’s desktop directory.
      MacPath::DesktopDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::DesktopDirectory)
      }

      // Discardable cache files (Library/Caches).
      MacPath::CachesDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::CachesDirectory)
      }

      // Application support files (Library/Application Support).
      MacPath::ApplicationSupportDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::ApplicationSupportDirectory)
      }

      // The user’s downloads directory.
      MacPath::DownloadsDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::DownloadsDirectory)
      }

      // Input Methods (Library/Input Methods).
      MacPath::InputMethodsDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::InputMethodsDirectory)
      }

      // The user’s Movies directory (~/Movies).
      MacPath::MoviesDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::MoviesDirectory)
      }

      // The user’s Music directory (~/Music).
      MacPath::MusicDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::MusicDirectory)
      }

      // The user’s Pictures directory (~/Pictures).
      MacPath::PicturesDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::PicturesDirectory)
      }

      // The system’s PPDs directory (Library/Printers/PPDs).
      MacPath::PrinterDescriptionDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::PrinterDescriptionDirectory)
      }

      // The user’s Public sharing directory (~/Public).
      MacPath::SharedPublicDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::SharedPublicDirectory)
      }

      // The PreferencePanes directory for use with System Preferences (Library/PreferencePanes).
      MacPath::PreferencePanesDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::PreferencePanesDirectory)
      }

      // The user scripts folder for the calling application (~/Library/Application Scripts/<code-signing-id>.
      MacPath::ApplicationScriptsDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::ApplicationScriptsDirectory)
      }

      // The constant used to create a temporary directory.
      MacPath::ItemReplacementDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::ItemReplacementDirectory)
      }

      // All directories where applications can be stored.
      MacPath::AllApplicationsDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::AllApplicationsDirectory)
      }

      // All directories where resources can be stored.
      MacPath::AllLibrariesDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::AllLibrariesDirectory)
      }

      // The trash directory.
      MacPath::TrashDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::TrashDirectory)
      }
   }
}
