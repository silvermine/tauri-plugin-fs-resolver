package org.silvermine.plugin.fs_resolver

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

// These are camel case for ease of conversion with the Rust enum.
enum class AndroidPath {
   dataDir,
   filesDir,
   noBackupFilesDir,
   obbDir,
   cacheDir,
   codeCacheDir,
   externalCacheDir,
   externalFilesDirectoryAlarms,
   externalFilesDirectoryAudiobooks,
   externalFilesDirectoryDcim,
   externalFilesDirectoryDocuments,
   externalFilesDirectoryDownloads,
   externalFilesDirectoryMovies,
   externalFilesDirectoryMusic,
   externalFilesDirectoryNotifications,
   externalFilesDirectoryPictures,
   externalFilesDirectoryPodcasts,
}

// These are camel case for ease of conversion with the Rust enum.
enum class AndroidPathCollection {
   externalCacheDirs,
   externalFilesDirs,
   externalMediaDirs,
   obbDirs,
}

class AndroidPathResolutionArgs {
   lateinit var path: AndroidPath
}

class AndroidPathCollectionResolutionArgs {
   lateinit var collection: AndroidPathCollection
}

@TauriPlugin
class FsResolverPlugin(private val activity: Activity): Plugin(activity) {

   private val context = activity.applicationContext

   @Command
   fun resolveDirectory(invoke: Invoke) {
      val args = invoke.parseArgs(AndroidPathResolutionArgs::class.java)

      try {
         val resolved = resolvePath(args.path)
         val ret = JSObject()
         ret.put("path", resolved)
         invoke.resolve(ret)
      } catch (t: Throwable) {
         invoke.reject(t.message ?: "Unknown error")
      }
   }

   @Command
   fun resolveDirectoryCollection(invoke: Invoke) {
      val args = invoke.parseArgs(AndroidPathCollectionResolutionArgs::class.java)

      try {
         val resolved = resolvePathCollection(args.collection)
         val ret = JSObject()
         ret.put("paths", org.json.JSONArray(resolved))
         invoke.resolve(ret)
      } catch (t: Throwable) {
         invoke.reject(t.message ?: "Unknown error")
      }
   }

   fun resolvePath(path: AndroidPath): String {
      return when (path) {
         AndroidPath.dataDir -> context.dataDir.absolutePath
         AndroidPath.filesDir -> context.filesDir.absolutePath
         AndroidPath.noBackupFilesDir -> context.noBackupFilesDir.absolutePath
         AndroidPath.obbDir -> context.obbDir.absolutePath
         AndroidPath.cacheDir -> context.cacheDir.absolutePath
         AndroidPath.codeCacheDir -> context.codeCacheDir.absolutePath
         AndroidPath.externalCacheDir -> context.externalCacheDir?.absolutePath
               ?: throw Error("externalCacheDir unavailable")
         AndroidPath.externalFilesDirectoryAlarms ->
               context.getExternalFilesDir(android.os.Environment.DIRECTORY_ALARMS)?.absolutePath
                  ?: throw Error("externalFilesDir(Alarms) unavailable")
         AndroidPath.externalFilesDirectoryAudiobooks ->
               context.getExternalFilesDir(android.os.Environment.DIRECTORY_AUDIOBOOKS)?.absolutePath
                  ?: throw Error("externalFilesDir(Audiobooks) unavailable")
         AndroidPath.externalFilesDirectoryDcim ->
               context.getExternalFilesDir(android.os.Environment.DIRECTORY_DCIM)?.absolutePath
                  ?: throw Error("externalFilesDir(DCIM) unavailable")
         AndroidPath.externalFilesDirectoryDocuments ->
               context.getExternalFilesDir(android.os.Environment.DIRECTORY_DOCUMENTS)?.absolutePath
                  ?: throw Error("externalFilesDir(Documents) unavailable")
         AndroidPath.externalFilesDirectoryDownloads ->
               context.getExternalFilesDir(android.os.Environment.DIRECTORY_DOWNLOADS)?.absolutePath
                  ?: throw Error("externalFilesDir(Downloads) unavailable")
         AndroidPath.externalFilesDirectoryMovies ->
               context.getExternalFilesDir(android.os.Environment.DIRECTORY_MOVIES)?.absolutePath
                  ?: throw Error("externalFilesDir(Movies) unavailable")
         AndroidPath.externalFilesDirectoryMusic ->
               context.getExternalFilesDir(android.os.Environment.DIRECTORY_MUSIC)?.absolutePath
                  ?: throw Error("externalFilesDir(Music) unavailable")
         AndroidPath.externalFilesDirectoryNotifications ->
               context.getExternalFilesDir(android.os.Environment.DIRECTORY_NOTIFICATIONS)?.absolutePath
                  ?: throw Error("externalFilesDir(Notifications) unavailable")
         AndroidPath.externalFilesDirectoryPictures ->
               context.getExternalFilesDir(android.os.Environment.DIRECTORY_PICTURES)?.absolutePath
                  ?: throw Error("externalFilesDir(Pictures) unavailable")
         AndroidPath.externalFilesDirectoryPodcasts ->
               context.getExternalFilesDir(android.os.Environment.DIRECTORY_PODCASTS)?.absolutePath
                  ?: throw Error("externalFilesDir(Podcasts) unavailable")
      }
   }

   fun resolvePathCollection(collection: AndroidPathCollection): List<String> {
      return when (collection) {
         AndroidPathCollection.externalCacheDirs ->
               context.externalCacheDirs.mapNotNull { it?.absolutePath }
         AndroidPathCollection.externalFilesDirs ->
               context.getExternalFilesDirs(null).mapNotNull { it?.absolutePath }
         AndroidPathCollection.externalMediaDirs ->
               @Suppress("DEPRECATION")
               context.externalMediaDirs.mapNotNull { it?.absolutePath }
         AndroidPathCollection.obbDirs ->
               context.obbDirs.mapNotNull { it?.absolutePath }
      }
   }
}
