/**
 * The filesystem environment of the current operating system.
 *
 * This is used to determine the appropriate filesystem path resolution strategy.
 *
 * Q: Why do we need this enum instead of the string for std::env::consts::OS?
 * A: This solves a specific Windows problem, where Windows can be either Win32
 * or WinPackaged.
 * The os string alone is not enough to determine the environment.
 * Win32 paths can be resolved on both Win32 and WinPackaged, but WinPackaged paths
 * cannot be resolved on Win32.
 * Having a declaritive enum is the most robust solution for this problem,
 * which allows us to determine the environment once at runtime and hold the value
 * for the lifetime of the application.
 */
export type FsEnvironment = 'android' | 'ios' | 'linux' | 'macos' | 'win32' | 'winpackaged';

/**
 * Android app directories.
 *
 * @see https://developer.android.com/reference/android/content/Context
 */
export enum AndroidPath {

   /** App-private data directory. Cleared on uninstall.
    * `/data/user/0/com.example.app`
    */
   DataDir = 'dataDir',

   /** App-private files directory. Cleared on uninstall.
    * `/data/user/0/com.example.app/files`
    */
   FilesDir = 'filesDir',

   /** App-private files excluded from auto-backup. Cleared on uninstall.
    * `/data/user/0/com.example.app/no_backup`
    */
   NoBackupFilesDir = 'noBackupFilesDir',

   /** Primary OBB (expansion file) directory for the app.
    * `/storage/emulated/0/Android/obb/com.example.app`
    */
   ObbDir = 'obbDir',

   /** App-private cache directory. Cleared on uninstall; may be purged by system.
    * `/data/user/0/com.example.app/cache`
    */
   CacheDir = 'cacheDir',

   /** App-private cache directory optimised for code/dex. May be purged by system.
    * `/data/user/0/com.example.app/code_cache`
    */
   CodeCacheDir = 'codeCacheDir',

   /** Primary external cache directory. May be purged by system or removed with media.
    * `/storage/emulated/0/Android/data/com.example.app/cache`
    */
   ExternalCacheDir = 'externalCacheDir',

   /** App-specific alarms directory on external storage.
    * `/storage/emulated/0/Android/data/com.example.app/files/Alarms`
    */
   ExternalFilesDirectoryAlarms = 'externalFilesDirectoryAlarms',

   /** App-specific audiobooks directory on external storage.
    * `/storage/emulated/0/Android/data/com.example.app/files/Audiobooks`
    */
   ExternalFilesDirectoryAudiobooks = 'externalFilesDirectoryAudiobooks',

   /** App-specific DCIM (camera) directory on external storage.
    * `/storage/emulated/0/Android/data/com.example.app/files/DCIM`
    */
   ExternalFilesDirectoryDcim = 'externalFilesDirectoryDcim',

   /** App-specific documents directory on external storage.
    * `/storage/emulated/0/Android/data/com.example.app/files/Documents`
    */
   ExternalFilesDirectoryDocuments = 'externalFilesDirectoryDocuments',

   /** App-specific downloads directory on external storage.
    * `/storage/emulated/0/Android/data/com.example.app/files/Download`
    */
   ExternalFilesDirectoryDownloads = 'externalFilesDirectoryDownloads',

   /** App-specific movies directory on external storage.
    * `/storage/emulated/0/Android/data/com.example.app/files/Movies`
    */
   ExternalFilesDirectoryMovies = 'externalFilesDirectoryMovies',

   /** App-specific music directory on external storage.
    * `/storage/emulated/0/Android/data/com.example.app/files/Music`
    */
   ExternalFilesDirectoryMusic = 'externalFilesDirectoryMusic',

   /** App-specific notifications directory on external storage.
    * `/storage/emulated/0/Android/data/com.example.app/files/Notifications`
    */
   ExternalFilesDirectoryNotifications = 'externalFilesDirectoryNotifications',

   /** App-specific pictures directory on external storage.
    * `/storage/emulated/0/Android/data/com.example.app/files/Pictures`
    */
   ExternalFilesDirectoryPictures = 'externalFilesDirectoryPictures',

   /** App-specific podcasts directory on external storage.
    * `/storage/emulated/0/Android/data/com.example.app/files/Podcasts`
    */
   ExternalFilesDirectoryPodcasts = 'externalFilesDirectoryPodcasts',
}

/**
 * Android path collections — methods returning multiple paths across storage volumes.
 *
 * @see https://developer.android.com/reference/android/content/Context
 */
export enum AndroidPathCollection {

   /** External cache directories across all storage volumes.
    * `[/storage/emulated/0/Android/data/com.example.app/cache, /storage/sdcard1/...]`
    */
   ExternalCacheDirs = 'externalCacheDirs',

   /** External files directories across all storage volumes.
    * `[/storage/emulated/0/Android/data/com.example.app/files, /storage/sdcard1/...]`
    */
   ExternalFilesDirs = 'externalFilesDirs',

   /** External media directories across all storage volumes (deprecated API 30+).
    * `[/storage/emulated/0/Android/media/com.example.app, /storage/sdcard1/...]`
    */
   ExternalMediaDirs = 'externalMediaDirs',

   /** OBB directories across all storage volumes.
    * `[/storage/emulated/0/Android/obb/com.example.app, /storage/sdcard1/...]`
    */
   ObbDirs = 'obbDirs',
}

/**
 * iOS sandbox directories.
 *
 * @see https://developer.apple.com/documentation/foundation/filemanager/searchpathdirectory/
 * @see https://developer.apple.com/documentation/foundation/filemanager#Accessing-user-directories
 */
export enum IosPath {

   /** Backed up, visible in Files app. Maps to SearchPathDirectory.documentDirectory.
    * `<sandbox>/Documents`
    */
   DocumentDirectory = 'documentDirectory',

   /** Contains app support, caches, preferences.
    * Maps to SearchPathDirectory.libraryDirectory.
    * `<sandbox>/Library`
    */
   LibraryDirectory = 'libraryDirectory',

   /** Not backed up, may be purged by system.
    * Maps to SearchPathDirectory.cachesDirectory.
    * `<sandbox>/Library/Caches`
    */
   CachesDirectory = 'cachesDirectory',

   /** Backed up, hidden from user.
    * Maps to SearchPathDirectory.applicationSupportDirectory.
    * `<sandbox>/Library/Application Support`
    */
   ApplicationSupportDirectory = 'applicationSupportDirectory',

   /** Autosaved document storage.
    * Maps to SearchPathDirectory.autosavedInformationDirectory.
    * `<sandbox>/Documents/Autosaved`
    */
   AutosavedInformationDirectory = 'autosavedInformationDirectory',

   /** User downloads. Maps to SearchPathDirectory.downloadsDirectory.
    * `<sandbox>/Downloads`
    */
   DownloadsDirectory = 'downloadsDirectory',

   /** Media storage. Maps to SearchPathDirectory.moviesDirectory.
    * `<sandbox>/Movies`
    */
   MoviesDirectory = 'moviesDirectory',

   /** Media storage. Maps to SearchPathDirectory.musicDirectory.
    * `<sandbox>/Music`
    */
   MusicDirectory = 'musicDirectory',

   /** Media storage. Maps to SearchPathDirectory.picturesDirectory.
    * `<sandbox>/Pictures`
    */
   PicturesDirectory = 'picturesDirectory',

   /** Used with FileManager.url(for:in:appropriateFor:create:)
    * for atomic safe-save operations. Not a fixed path.
    */
   ItemReplacementDirectory = 'itemReplacementDirectory',

   /** Not backed up, may be purged by system.
    * Maps to FileManager.temporaryDirectory.
    * `<sandbox>/tmp`
    */
   TemporaryDirectory = 'temporaryDirectory',

   /** The sandbox root directory for the app.
    * Maps to FileManager.homeDirectoryForCurrentUser.
    * `<sandbox>/`
    */
   HomeDirectory = 'homeDirectory',
}

/**
 * Linux XDG directory paths.
 *
 * Returns base directories per the XDG Base Directory Specification.
 * Apps may append their own identifier (e.g. `DataHome` + `/<app-id>/`).
 * `*ForCurrentApp` variants append the bundle identifier automatically.
 * Flatpak/Snap runtimes automatically remap XDG vars to sandbox paths.
 *
 * @see https://specifications.freedesktop.org/basedir-spec/latest/
 * @see https://www.freedesktop.org/wiki/Software/xdg-user-dirs/
 */
export enum LinuxPath {

   /** App-specific data: databases, user-generated content.
    * `$XDG_DATA_HOME` (`~/.local/share`)
    */
   DataHome = 'dataHome',

   /** Per-app data directory (Tauri app_data_dir).
    * `$XDG_DATA_HOME/<app-id>`
    *
    * @see https://specifications.freedesktop.org/basedir-spec/latest/#variables
    */
   DataHomeForCurrentApp = 'dataHomeForCurrentApp',

   /** App-specific configuration files.
    * `$XDG_CONFIG_HOME` (`~/.config`)
    */
   ConfigHome = 'configHome',

   /** Per-app configuration directory (Tauri app_config_dir).
    * `$XDG_CONFIG_HOME/<app-id>`
    *
    * @see https://specifications.freedesktop.org/basedir-spec/latest/#variables
    */
   ConfigHomeForCurrentApp = 'configHomeForCurrentApp',

   /** Disposable cache data (safe to delete).
    * `$XDG_CACHE_HOME` (`~/.cache`)
    */
   CacheHome = 'cacheHome',

   /** Per-app cache directory (Tauri app_cache_dir).
    * `$XDG_CACHE_HOME/<app-id>`
    *
    * @see https://specifications.freedesktop.org/basedir-spec/latest/#variables
    */
   CacheHomeForCurrentApp = 'cacheHomeForCurrentApp',

   /** Non-portable state: logs, undo history, session state.
    * `$XDG_STATE_HOME` (`~/.local/state`)
    */
   StateHome = 'stateHome',

   /** Per-app state directory for logs, history, and session data.
    * `$XDG_STATE_HOME/<app-id>`
    *
    * @see https://specifications.freedesktop.org/basedir-spec/latest/#variables
    */
   StateHomeForCurrentApp = 'stateHomeForCurrentApp',

   /** Ephemeral runtime files: sockets, named pipes. Lifetime bound to login session.
    * `$XDG_RUNTIME_DIR` (`/run/user/<uid>`, set by pam/systemd; no fallback)
    */
   RuntimeDir = 'runtimeDir',

   /** User home directory.
    * `$HOME` (`~`)
    */
   Home = 'home',

   /** User-specific executables.
    * `$XDG_BIN_HOME` or `~/.local/bin` (de-facto convention, not part of the XDG
    * spec).
    */
   ExecutableDir = 'executableDir',

   /** User-specific fonts.
    * `$XDG_DATA_HOME/fonts` (`~/.local/share/fonts`)
    */
   FontDir = 'fontDir',

   /** `~/Desktop` (from `user-dirs.dirs`) */
   DesktopDir = 'desktopDir',

   /** `~/Documents` (from `user-dirs.dirs`) */
   DocumentDir = 'documentDir',

   /** `~/Downloads` (from `user-dirs.dirs`) */
   DownloadDir = 'downloadDir',

   /** `~/Music` (from `user-dirs.dirs`) */
   MusicDir = 'musicDir',

   /** `~/Pictures` (from `user-dirs.dirs`) */
   PictureDir = 'pictureDir',

   /** `~/Videos` (from `user-dirs.dirs`) */
   VideoDir = 'videoDir',

   /** `~/Templates` (from `user-dirs.dirs`) */
   TemplateDir = 'templateDir',

   /** `~/Public` (from `user-dirs.dirs`) */
   PublicDir = 'publicDir',
}

/**
 * macOS SearchPathDirectory paths.
 *
 * @see https://developer.apple.com/documentation/foundation/filemanager/searchpathdirectory/
 */
export enum MacPath {

   /** Supported applications.
    * `/Applications`
    */
   ApplicationDirectory = 'applicationDirectory',

   /** Unsupported applications and demonstration versions.
    * `/Applications/Demos`
    */
   DemoApplicationDirectory = 'demoApplicationDirectory',

   /** Developer applications.
    * `/Developer/Applications`
    */
   DeveloperApplicationDirectory = 'developerApplicationDirectory',

   /** System and network administration applications.
    * `/Applications/Utilities`
    */
   AdminApplicationDirectory = 'adminApplicationDirectory',

   /** Various user-visible documentation, support, and configuration files.
    * `/Library`
    */
   LibraryDirectory = 'libraryDirectory',

   /** Developer resources.
    * `/Developer`
    */
   DeveloperDirectory = 'developerDirectory',

   /** User home directories.
    * `/Users`
    */
   UserDirectory = 'userDirectory',

   /** Documentation.
    * `/Library/Documentation`
    */
   DocumentationDirectory = 'documentationDirectory',

   /** User document directory.
    * `~/Documents`
    */
   DocumentDirectory = 'documentDirectory',

   /** Core services.
    * `/System/Library/CoreServices`
    */
   CoreServiceDirectory = 'coreServiceDirectory',

   /** The user's autosaved documents.
    * `~/Library/Autosave Information`
    */
   AutosavedInformationDirectory = 'autosavedInformationDirectory',

   /** The user's desktop directory.
    * `~/Desktop`
    */
   DesktopDirectory = 'desktopDirectory',

   /** Discardable cache files.
    * `~/Library/Caches`
    */
   CachesDirectory = 'cachesDirectory',

   /** Application support files.
    * `~/Library/Application Support`
    */
   ApplicationSupportDirectory = 'applicationSupportDirectory',

   /** Per-app persistent data directory under Application Support.
    * `~/Library/Application Support/<bundle-id>`
    *
    * @see https://developer.apple.com/documentation/foundation/filemanager/searchpathdirectory/applicationsupportdirectory
    * @see https://developer.apple.com/library/archive/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/MacOSXDirectories/MacOSXDirectories.html
    */
   ApplicationSupportDirectoryForCurrentApp = 'applicationSupportDirectoryForCurrentApp',

   /** Per-app discardable cache directory under the shared Caches folder.
    * `~/Library/Caches/<bundle-id>`
    *
    * @see https://developer.apple.com/documentation/foundation/filemanager/searchpathdirectory/cachesdirectory
    */
   CachesDirectoryForCurrentApp = 'cachesDirectoryForCurrentApp',

   /** The user's downloads directory.
    * `~/Downloads`
    */
   DownloadsDirectory = 'downloadsDirectory',

   /** Input methods.
    * `~/Library/Input Methods`
    */
   InputMethodsDirectory = 'inputMethodsDirectory',

   /** The user's Movies directory.
    * `~/Movies`
    */
   MoviesDirectory = 'moviesDirectory',

   /** The user's Music directory.
    * `~/Music`
    */
   MusicDirectory = 'musicDirectory',

   /** The user's Pictures directory.
    * `~/Pictures`
    */
   PicturesDirectory = 'picturesDirectory',

   /** The system's PPDs directory.
    * `/Library/Printers/PPDs`
    */
   PrinterDescriptionDirectory = 'printerDescriptionDirectory',

   /** The user's Public sharing directory.
    * `~/Public`
    */
   SharedPublicDirectory = 'sharedPublicDirectory',

   /** The PreferencePanes directory for use with System Preferences.
    * `~/Library/PreferencePanes`
    */
   PreferencePanesDirectory = 'preferencePanesDirectory',

   /** The user scripts folder for the calling application.
    * `~/Library/Application Scripts/<code-signing-id>`
    */
   ApplicationScriptsDirectory = 'applicationScriptsDirectory',

   /** Used with url(for:in:appropriateFor:create:) for atomic safe-save operations.
    * Not a fixed path.
    */
   ItemReplacementDirectory = 'itemReplacementDirectory',

   /** All directories where applications can be stored.
    * `(/Applications, ~/Applications, /Network/Applications)`
    */
   AllApplicationsDirectory = 'allApplicationsDirectory',

   /** All directories where resources can be stored.
    * `(/Library, ~/Library, /Network/Library)`
    */
   AllLibrariesDirectory = 'allLibrariesDirectory',

   /** The trash directory.
    * `~/.Trash`
    */
   TrashDirectory = 'trashDirectory',
}

/**
 * Win32 known folder paths for unpackaged applications.
 *
 * @see https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid
 */
export enum Win32Path {

   /** User account pictures.
    * `%APPDATA%\Microsoft\Windows\AccountPictures`
    */
   AccountPictures = 'accountPictures',

   /** User administrative tools shortcuts.
    * `%APPDATA%\Microsoft\Windows\Start Menu\Programs\Administrative Tools`
    */
   AdminTools = 'adminTools',

   /** Per-user application shortcuts for pinning.
    * `%LOCALAPPDATA%\Microsoft\Windows\Application Shortcuts`
    */
   ApplicationShortcuts = 'applicationShortcuts',

   /** Camera roll photos.
    * `%USERPROFILE%\Pictures\Camera Roll`
    */
   CameraRoll = 'cameraRoll',

   /** Staging area for burning CDs/DVDs.
    * `%LOCALAPPDATA%\Microsoft\Windows\Burn\Burn`
    */
   CdBurning = 'cdBurning',

   /** System-wide administrative tools shortcuts.
    * `%ALLUSERSPROFILE%\Microsoft\Windows\Start Menu\Programs\Administrative Tools`
    */
   CommonAdminTools = 'commonAdminTools',

   /** OEM links visible in the Computer folder.
    * `%ALLUSERSPROFILE%\OEM Links`
    */
   CommonOemLinks = 'commonOemLinks',

   /** System-wide Start Menu programs.
    * `%ALLUSERSPROFILE%\Microsoft\Windows\Start Menu\Programs`
    */
   CommonPrograms = 'commonPrograms',

   /** System-wide Start Menu root.
    * `%ALLUSERSPROFILE%\Microsoft\Windows\Start Menu`
    */
   CommonStartMenu = 'commonStartMenu',

   /** System-wide startup programs.
    * `%ALLUSERSPROFILE%\Microsoft\Windows\Start Menu\Programs\StartUp`
    */
   CommonStartup = 'commonStartup',

   /** System-wide document templates.
    * `%ALLUSERSPROFILE%\Microsoft\Windows\Templates`
    */
   CommonTemplates = 'commonTemplates',

   /** User contacts.
    * `%USERPROFILE%\Contacts`
    */
   Contacts = 'contacts',

   /** Internet cookies.
    * `%APPDATA%\Microsoft\Windows\Cookies`
    */
   Cookies = 'cookies',

   /** User desktop.
    * `%USERPROFILE%\Desktop`
    */
   Desktop = 'desktop',

   /** Device metadata store.
    * `%ALLUSERSPROFILE%\Microsoft\Windows\DeviceMetadataStore`
    */
   DeviceMetadataStore = 'deviceMetadataStore',

   /** User documents.
    * `%USERPROFILE%\Documents`
    */
   Documents = 'documents',

   /** Documents library definition.
    * `%APPDATA%\Microsoft\Windows\Libraries\Documents.library-ms`
    */
   DocumentsLibrary = 'documentsLibrary',

   /** User downloads.
    * `%USERPROFILE%\Downloads`
    */
   Downloads = 'downloads',

   /** Internet Explorer favorites.
    * `%USERPROFILE%\Favorites`
    */
   Favorites = 'favorites',

   /** System fonts.
    * `%windir%\Fonts`
    */
   Fonts = 'fonts',

   /** Game Explorer data.
    * `%LOCALAPPDATA%\Microsoft\Windows\GameExplorer`
    */
   GameTasks = 'gameTasks',

   /** Browser history.
    * `%LOCALAPPDATA%\Microsoft\Windows\History`
    */
   History = 'history',

   /** Implicit app shortcuts for the jump list.
    * `%APPDATA%\Microsoft\Internet Explorer\Quick Launch\`
    * `User Pinned\ImplicitAppShortcuts`
    */
   ImplicitAppShortcuts = 'implicitAppShortcuts',

   /** Temporary internet files cache.
    * `%LOCALAPPDATA%\Microsoft\Windows\Temporary Internet Files`
    */
   InternetCache = 'internetCache',

   /** Windows libraries root.
    * `%APPDATA%\Microsoft\Windows\Libraries`
    */
   Libraries = 'libraries',

   /** User links / favorites in Explorer navigation pane.
    * `%USERPROFILE%\Links`
    */
   Links = 'links',

   /** Per-user local application data.
    * `%LOCALAPPDATA% (%USERPROFILE%\AppData\Local)`
    */
   LocalAppData = 'localAppData',

   /** Per-app local data directory (Tauri app_local_data_dir).
    * `%LOCALAPPDATA%/<app-id>`
    *
    * @see https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid
    */
   LocalAppDataForCurrentApp = 'localAppDataForCurrentApp',

   /** Per-user low-integrity application data.
    * `%USERPROFILE%\AppData\LocalLow`
    */
   LocalAppDataLow = 'localAppDataLow',

   /** Localized resource directory.
    * `%windir%\resources\0409 (code page)`
    */
   LocalizedResourcesDir = 'localizedResourcesDir',

   /** User music.
    * `%USERPROFILE%\Music`
    */
   Music = 'music',

   /** Music library definition.
    * `%APPDATA%\Microsoft\Windows\Libraries\Music.library-ms`
    */
   MusicLibrary = 'musicLibrary',

   /** Network shortcuts (NetHood).
    * `%APPDATA%\Microsoft\Windows\Network Shortcuts`
    */
   NetworkShortcuts = 'networkShortcuts',

   /** 3D Objects folder.
    * `%USERPROFILE%\3D Objects`
    */
   Objects3D = 'objects3D',

   /** Photo Gallery original images.
    * `%LOCALAPPDATA%\Microsoft\Windows Photo Gallery\Original Images`
    */
   OriginalImages = 'originalImages',

   /** Photo album slide shows.
    * `%USERPROFILE%\Pictures\Slide Shows`
    */
   PhotoAlbums = 'photoAlbums',

   /** Pictures library definition.
    * `%APPDATA%\Microsoft\Windows\Libraries\Pictures.library-ms`
    */
   PicturesLibrary = 'picturesLibrary',

   /** User pictures.
    * `%USERPROFILE%\Pictures`
    */
   Pictures = 'pictures',

   /** Music playlists.
    * `%USERPROFILE%\Music\Playlists`
    */
   Playlists = 'playlists',

   /** Printer shortcuts (PrintHood).
    * `%APPDATA%\Microsoft\Windows\Printer Shortcuts`
    */
   PrintHood = 'printHood',

   /** User profile root.
    * `%USERPROFILE% (%SystemDrive%\Users\%USERNAME%)`
    */
   Profile = 'profile',

   /** Machine-wide application data.
    * `%ALLUSERSPROFILE% (%ProgramData%, %SystemDrive%\ProgramData)`
    */
   ProgramData = 'programData',

   /** Program Files directory.
    * `%ProgramFiles% (%SystemDrive%\Program Files)`
    */
   ProgramFiles = 'programFiles',

   /** Program Files directory (64-bit).
    * `%ProgramFiles% (%SystemDrive%\Program Files)`
    */
   ProgramFilesX64 = 'programFilesX64',

   /** Program Files directory (32-bit on 64-bit OS).
    * `%ProgramFiles(x86)% (%SystemDrive%\Program Files (x86))`
    */
   ProgramFilesX86 = 'programFilesX86',

   /** Common Files directory.
    * `%ProgramFiles%\Common Files`
    */
   ProgramFilesCommon = 'programFilesCommon',

   /** Common Files directory (64-bit).
    * `%ProgramFiles%\Common Files`
    */
   ProgramFilesCommonX64 = 'programFilesCommonX64',

   /** Common Files directory (32-bit on 64-bit OS).
    * `%ProgramFiles(x86)%\Common Files`
    */
   ProgramFilesCommonX86 = 'programFilesCommonX86',

   /** User Start Menu programs.
    * `%APPDATA%\Microsoft\Windows\Start Menu\Programs`
    */
   Programs = 'programs',

   /** Public user profile root.
    * `%PUBLIC% (%SystemDrive%\Users\Public)`
    */
   Public = 'public',

   /** Public desktop.
    * `%PUBLIC%\Desktop`
    */
   PublicDesktop = 'publicDesktop',

   /** Public documents.
    * `%PUBLIC%\Documents`
    */
   PublicDocuments = 'publicDocuments',

   /** Public downloads.
    * `%PUBLIC%\Downloads`
    */
   PublicDownloads = 'publicDownloads',

   /** Public game explorer data.
    * `%ALLUSERSPROFILE%\Microsoft\Windows\GameExplorer`
    */
   PublicGameTasks = 'publicGameTasks',

   /** Public libraries root.
    * `%ALLUSERSPROFILE%\Microsoft\Windows\Libraries`
    */
   PublicLibraries = 'publicLibraries',

   /** Public music.
    * `%PUBLIC%\Music`
    */
   PublicMusic = 'publicMusic',

   /** Public pictures.
    * `%PUBLIC%\Pictures`
    */
   PublicPictures = 'publicPictures',

   /** Public ringtones.
    * `%ALLUSERSPROFILE%\Microsoft\Windows\Ringtones`
    */
   PublicRingtones = 'publicRingtones',

   /** Public account pictures.
    * `%PUBLIC%\AccountPictures`
    */
   PublicUserTiles = 'publicUserTiles',

   /** Public videos.
    * `%PUBLIC%\Videos`
    */
   PublicVideos = 'publicVideos',

   /** Quick Launch toolbar shortcuts.
    * `%APPDATA%\Microsoft\Internet Explorer\Quick Launch`
    */
   QuickLaunch = 'quickLaunch',

   /** Recently used files.
    * `%APPDATA%\Microsoft\Windows\Recent`
    */
   Recent = 'recent',

   /** Recorded TV library definition.
    * `%PUBLIC%\RecordedTV.library-ms`
    */
   RecordedTVLibrary = 'recordedTVLibrary',

   /** System resources root.
    * `%windir%\Resources`
    */
   ResourceDir = 'resourceDir',

   /** User ringtones.
    * `%LOCALAPPDATA%\Microsoft\Windows\Ringtones`
    */
   Ringtones = 'ringtones',

   /** Per-user roaming application data.
    * `%APPDATA% (%USERPROFILE%\AppData\Roaming)`
    */
   RoamingAppData = 'roamingAppData',

   /** Per-app roaming data directory (Tauri app_data_dir / app_config_dir).
    * `%APPDATA%/<app-id>`
    *
    * @see https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid
    */
   RoamingAppDataForCurrentApp = 'roamingAppDataForCurrentApp',

   /** Roamed tile images for Start.
    * `%LOCALAPPDATA%\Microsoft\Windows\RoamedTileImages`
    */
   RoamedTileImages = 'roamedTileImages',

   /** Roaming tile data for Start.
    * `%LOCALAPPDATA%\Microsoft\Windows\RoamingTiles`
    */
   RoamingTiles = 'roamingTiles',

   /** Sample music files.
    * `%PUBLIC%\Music\Sample Music`
    */
   SampleMusic = 'sampleMusic',

   /** Sample picture files.
    * `%PUBLIC%\Pictures\Sample Pictures`
    */
   SamplePictures = 'samplePictures',

   /** Sample playlist files.
    * `%PUBLIC%\Music\Sample Playlists`
    */
   SamplePlaylists = 'samplePlaylists',

   /** Sample video files.
    * `%PUBLIC%\Videos\Sample Videos`
    */
   SampleVideos = 'sampleVideos',

   /** User saved games.
    * `%USERPROFILE%\Saved Games`
    */
   SavedGames = 'savedGames',

   /** User saved pictures.
    * `%USERPROFILE%\Pictures\Saved Pictures`
    */
   SavedPictures = 'savedPictures',

   /** Saved pictures library definition.
    * `%APPDATA%\Microsoft\Windows\Libraries\SavedPictures.library-ms`
    */
   SavedPicturesLibrary = 'savedPicturesLibrary',

   /** Saved search queries.
    * `%USERPROFILE%\Searches`
    */
   Searches = 'searches',

   /** User screenshots.
    * `%USERPROFILE%\Pictures\Screenshots`
    */
   Screenshots = 'screenshots',

   /** Connected search history.
    * `%LOCALAPPDATA%\Microsoft\Windows\ConnectedSearch\History`
    */
   SearchHistory = 'searchHistory',

   /** Connected search templates.
    * `%LOCALAPPDATA%\Microsoft\Windows\ConnectedSearch\Templates`
    */
   SearchTemplates = 'searchTemplates',

   /** "Send To" context menu targets.
    * `%APPDATA%\Microsoft\Windows\SendTo`
    */
   SendTo = 'sendTo',

   /** Default sidebar gadgets (Windows 7).
    * `%ProgramFiles%\Windows Sidebar\Gadgets`
    */
   SidebarDefaultParts = 'sidebarDefaultParts',

   /** User-installed sidebar gadgets (Windows 7).
    * `%LOCALAPPDATA%\Microsoft\Windows Sidebar\Gadgets`
    */
   SidebarParts = 'sidebarParts',

   /** OneDrive root.
    * `%USERPROFILE%\OneDrive`
    */
   SkyDrive = 'skyDrive',

   /** OneDrive camera roll.
    * `%USERPROFILE%\OneDrive\Pictures\Camera Roll`
    */
   SkyDriveCameraRoll = 'skyDriveCameraRoll',

   /** OneDrive documents.
    * `%USERPROFILE%\OneDrive\Documents`
    */
   SkyDriveDocuments = 'skyDriveDocuments',

   /** OneDrive pictures.
    * `%USERPROFILE%\OneDrive\Pictures`
    */
   SkyDrivePictures = 'skyDrivePictures',

   /** User Start Menu root.
    * `%APPDATA%\Microsoft\Windows\Start Menu`
    */
   StartMenu = 'startMenu',

   /** User startup programs.
    * `%APPDATA%\Microsoft\Windows\Start Menu\Programs\StartUp`
    */
   Startup = 'startup',

   /** System32 directory.
    * `%windir%\system32`
    */
   System32 = 'system32',

   /** System32 directory (32-bit on 64-bit OS).
    * `%windir%\system32`
    */
   SystemX86 = 'systemX86',

   /** User document templates.
    * `%APPDATA%\Microsoft\Windows\Templates`
    */
   Templates = 'templates',

   /** User-pinned taskbar and Start items.
    * `%APPDATA%\Microsoft\Internet Explorer\Quick Launch\User Pinned`
    */
   UserPinned = 'userPinned',

   /** Users root directory.
    * `%SystemDrive%\Users`
    */
   Users = 'users',

   /** Per-user program installations.
    * `%LOCALAPPDATA%\Programs`
    */
   UserProgramFiles = 'userProgramFiles',

   /** Per-user common program files.
    * `%LOCALAPPDATA%\Programs\Common`
    */
   UserProgramFilesCommon = 'userProgramFilesCommon',

   /** User videos.
    * `%USERPROFILE%\Videos`
    */
   Videos = 'videos',

   /** Video library definition.
    * `%APPDATA%\Microsoft\Windows\Libraries\Videos.library-ms`
    */
   VideoLibrary = 'videoLibrary',

   /** Windows installation root.
    * `%windir%`
    */
   Windows = 'windows',
}

/**
 * Windows ApplicationData paths for packaged applications.
 *
 * @see https://learn.microsoft.com/en-us/uwp/api/windows.storage.applicationdata?view=winrt-28000#properties
 */
export enum WindowsApplicationDataPath {

   /** App-specific local cache. Not roamed, not backed up; may be purged by system.
    * `C:\Users\<user>\AppData\Local\Packages\<id>\LocalCache`
    */
   LocalCacheFolder = 'localCacheFolder',

   /** App-specific local data. Not roamed; backed up if configured.
    * `C:\Users\<user>\AppData\Local\Packages\<id>\LocalState`
    */
   LocalFolder = 'localFolder',

   /** App-specific roaming data. Synced across devices via the user's Microsoft account.
    * `C:\Users\<user>\AppData\Local\Packages\<id>\RoamingState`
    */
   RoamingFolder = 'roamingFolder',

   /** App data shared between all users of the machine for this package.
    * `C:\ProgramData\Packages\<id>\LocalCache`
    */
   SharedLocalFolder = 'sharedLocalFolder',

   /** App-specific temporary files. May be purged by system at any time.
    * `C:\Users\<user>\AppData\Local\Packages\<id>\TempState`
    */
   TemporaryFolder = 'temporaryFolder',
}
