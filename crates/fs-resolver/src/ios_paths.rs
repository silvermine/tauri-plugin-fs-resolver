use std::fmt::Display;

use serde::Deserialize;

// Directories that resolve to usable paths within the iOS app sandbox.
//
// SearchPathDirectory values that exist in the API but have no meaningful
// resolution on iOS are excluded: ApplicationDirectory, DemoApplicationDirectory,
// DeveloperApplicationDirectory, AdminApplicationDirectory, DeveloperDirectory,
// UserDirectory, DocumentationDirectory, CoreServiceDirectory, DesktopDirectory,
// InputMethodsDirectory, PrinterDescriptionDirectory, SharedPublicDirectory,
// PreferencePanesDirectory, AllApplicationsDirectory, AllLibrariesDirectory,
// ApplicationScriptsDirectory (macOS only), TrashDirectory (macOS only).
//
// Values from FileManager's "Accessing user directories" section
// (temporaryDirectory, homeDirectoryForCurrentUser) are included as they map
// to standalone FileManager properties rather than SearchPathDirectory cases.
// https://developer.apple.com/documentation/foundation/filemanager#Accessing-user-directories
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum IosPath {
   // Backed up, visible in Files app. Maps to SearchPathDirectory.documentDirectory.
   // <sandbox>/Documents
   DocumentDirectory,

   // Contains app support, caches, preferences. Maps to SearchPathDirectory.libraryDirectory.
   // <sandbox>/Library
   LibraryDirectory,

   // Not backed up, may be purged by system. Maps to SearchPathDirectory.cachesDirectory.
   // <sandbox>/Library/Caches
   CachesDirectory,

   // Backed up, hidden from user. Maps to SearchPathDirectory.applicationSupportDirectory.
   // <sandbox>/Library/Application Support
   ApplicationSupportDirectory,

   // Autosaved document storage. Maps to SearchPathDirectory.autosavedInformationDirectory.
   // <sandbox>/Documents/Autosaved
   AutosavedInformationDirectory,

   // User downloads. Maps to SearchPathDirectory.downloadsDirectory.
   // <sandbox>/Downloads
   DownloadsDirectory,

   // Media storage. Maps to SearchPathDirectory.moviesDirectory.
   // <sandbox>/Movies
   MoviesDirectory,

   // Media storage. Maps to SearchPathDirectory.musicDirectory.
   // <sandbox>/Music
   MusicDirectory,

   // Media storage. Maps to SearchPathDirectory.picturesDirectory.
   // <sandbox>/Pictures
   PicturesDirectory,

   // Used with FileManager.url(for:in:appropriateFor:create:) for atomic safe-save operations.
   // Not a fixed path.
   ItemReplacementDirectory,

   // Not backed up, may be purged by system. Maps to FileManager.temporaryDirectory.
   // <sandbox>/tmp
   TemporaryDirectory,

   // The sandbox root directory for the app. Maps to FileManager.homeDirectoryForCurrentUser.
   // <sandbox>/
   HomeDirectory,
}

impl Display for IosPath {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
         IosPath::DocumentDirectory => write!(f, "documentDirectory"),
         IosPath::LibraryDirectory => write!(f, "libraryDirectory"),
         IosPath::CachesDirectory => write!(f, "cachesDirectory"),
         IosPath::ApplicationSupportDirectory => write!(f, "applicationSupportDirectory"),
         IosPath::AutosavedInformationDirectory => write!(f, "autosavedInformationDirectory"),
         IosPath::DownloadsDirectory => write!(f, "downloadsDirectory"),
         IosPath::MoviesDirectory => write!(f, "moviesDirectory"),
         IosPath::MusicDirectory => write!(f, "musicDirectory"),
         IosPath::PicturesDirectory => write!(f, "picturesDirectory"),
         IosPath::ItemReplacementDirectory => write!(f, "itemReplacementDirectory"),
         IosPath::TemporaryDirectory => write!(f, "temporaryDirectory"),
         IosPath::HomeDirectory => write!(f, "homeDirectory"),
      }
   }
}
