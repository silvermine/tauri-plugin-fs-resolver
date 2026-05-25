use serde::Deserialize;
use std::fmt::Display;

// https://developer.android.com/reference/android/content/Context
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AndroidPath {
   // App-private data directory. Cleared on uninstall.
   // /data/user/0/com.example.app
   // https://developer.android.com/reference/android/content/Context#getDataDir()
   DataDir,

   // App-private files directory. Cleared on uninstall.
   // /data/user/0/com.example.app/files
   // https://developer.android.com/reference/android/content/Context#getFilesDir()
   FilesDir,

   // App-private files excluded from auto-backup. Cleared on uninstall.
   // /data/user/0/com.example.app/no_backup
   // https://developer.android.com/reference/android/content/Context#getNoBackupFilesDir()
   NoBackupFilesDir,

   // Primary OBB (expansion file) directory for the app.
   // /storage/emulated/0/Android/obb/com.example.app
   // https://developer.android.com/reference/android/content/Context#getObbDir()
   ObbDir,

   // App-private cache directory. Cleared on uninstall; may be purged by system.
   // /data/user/0/com.example.app/cache
   // https://developer.android.com/reference/android/content/Context#getCacheDir()
   CacheDir,

   // App-private cache directory optimised for code/dex. May be purged by system.
   // /data/user/0/com.example.app/code_cache
   // https://developer.android.com/reference/android/content/Context#getCodeCacheDir()
   CodeCacheDir,

   // Primary external cache directory. May be purged by system or removed with media.
   // /storage/emulated/0/Android/data/com.example.app/cache
   // https://developer.android.com/reference/android/content/Context#getExternalCacheDir()
   ExternalCacheDir,

   // The following ExternalFilesDirectory* values map to getExternalFilesDir(type)
   // with the corresponding Environment.DIRECTORY_* constant.
   // https://developer.android.com/reference/android/content/Context#getExternalFilesDir(java.lang.String)
   // https://developer.android.com/reference/android/os/Environment#fields_1

   // App-specific alarms directory on external storage.
   // /storage/emulated/0/Android/data/com.example.app/files/Alarms
   ExternalFilesDirectoryAlarms,

   // App-specific audiobooks directory on external storage.
   // /storage/emulated/0/Android/data/com.example.app/files/Audiobooks
   ExternalFilesDirectoryAudiobooks,

   // App-specific DCIM (camera) directory on external storage.
   // /storage/emulated/0/Android/data/com.example.app/files/DCIM
   ExternalFilesDirectoryDcim,

   // App-specific documents directory on external storage.
   // /storage/emulated/0/Android/data/com.example.app/files/Documents
   ExternalFilesDirectoryDocuments,

   // App-specific downloads directory on external storage.
   // /storage/emulated/0/Android/data/com.example.app/files/Download
   ExternalFilesDirectoryDownloads,

   // App-specific movies directory on external storage.
   // /storage/emulated/0/Android/data/com.example.app/files/Movies
   ExternalFilesDirectoryMovies,

   // App-specific music directory on external storage.
   // /storage/emulated/0/Android/data/com.example.app/files/Music
   ExternalFilesDirectoryMusic,

   // App-specific notifications directory on external storage.
   // /storage/emulated/0/Android/data/com.example.app/files/Notifications
   ExternalFilesDirectoryNotifications,

   // App-specific pictures directory on external storage.
   // /storage/emulated/0/Android/data/com.example.app/files/Pictures
   ExternalFilesDirectoryPictures,

   // App-specific podcasts directory on external storage.
   // /storage/emulated/0/Android/data/com.example.app/files/Podcasts
   ExternalFilesDirectoryPodcasts,
}

impl Display for AndroidPath {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
         AndroidPath::DataDir => write!(f, "dataDir"),
         AndroidPath::FilesDir => write!(f, "filesDir"),
         AndroidPath::NoBackupFilesDir => write!(f, "noBackupFilesDir"),
         AndroidPath::ObbDir => write!(f, "obbDir"),
         AndroidPath::CacheDir => write!(f, "cacheDir"),
         AndroidPath::CodeCacheDir => write!(f, "codeCacheDir"),
         AndroidPath::ExternalCacheDir => write!(f, "externalCacheDir"),
         AndroidPath::ExternalFilesDirectoryAlarms => write!(f, "externalFilesDirectoryAlarms"),
         AndroidPath::ExternalFilesDirectoryAudiobooks => {
            write!(f, "externalFilesDirectoryAudiobooks")
         }
         AndroidPath::ExternalFilesDirectoryDcim => write!(f, "externalFilesDirectoryDcim"),
         AndroidPath::ExternalFilesDirectoryDocuments => {
            write!(f, "externalFilesDirectoryDocuments")
         }
         AndroidPath::ExternalFilesDirectoryDownloads => {
            write!(f, "externalFilesDirectoryDownloads")
         }
         AndroidPath::ExternalFilesDirectoryMovies => write!(f, "externalFilesDirectoryMovies"),
         AndroidPath::ExternalFilesDirectoryMusic => write!(f, "externalFilesDirectoryMusic"),
         AndroidPath::ExternalFilesDirectoryNotifications => {
            write!(f, "externalFilesDirectoryNotifications")
         }
         AndroidPath::ExternalFilesDirectoryPictures => write!(f, "externalFilesDirectoryPictures"),
         AndroidPath::ExternalFilesDirectoryPodcasts => write!(f, "externalFilesDirectoryPodcasts"),
      }
   }
}

// These map to methods that provide a collection of paths, rather than a single path.
// https://developer.android.com/reference/android/content/Context
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AndroidPathCollection {
   // External cache directories across all storage volumes.
   // [/storage/emulated/0/Android/data/com.example.app/cache, /storage/sdcard1/Android/data/com.example.app/cache]
   // https://developer.android.com/reference/android/content/Context#getExternalCacheDirs()
   ExternalCacheDirs,

   // External files directories across all storage volumes.
   // [/storage/emulated/0/Android/data/com.example.app/files, /storage/sdcard1/Android/data/com.example.app/files]
   // https://developer.android.com/reference/android/content/Context#getExternalFilesDirs(java.lang.String)
   ExternalFilesDirs,

   // External media directories across all storage volumes (deprecated API 30+).
   // [/storage/emulated/0/Android/media/com.example.app, /storage/sdcard1/Android/media/com.example.app]
   // https://developer.android.com/reference/android/content/Context#getExternalMediaDirs()
   ExternalMediaDirs,

   // OBB directories across all storage volumes.
   // [/storage/emulated/0/Android/obb/com.example.app, /storage/sdcard1/Android/obb/com.example.app]
   // https://developer.android.com/reference/android/content/Context#getObbDirs()
   ObbDirs,
}

impl Display for AndroidPathCollection {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
         AndroidPathCollection::ExternalCacheDirs => write!(f, "externalCacheDirs"),
         AndroidPathCollection::ExternalFilesDirs => write!(f, "externalFilesDirs"),
         AndroidPathCollection::ExternalMediaDirs => write!(f, "externalMediaDirs"),
         AndroidPathCollection::ObbDirs => write!(f, "obbDirs"),
      }
   }
}
