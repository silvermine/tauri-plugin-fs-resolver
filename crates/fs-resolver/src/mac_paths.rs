use serde::Deserialize;
use std::fmt::Display;

// These values are based on SearchPathDirectory enum in Apple's Foundation framework:
// https://developer.apple.com/documentation/foundation/filemanager/searchpathdirectory/
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MacPath {
   // Supported applications.
   // /Applications
   ApplicationDirectory,

   // Unsupported applications and demonstration versions.
   // /Applications/Demos
   DemoApplicationDirectory,

   // Developer applications.
   // /Developer/Applications
   DeveloperApplicationDirectory,

   // System and network administration applications.
   // /Applications/Utilities
   AdminApplicationDirectory,

   // Various user-visible documentation, support, and configuration files.
   // /Library
   LibraryDirectory,

   // Developer resources.
   // /Developer
   DeveloperDirectory,

   // User home directories.
   // /Users
   UserDirectory,

   // Documentation.
   // /Library/Documentation
   DocumentationDirectory,

   // User document directory.
   // ~/Documents
   DocumentDirectory,

   // Core services.
   // /System/Library/CoreServices
   CoreServiceDirectory,

   // The user's autosaved documents.
   // ~/Library/Autosave Information
   AutosavedInformationDirectory,

   // The user's desktop directory.
   // ~/Desktop
   DesktopDirectory,

   // Discardable cache files.
   // ~/Library/Caches
   CachesDirectory,

   // Application support files.
   // ~/Library/Application Support
   ApplicationSupportDirectory,

   // The user's downloads directory.
   // ~/Downloads
   DownloadsDirectory,

   // Input methods.
   // ~/Library/Input Methods
   InputMethodsDirectory,

   // The user's Movies directory.
   // ~/Movies
   MoviesDirectory,

   // The user's Music directory.
   // ~/Music
   MusicDirectory,

   // The user's Pictures directory.
   // ~/Pictures
   PicturesDirectory,

   // The system's PPDs directory.
   // /Library/Printers/PPDs
   PrinterDescriptionDirectory,

   // The user's Public sharing directory.
   // ~/Public
   SharedPublicDirectory,

   // The PreferencePanes directory for use with System Preferences.
   // ~/Library/PreferencePanes
   PreferencePanesDirectory,

   // The user scripts folder for the calling application.
   // ~/Library/Application Scripts/<code-signing-id>
   ApplicationScriptsDirectory,

   // Used with url(for:in:appropriateFor:create:) for atomic safe-save operations.
   // Not a fixed path.
   ItemReplacementDirectory,

   // All directories where applications can be stored.
   // (/Applications, ~/Applications, /Network/Applications)
   AllApplicationsDirectory,

   // All directories where resources can be stored.
   // (/Library, ~/Library, /Network/Library)
   AllLibrariesDirectory,

   // The trash directory.
   // ~/.Trash
   TrashDirectory,
}

impl Display for MacPath {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
         MacPath::ApplicationDirectory => write!(f, "applicationDirectory"),
         MacPath::DemoApplicationDirectory => write!(f, "demoApplicationDirectory"),
         MacPath::DeveloperApplicationDirectory => write!(f, "developerApplicationDirectory"),
         MacPath::AdminApplicationDirectory => write!(f, "adminApplicationDirectory"),
         MacPath::LibraryDirectory => write!(f, "libraryDirectory"),
         MacPath::DeveloperDirectory => write!(f, "developerDirectory"),
         MacPath::UserDirectory => write!(f, "userDirectory"),
         MacPath::DocumentationDirectory => write!(f, "documentationDirectory"),
         MacPath::DocumentDirectory => write!(f, "documentDirectory"),
         MacPath::CoreServiceDirectory => write!(f, "coreServiceDirectory"),
         MacPath::AutosavedInformationDirectory => write!(f, "autosavedInformationDirectory"),
         MacPath::DesktopDirectory => write!(f, "desktopDirectory"),
         MacPath::CachesDirectory => write!(f, "cachesDirectory"),
         MacPath::ApplicationSupportDirectory => write!(f, "applicationSupportDirectory"),
         MacPath::DownloadsDirectory => write!(f, "downloadsDirectory"),
         MacPath::InputMethodsDirectory => write!(f, "inputMethodsDirectory"),
         MacPath::MoviesDirectory => write!(f, "moviesDirectory"),
         MacPath::MusicDirectory => write!(f, "musicDirectory"),
         MacPath::PicturesDirectory => write!(f, "picturesDirectory"),
         MacPath::PrinterDescriptionDirectory => write!(f, "printerDescriptionDirectory"),
         MacPath::SharedPublicDirectory => write!(f, "sharedPublicDirectory"),
         MacPath::PreferencePanesDirectory => write!(f, "preferencePanesDirectory"),
         MacPath::ApplicationScriptsDirectory => write!(f, "applicationScriptsDirectory"),
         MacPath::ItemReplacementDirectory => write!(f, "itemReplacementDirectory"),
         MacPath::AllApplicationsDirectory => write!(f, "allApplicationsDirectory"),
         MacPath::AllLibrariesDirectory => write!(f, "allLibrariesDirectory"),
         MacPath::TrashDirectory => write!(f, "trashDirectory"),
      }
   }
}
