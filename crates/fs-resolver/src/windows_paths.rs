use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum WindowsPath {
   Win32(Win32Path),
   WinMsix(WindowsApplicationDataPath),
}

impl Display for WindowsPath {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
         WindowsPath::Win32(path) => write!(f, "win32::{}", path),
         WindowsPath::WinMsix(path) => write!(f, "winmsix::{}", path),
      }
   }
}

// Taken from KNOWNFOLDERID:
// https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid
// These are meant to be used for Win32 applications packaged as MSI.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Win32Path {
   // User account pictures.
   // %APPDATA%\Microsoft\Windows\AccountPictures
   AccountPictures,

   // User administrative tools shortcuts.
   // %APPDATA%\Microsoft\Windows\Start Menu\Programs\Administrative Tools
   AdminTools,

   // Per-user application shortcuts for pinning.
   // %LOCALAPPDATA%\Microsoft\Windows\Application Shortcuts
   ApplicationShortcuts,

   // Camera roll photos.
   // %USERPROFILE%\Pictures\Camera Roll
   CameraRoll,

   // Staging area for burning CDs/DVDs.
   // %LOCALAPPDATA%\Microsoft\Windows\Burn\Burn
   CdBurning,

   // System-wide administrative tools shortcuts.
   // %ALLUSERSPROFILE%\Microsoft\Windows\Start Menu\Programs\Administrative Tools
   CommonAdminTools,

   // OEM links visible in the Computer folder.
   // %ALLUSERSPROFILE%\OEM Links
   CommonOemLinks,

   // System-wide Start Menu programs.
   // %ALLUSERSPROFILE%\Microsoft\Windows\Start Menu\Programs
   CommonPrograms,

   // System-wide Start Menu root.
   // %ALLUSERSPROFILE%\Microsoft\Windows\Start Menu
   CommonStartMenu,

   // System-wide startup programs.
   // %ALLUSERSPROFILE%\Microsoft\Windows\Start Menu\Programs\StartUp
   CommonStartup,

   // System-wide document templates.
   // %ALLUSERSPROFILE%\Microsoft\Windows\Templates
   CommonTemplates,

   // User contacts.
   // %USERPROFILE%\Contacts
   Contacts,

   // Internet cookies.
   // %APPDATA%\Microsoft\Windows\Cookies
   Cookies,

   // User desktop.
   // %USERPROFILE%\Desktop
   Desktop,

   // Device metadata store.
   // %ALLUSERSPROFILE%\Microsoft\Windows\DeviceMetadataStore
   DeviceMetadataStore,

   // User documents.
   // %USERPROFILE%\Documents
   Documents,

   // Documents library definition.
   // %APPDATA%\Microsoft\Windows\Libraries\Documents.library-ms
   DocumentsLibrary,

   // User downloads.
   // %USERPROFILE%\Downloads
   Downloads,

   // Internet Explorer favorites.
   // %USERPROFILE%\Favorites
   Favorites,

   // System fonts.
   // %windir%\Fonts
   Fonts,

   // Game Explorer data.
   // %LOCALAPPDATA%\Microsoft\Windows\GameExplorer
   GameTasks,

   // Browser history.
   // %LOCALAPPDATA%\Microsoft\Windows\History
   History,

   // Implicit app shortcuts for the jump list.
   // %APPDATA%\Microsoft\Internet Explorer\Quick Launch\User Pinned\ImplicitAppShortcuts
   ImplicitAppShortcuts,

   // Temporary internet files cache.
   // %LOCALAPPDATA%\Microsoft\Windows\Temporary Internet Files
   InternetCache,

   // Windows libraries root.
   // %APPDATA%\Microsoft\Windows\Libraries
   Libraries,

   // User links / favorites in Explorer navigation pane.
   // %USERPROFILE%\Links
   Links,

   // Per-user local application data.
   // %LOCALAPPDATA% (%USERPROFILE%\AppData\Local)
   LocalAppData,

   // Per-user low-integrity application data.
   // %USERPROFILE%\AppData\LocalLow
   LocalAppDataLow,

   // Localized resource directory.
   // %windir%\resources\0409 (code page)
   LocalizedResourcesDir,

   // User music.
   // %USERPROFILE%\Music
   Music,

   // Music library definition.
   // %APPDATA%\Microsoft\Windows\Libraries\Music.library-ms
   MusicLibrary,

   // Network shortcuts (NetHood).
   // %APPDATA%\Microsoft\Windows\Network Shortcuts
   NetworkShortcuts,

   // 3D Objects folder.
   // %USERPROFILE%\3D Objects
   Objects3D,

   // Photo Gallery original images.
   // %LOCALAPPDATA%\Microsoft\Windows Photo Gallery\Original Images
   OriginalImages,

   // Photo album slide shows.
   // %USERPROFILE%\Pictures\Slide Shows
   PhotoAlbums,

   // Pictures library definition.
   // %APPDATA%\Microsoft\Windows\Libraries\Pictures.library-ms
   PicturesLibrary,

   // User pictures.
   // %USERPROFILE%\Pictures
   Pictures,

   // Music playlists.
   // %USERPROFILE%\Music\Playlists
   Playlists,

   // Printer shortcuts (PrintHood).
   // %APPDATA%\Microsoft\Windows\Printer Shortcuts
   PrintHood,

   // User profile root.
   // %USERPROFILE% (%SystemDrive%\Users\%USERNAME%)
   Profile,

   // Machine-wide application data.
   // %ALLUSERSPROFILE% (%ProgramData%, %SystemDrive%\ProgramData)
   ProgramData,

   // Program Files directory.
   // %ProgramFiles% (%SystemDrive%\Program Files)
   ProgramFiles,

   // Program Files directory (64-bit).
   // %ProgramFiles% (%SystemDrive%\Program Files)
   ProgramFilesX64,

   // Program Files directory (32-bit on 64-bit OS).
   // %ProgramFiles(x86)% (%SystemDrive%\Program Files (x86))
   ProgramFilesX86,

   // Common Files directory.
   // %ProgramFiles%\Common Files
   ProgramFilesCommon,

   // Common Files directory (64-bit).
   // %ProgramFiles%\Common Files
   ProgramFilesCommonX64,

   // Common Files directory (32-bit on 64-bit OS).
   // %ProgramFiles(x86)%\Common Files
   ProgramFilesCommonX86,

   // User Start Menu programs.
   // %APPDATA%\Microsoft\Windows\Start Menu\Programs
   Programs,

   // Public user profile root.
   // %PUBLIC% (%SystemDrive%\Users\Public)
   Public,

   // Public desktop.
   // %PUBLIC%\Desktop
   PublicDesktop,

   // Public documents.
   // %PUBLIC%\Documents
   PublicDocuments,

   // Public downloads.
   // %PUBLIC%\Downloads
   PublicDownloads,

   // Public game explorer data.
   // %ALLUSERSPROFILE%\Microsoft\Windows\GameExplorer
   PublicGameTasks,

   // Public libraries root.
   // %ALLUSERSPROFILE%\Microsoft\Windows\Libraries
   PublicLibraries,

   // Public music.
   // %PUBLIC%\Music
   PublicMusic,

   // Public pictures.
   // %PUBLIC%\Pictures
   PublicPictures,

   // Public ringtones.
   // %ALLUSERSPROFILE%\Microsoft\Windows\Ringtones
   PublicRingtones,

   // Public account pictures.
   // %PUBLIC%\AccountPictures
   PublicUserTiles,

   // Public videos.
   // %PUBLIC%\Videos
   PublicVideos,

   // Quick Launch toolbar shortcuts.
   // %APPDATA%\Microsoft\Internet Explorer\Quick Launch
   QuickLaunch,

   // Recently used files.
   // %APPDATA%\Microsoft\Windows\Recent
   Recent,

   // Recorded TV library definition.
   // %PUBLIC%\RecordedTV.library-ms
   RecordedTVLibrary,

   // System resources root.
   // %windir%\Resources
   ResourceDir,

   // User ringtones.
   // %LOCALAPPDATA%\Microsoft\Windows\Ringtones
   Ringtones,

   // Per-user roaming application data.
   // %APPDATA% (%USERPROFILE%\AppData\Roaming)
   RoamingAppData,

   // Roamed tile images for Start.
   // %LOCALAPPDATA%\Microsoft\Windows\RoamedTileImages
   RoamedTileImages,

   // Roaming tile data for Start.
   // %LOCALAPPDATA%\Microsoft\Windows\RoamingTiles
   RoamingTiles,

   // Sample music files.
   // %PUBLIC%\Music\Sample Music
   SampleMusic,

   // Sample picture files.
   // %PUBLIC%\Pictures\Sample Pictures
   SamplePictures,

   // Sample playlist files.
   // %PUBLIC%\Music\Sample Playlists
   SamplePlaylists,

   // Sample video files.
   // %PUBLIC%\Videos\Sample Videos
   SampleVideos,

   // User saved games.
   // %USERPROFILE%\Saved Games
   SavedGames,

   // User saved pictures.
   // %USERPROFILE%\Pictures\Saved Pictures
   SavedPictures,

   // Saved pictures library definition.
   // %APPDATA%\Microsoft\Windows\Libraries\SavedPictures.library-ms
   SavedPicturesLibrary,

   // Saved search queries.
   // %USERPROFILE%\Searches
   Searches,

   // User screenshots.
   // %USERPROFILE%\Pictures\Screenshots
   Screenshots,

   // Connected search history.
   // %LOCALAPPDATA%\Microsoft\Windows\ConnectedSearch\History
   SearchHistory,

   // Connected search templates.
   // %LOCALAPPDATA%\Microsoft\Windows\ConnectedSearch\Templates
   SearchTemplates,

   // "Send To" context menu targets.
   // %APPDATA%\Microsoft\Windows\SendTo
   SendTo,

   // Default sidebar gadgets (Windows 7).
   // %ProgramFiles%\Windows Sidebar\Gadgets
   SidebarDefaultParts,

   // User-installed sidebar gadgets (Windows 7).
   // %LOCALAPPDATA%\Microsoft\Windows Sidebar\Gadgets
   SidebarParts,

   // OneDrive root.
   // %USERPROFILE%\OneDrive
   SkyDrive,

   // OneDrive camera roll.
   // %USERPROFILE%\OneDrive\Pictures\Camera Roll
   SkyDriveCameraRoll,

   // OneDrive documents.
   // %USERPROFILE%\OneDrive\Documents
   SkyDriveDocuments,

   // OneDrive pictures.
   // %USERPROFILE%\OneDrive\Pictures
   SkyDrivePictures,

   // User Start Menu root.
   // %APPDATA%\Microsoft\Windows\Start Menu
   StartMenu,

   // User startup programs.
   // %APPDATA%\Microsoft\Windows\Start Menu\Programs\StartUp
   Startup,

   // System32 directory.
   // %windir%\system32
   System32,

   // System32 directory (32-bit on 64-bit OS).
   // %windir%\system32
   SystemX86,

   // User document templates.
   // %APPDATA%\Microsoft\Windows\Templates
   Templates,

   // User-pinned taskbar and Start items.
   // %APPDATA%\Microsoft\Internet Explorer\Quick Launch\User Pinned
   UserPinned,

   // Users root directory.
   // %SystemDrive%\Users
   Users,

   // Per-user program installations.
   // %LOCALAPPDATA%\Programs
   UserProgramFiles,

   // Per-user common program files.
   // %LOCALAPPDATA%\Programs\Common
   UserProgramFilesCommon,

   // User videos.
   // %USERPROFILE%\Videos
   Videos,

   // Video library definition.
   // %APPDATA%\Microsoft\Windows\Libraries\Videos.library-ms
   VideoLibrary,

   // Windows installation root.
   // %windir%
   Windows,
}

impl Display for Win32Path {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
         Win32Path::AccountPictures => write!(f, "accountPictures"),
         Win32Path::AdminTools => write!(f, "adminTools"),
         Win32Path::ApplicationShortcuts => write!(f, "applicationShortcuts"),
         Win32Path::CameraRoll => write!(f, "cameraRoll"),
         Win32Path::CdBurning => write!(f, "cdBurning"),
         Win32Path::CommonAdminTools => write!(f, "commonAdminTools"),
         Win32Path::CommonOemLinks => write!(f, "commonOemLinks"),
         Win32Path::CommonPrograms => write!(f, "commonPrograms"),
         Win32Path::CommonStartMenu => write!(f, "commonStartMenu"),
         Win32Path::CommonStartup => write!(f, "commonStartup"),
         Win32Path::CommonTemplates => write!(f, "commonTemplates"),
         Win32Path::Contacts => write!(f, "contacts"),
         Win32Path::Cookies => write!(f, "cookies"),
         Win32Path::Desktop => write!(f, "desktop"),
         Win32Path::DeviceMetadataStore => write!(f, "deviceMetadataStore"),
         Win32Path::Documents => write!(f, "documents"),
         Win32Path::DocumentsLibrary => write!(f, "documentsLibrary"),
         Win32Path::Downloads => write!(f, "downloads"),
         Win32Path::Favorites => write!(f, "favorites"),
         Win32Path::Fonts => write!(f, "fonts"),
         Win32Path::GameTasks => write!(f, "gameTasks"),
         Win32Path::History => write!(f, "history"),
         Win32Path::ImplicitAppShortcuts => write!(f, "implicitAppShortcuts"),
         Win32Path::InternetCache => write!(f, "internetCache"),
         Win32Path::Libraries => write!(f, "libraries"),
         Win32Path::Links => write!(f, "links"),
         Win32Path::LocalAppData => write!(f, "localAppData"),
         Win32Path::LocalAppDataLow => write!(f, "localAppDataLow"),
         Win32Path::LocalizedResourcesDir => write!(f, "localizedResourcesDir"),
         Win32Path::Music => write!(f, "music"),
         Win32Path::MusicLibrary => write!(f, "musicLibrary"),
         Win32Path::NetworkShortcuts => write!(f, "networkShortcuts"),
         Win32Path::Objects3D => write!(f, "objects3D"),
         Win32Path::OriginalImages => write!(f, "originalImages"),
         Win32Path::PhotoAlbums => write!(f, "photoAlbums"),
         Win32Path::PicturesLibrary => write!(f, "picturesLibrary"),
         Win32Path::Pictures => write!(f, "pictures"),
         Win32Path::Playlists => write!(f, "playlists"),
         Win32Path::PrintHood => write!(f, "printHood"),
         Win32Path::Profile => write!(f, "profile"),
         Win32Path::ProgramData => write!(f, "programData"),
         Win32Path::ProgramFiles => write!(f, "programFiles"),
         Win32Path::ProgramFilesX64 => write!(f, "programFilesX64"),
         Win32Path::ProgramFilesX86 => write!(f, "programFilesX86"),
         Win32Path::ProgramFilesCommon => write!(f, "programFilesCommon"),
         Win32Path::ProgramFilesCommonX64 => write!(f, "programFilesCommonX64"),
         Win32Path::ProgramFilesCommonX86 => write!(f, "programFilesCommonX86"),
         Win32Path::Programs => write!(f, "programs"),
         Win32Path::Public => write!(f, "public"),
         Win32Path::PublicDesktop => write!(f, "publicDesktop"),
         Win32Path::PublicDocuments => write!(f, "publicDocuments"),
         Win32Path::PublicDownloads => write!(f, "publicDownloads"),
         Win32Path::PublicGameTasks => write!(f, "publicGameTasks"),
         Win32Path::PublicLibraries => write!(f, "publicLibraries"),
         Win32Path::PublicMusic => write!(f, "publicMusic"),
         Win32Path::PublicPictures => write!(f, "publicPictures"),
         Win32Path::PublicRingtones => write!(f, "publicRingtones"),
         Win32Path::PublicUserTiles => write!(f, "publicUserTiles"),
         Win32Path::PublicVideos => write!(f, "publicVideos"),
         Win32Path::QuickLaunch => write!(f, "quickLaunch"),
         Win32Path::Recent => write!(f, "recent"),
         Win32Path::RecordedTVLibrary => write!(f, "recordedTVLibrary"),
         Win32Path::ResourceDir => write!(f, "resourceDir"),
         Win32Path::Ringtones => write!(f, "ringtones"),
         Win32Path::RoamingAppData => write!(f, "roamingAppData"),
         Win32Path::RoamedTileImages => write!(f, "roamedTileImages"),
         Win32Path::RoamingTiles => write!(f, "roamingTiles"),
         Win32Path::SampleMusic => write!(f, "sampleMusic"),
         Win32Path::SamplePictures => write!(f, "samplePictures"),
         Win32Path::SamplePlaylists => write!(f, "samplePlaylists"),
         Win32Path::SampleVideos => write!(f, "sampleVideos"),
         Win32Path::SavedGames => write!(f, "savedGames"),
         Win32Path::SavedPictures => write!(f, "savedPictures"),
         Win32Path::SavedPicturesLibrary => write!(f, "savedPicturesLibrary"),
         Win32Path::Searches => write!(f, "searches"),
         Win32Path::Screenshots => write!(f, "screenshots"),
         Win32Path::SearchHistory => write!(f, "searchHistory"),
         Win32Path::SearchTemplates => write!(f, "searchTemplates"),
         Win32Path::SendTo => write!(f, "sendTo"),
         Win32Path::SidebarDefaultParts => write!(f, "sidebarDefaultParts"),
         Win32Path::SidebarParts => write!(f, "sidebarParts"),
         Win32Path::SkyDrive => write!(f, "skyDrive"),
         Win32Path::SkyDriveCameraRoll => write!(f, "skyDriveCameraRoll"),
         Win32Path::SkyDriveDocuments => write!(f, "skyDriveDocuments"),
         Win32Path::SkyDrivePictures => write!(f, "skyDrivePictures"),
         Win32Path::StartMenu => write!(f, "startMenu"),
         Win32Path::Startup => write!(f, "startup"),
         Win32Path::System32 => write!(f, "system32"),
         Win32Path::SystemX86 => write!(f, "systemX86"),
         Win32Path::Templates => write!(f, "templates"),
         Win32Path::UserPinned => write!(f, "userPinned"),
         Win32Path::Users => write!(f, "users"),
         Win32Path::UserProgramFiles => write!(f, "userProgramFiles"),
         Win32Path::UserProgramFilesCommon => write!(f, "userProgramFilesCommon"),
         Win32Path::Videos => write!(f, "videos"),
         Win32Path::VideoLibrary => write!(f, "videoLibrary"),
         Win32Path::Windows => write!(f, "windows"),
      }
   }
}

// Taken from here:
// https://learn.microsoft.com/en-us/uwp/api/windows.storage.applicationdata?view=winrt-28000#properties
// These are meant to be used for Win32 applications packaged as MSIX.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum WindowsApplicationDataPath {
   // App-specific local cache. Not roamed, not backed up; may be purged by system.
   // C:\Users\<user>\AppData\Local\Packages\<id>\LocalCache
   LocalCacheFolder,

   // App-specific local data. Not roamed; backed up if configured.
   // C:\Users\<user>\AppData\Local\Packages\<id>\LocalState
   LocalFolder,

   // App-specific local settings container (registry-backed, not a filesystem path).
   // HKCU\Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\<id>
   LocalSettings,

   // App-specific roaming data. Synced across devices via the user's Microsoft account.
   // C:\Users\<user>\AppData\Local\Packages\<id>\RoamingState
   RoamingFolder,

   // App-specific roaming settings container (registry-backed, not a filesystem path).
   // Synced across devices via the user's Microsoft account.
   RoamingSettings,

   // App data shared between all users of the machine for this package.
   // C:\ProgramData\Packages\<id>\LocalCache
   SharedLocalFolder,

   // App-specific temporary files. May be purged by system at any time.
   // C:\Users\<user>\AppData\Local\Packages\<id>\TempState
   TemporaryFolder,
}

impl Display for WindowsApplicationDataPath {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
         WindowsApplicationDataPath::LocalCacheFolder => write!(f, "localCacheFolder"),
         WindowsApplicationDataPath::LocalFolder => write!(f, "localFolder"),
         WindowsApplicationDataPath::LocalSettings => write!(f, "localSettings"),
         WindowsApplicationDataPath::RoamingFolder => write!(f, "roamingFolder"),
         WindowsApplicationDataPath::RoamingSettings => write!(f, "roamingSettings"),
         WindowsApplicationDataPath::SharedLocalFolder => write!(f, "sharedLocalFolder"),
         WindowsApplicationDataPath::TemporaryFolder => write!(f, "temporaryFolder"),
      }
   }
}
