use crate::Win32Path;
use crate::WindowsApplicationDataPath;
use crate::error::Error;
use crate::error::Result;
use crate::windows_paths::WindowsPath;
use std::path::PathBuf;

pub(crate) fn resolve_windows_path(path: &WindowsPath) -> Result<PathBuf> {
   match path {
      WindowsPath::Win32(path) => resolve_win32_path(path),
      WindowsPath::WinMsix(path) => resolve_winmsix_path(path),
   }
}

/// Resolves MSIX app-container paths via the WinRT ApplicationData API.
///
/// MSIX-packaged apps (e.g. Microsoft Store) run in a virtualized filesystem where
/// standard Win32 known-folder paths are redirected. ApplicationData::Current() returns
/// the real sandboxed locations (e.g. `AppData\Local\Packages\<id>\LocalState`).
///
/// Fails with IncorrectOS on non-Windows, or InvalidPath if the app is not running
/// in an MSIX package context (ApplicationData is unavailable outside app containers).
fn resolve_winmsix_path(path: &WindowsApplicationDataPath) -> Result<PathBuf> {
   #[cfg(target_os = "windows")]
   {
      resolve_winmsix_path_inner(path)
   }

   #[cfg(not(target_os = "windows"))]
   {
      Err(Error::IncorrectOS {
         path: path.to_string(),
         current_os: std::env::consts::OS.to_string(),
         expected_os: "windows".to_string(),
      })
   }
}

#[cfg(target_os = "windows")]
fn resolve_winmsix_path_inner(path: &WindowsApplicationDataPath) -> Result<PathBuf> {
   use std::ffi::OsString;
   use std::os::windows::ffi::OsStringExt;

   // ApplicationData::Current() requires the process to be running inside an MSIX
   // app container. Will return E_NOT_SET / 0x80070490 if not packaged.
   let app_data = get_application_data()?;

   let folder = match path {
      WindowsApplicationDataPath::LocalCacheFolder => app_data.LocalCacheFolder(),
      WindowsApplicationDataPath::LocalFolder => app_data.LocalFolder(),
      WindowsApplicationDataPath::RoamingFolder => app_data.RoamingFolder(),
      WindowsApplicationDataPath::SharedLocalFolder => app_data.SharedLocalFolder(),
      WindowsApplicationDataPath::TemporaryFolder => app_data.TemporaryFolder(),
   }
   .map_err(|e| Error::InvalidPath(format!("Failed to get folder for {path}: {e}")))?;

   // StorageFolder::Path() returns the absolute filesystem path as an HSTRING.
   let hpath = folder
      .Path()
      .map_err(|e| Error::InvalidPath(format!("Failed to get path for {path}: {e}")))?;

   // Use from_wide (lossless) to stay consistent with the Win32 branch rather than
   // to_string_lossy(), which would replace unpaired surrogates with U+FFFD.
   Ok(PathBuf::from(OsString::from_wide(&hpath)))
}

/// Resolves Win32 known-folder paths via SHGetKnownFolderPath.
///
/// Each Win32Path variant maps 1:1 to a KNOWNFOLDERID GUID. This is the standard
/// resolution mechanism for MSI-packaged (non-sandboxed) Win32 desktop applications.
/// Implementation mirrors the pattern used by the `dirs-sys` crate.
///
/// See: https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid
fn resolve_win32_path(path: &Win32Path) -> Result<PathBuf> {
   #[cfg(target_os = "windows")]
   {
      resolve_win32_path_inner(path)
   }

   #[cfg(not(target_os = "windows"))]
   {
      Err(Error::IncorrectOS {
         path: path.to_string(),
         current_os: std::env::consts::OS.to_string(),
         expected_os: "windows".to_string(),
      })
   }
}

#[cfg(target_os = "windows")]
fn resolve_win32_path_inner(path: &Win32Path) -> Result<PathBuf> {
   use std::ffi::OsString;
   use std::ffi::c_void;
   use std::os::windows::ffi::OsStringExt;
   use std::slice;

   use windows_sys::Win32::Globalization::lstrlenW;
   use windows_sys::Win32::System::Com::CoTaskMemFree;
   use windows_sys::Win32::UI::Shell::{
      FOLDERID_AccountPictures, FOLDERID_AdminTools, FOLDERID_ApplicationShortcuts,
      FOLDERID_CDBurning, FOLDERID_CameraRoll, FOLDERID_CommonAdminTools, FOLDERID_CommonOEMLinks,
      FOLDERID_CommonPrograms, FOLDERID_CommonStartMenu, FOLDERID_CommonStartup,
      FOLDERID_CommonTemplates, FOLDERID_Contacts, FOLDERID_Cookies, FOLDERID_Desktop,
      FOLDERID_DeviceMetadataStore, FOLDERID_Documents, FOLDERID_DocumentsLibrary,
      FOLDERID_Downloads, FOLDERID_Favorites, FOLDERID_Fonts, FOLDERID_GameTasks, FOLDERID_History,
      FOLDERID_ImplicitAppShortcuts, FOLDERID_InternetCache, FOLDERID_Libraries, FOLDERID_Links,
      FOLDERID_LocalAppData, FOLDERID_LocalAppDataLow, FOLDERID_LocalizedResourcesDir,
      FOLDERID_Music, FOLDERID_MusicLibrary, FOLDERID_NetHood, FOLDERID_Objects3D,
      FOLDERID_OriginalImages, FOLDERID_PhotoAlbums, FOLDERID_Pictures, FOLDERID_PicturesLibrary,
      FOLDERID_Playlists, FOLDERID_PrintHood, FOLDERID_Profile, FOLDERID_ProgramData,
      FOLDERID_ProgramFiles, FOLDERID_ProgramFilesCommon, FOLDERID_ProgramFilesCommonX64,
      FOLDERID_ProgramFilesCommonX86, FOLDERID_ProgramFilesX64, FOLDERID_ProgramFilesX86,
      FOLDERID_Programs, FOLDERID_Public, FOLDERID_PublicDesktop, FOLDERID_PublicDocuments,
      FOLDERID_PublicDownloads, FOLDERID_PublicGameTasks, FOLDERID_PublicLibraries,
      FOLDERID_PublicMusic, FOLDERID_PublicPictures, FOLDERID_PublicRingtones,
      FOLDERID_PublicUserTiles, FOLDERID_PublicVideos, FOLDERID_QuickLaunch, FOLDERID_Recent,
      FOLDERID_RecordedTVLibrary, FOLDERID_ResourceDir, FOLDERID_Ringtones,
      FOLDERID_RoamedTileImages, FOLDERID_RoamingAppData, FOLDERID_RoamingTiles,
      FOLDERID_SampleMusic, FOLDERID_SamplePictures, FOLDERID_SamplePlaylists,
      FOLDERID_SampleVideos, FOLDERID_SavedGames, FOLDERID_SavedPictures, FOLDERID_SavedSearches,
      FOLDERID_Screenshots, FOLDERID_SearchHistory, FOLDERID_SearchTemplates, FOLDERID_SendTo,
      FOLDERID_SidebarDefaultParts, FOLDERID_SidebarParts, FOLDERID_SkyDrive,
      FOLDERID_SkyDriveCameraRoll, FOLDERID_SkyDriveDocuments, FOLDERID_SkyDrivePictures,
      FOLDERID_StartMenu, FOLDERID_Startup, FOLDERID_System, FOLDERID_SystemX86,
      FOLDERID_Templates, FOLDERID_UserPinned, FOLDERID_UserProfiles, FOLDERID_UserProgramFiles,
      FOLDERID_UserProgramFilesCommon, FOLDERID_Videos, FOLDERID_VideosLibrary, FOLDERID_Windows,
      KF_FLAG_DONT_VERIFY, SHGetKnownFolderPath,
   };
   use windows_sys::core::GUID;

   // If we try to access a Win32 path from an MSIX packaged context, we want to fail this operation.
   if get_application_data().is_ok() {
      return Err(Error::Win32PathInvokedFromMsixPackagedContext);
   }

   fn known_folder(folder_id: &GUID) -> Result<PathBuf> {
      unsafe {
         let mut path_ptr: *mut u16 = std::ptr::null_mut();
         // KF_FLAG_DONT_VERIFY: return the path even if the backing directory hasn't been
         // created yet (e.g. SavedGames, Screenshots, library folders). Mirrors dirs-sys.
         let result = SHGetKnownFolderPath(
            folder_id,
            KF_FLAG_DONT_VERIFY.try_into().unwrap(),
            std::ptr::null_mut(),
            &mut path_ptr,
         );
         if result == 0 {
            let len = lstrlenW(path_ptr) as usize;
            let path = slice::from_raw_parts(path_ptr, len);
            let ostr: OsString = OsStringExt::from_wide(path);
            CoTaskMemFree(path_ptr as *const c_void);
            Ok(PathBuf::from(ostr))
         } else {
            CoTaskMemFree(path_ptr as *const c_void);
            Err(Error::InvalidPath(format!(
               "SHGetKnownFolderPath failed with HRESULT: 0x{:08X}",
               result
            )))
         }
      }
   }

   match path {
      Win32Path::AccountPictures => known_folder(&FOLDERID_AccountPictures),
      Win32Path::AdminTools => known_folder(&FOLDERID_AdminTools),
      Win32Path::ApplicationShortcuts => known_folder(&FOLDERID_ApplicationShortcuts),
      Win32Path::CameraRoll => known_folder(&FOLDERID_CameraRoll),
      Win32Path::CdBurning => known_folder(&FOLDERID_CDBurning),
      Win32Path::CommonAdminTools => known_folder(&FOLDERID_CommonAdminTools),
      Win32Path::CommonOemLinks => known_folder(&FOLDERID_CommonOEMLinks),
      Win32Path::CommonPrograms => known_folder(&FOLDERID_CommonPrograms),
      Win32Path::CommonStartMenu => known_folder(&FOLDERID_CommonStartMenu),
      Win32Path::CommonStartup => known_folder(&FOLDERID_CommonStartup),
      Win32Path::CommonTemplates => known_folder(&FOLDERID_CommonTemplates),
      Win32Path::Contacts => known_folder(&FOLDERID_Contacts),
      Win32Path::Cookies => known_folder(&FOLDERID_Cookies),
      Win32Path::Desktop => known_folder(&FOLDERID_Desktop),
      Win32Path::DeviceMetadataStore => known_folder(&FOLDERID_DeviceMetadataStore),
      Win32Path::Documents => known_folder(&FOLDERID_Documents),
      Win32Path::DocumentsLibrary => known_folder(&FOLDERID_DocumentsLibrary),
      Win32Path::Downloads => known_folder(&FOLDERID_Downloads),
      Win32Path::Favorites => known_folder(&FOLDERID_Favorites),
      Win32Path::Fonts => known_folder(&FOLDERID_Fonts),
      Win32Path::GameTasks => known_folder(&FOLDERID_GameTasks),
      Win32Path::History => known_folder(&FOLDERID_History),
      Win32Path::ImplicitAppShortcuts => known_folder(&FOLDERID_ImplicitAppShortcuts),
      Win32Path::InternetCache => known_folder(&FOLDERID_InternetCache),
      Win32Path::Libraries => known_folder(&FOLDERID_Libraries),
      Win32Path::Links => known_folder(&FOLDERID_Links),
      Win32Path::LocalAppData => known_folder(&FOLDERID_LocalAppData),
      Win32Path::LocalAppDataLow => known_folder(&FOLDERID_LocalAppDataLow),
      Win32Path::LocalizedResourcesDir => known_folder(&FOLDERID_LocalizedResourcesDir),
      Win32Path::Music => known_folder(&FOLDERID_Music),
      Win32Path::MusicLibrary => known_folder(&FOLDERID_MusicLibrary),
      Win32Path::NetworkShortcuts => known_folder(&FOLDERID_NetHood),
      Win32Path::Objects3D => known_folder(&FOLDERID_Objects3D),
      Win32Path::OriginalImages => known_folder(&FOLDERID_OriginalImages),
      Win32Path::PhotoAlbums => known_folder(&FOLDERID_PhotoAlbums),
      Win32Path::PicturesLibrary => known_folder(&FOLDERID_PicturesLibrary),
      Win32Path::Pictures => known_folder(&FOLDERID_Pictures),
      Win32Path::Playlists => known_folder(&FOLDERID_Playlists),
      Win32Path::PrintHood => known_folder(&FOLDERID_PrintHood),
      Win32Path::Profile => known_folder(&FOLDERID_Profile),
      Win32Path::ProgramData => known_folder(&FOLDERID_ProgramData),
      Win32Path::ProgramFiles => known_folder(&FOLDERID_ProgramFiles),
      Win32Path::ProgramFilesX64 => known_folder(&FOLDERID_ProgramFilesX64),
      Win32Path::ProgramFilesX86 => known_folder(&FOLDERID_ProgramFilesX86),
      Win32Path::ProgramFilesCommon => known_folder(&FOLDERID_ProgramFilesCommon),
      Win32Path::ProgramFilesCommonX64 => known_folder(&FOLDERID_ProgramFilesCommonX64),
      Win32Path::ProgramFilesCommonX86 => known_folder(&FOLDERID_ProgramFilesCommonX86),
      Win32Path::Programs => known_folder(&FOLDERID_Programs),
      Win32Path::Public => known_folder(&FOLDERID_Public),
      Win32Path::PublicDesktop => known_folder(&FOLDERID_PublicDesktop),
      Win32Path::PublicDocuments => known_folder(&FOLDERID_PublicDocuments),
      Win32Path::PublicDownloads => known_folder(&FOLDERID_PublicDownloads),
      Win32Path::PublicGameTasks => known_folder(&FOLDERID_PublicGameTasks),
      Win32Path::PublicLibraries => known_folder(&FOLDERID_PublicLibraries),
      Win32Path::PublicMusic => known_folder(&FOLDERID_PublicMusic),
      Win32Path::PublicPictures => known_folder(&FOLDERID_PublicPictures),
      Win32Path::PublicRingtones => known_folder(&FOLDERID_PublicRingtones),
      Win32Path::PublicUserTiles => known_folder(&FOLDERID_PublicUserTiles),
      Win32Path::PublicVideos => known_folder(&FOLDERID_PublicVideos),
      Win32Path::QuickLaunch => known_folder(&FOLDERID_QuickLaunch),
      Win32Path::Recent => known_folder(&FOLDERID_Recent),
      Win32Path::RecordedTVLibrary => known_folder(&FOLDERID_RecordedTVLibrary),
      Win32Path::ResourceDir => known_folder(&FOLDERID_ResourceDir),
      Win32Path::Ringtones => known_folder(&FOLDERID_Ringtones),
      Win32Path::RoamingAppData => known_folder(&FOLDERID_RoamingAppData),
      Win32Path::RoamedTileImages => known_folder(&FOLDERID_RoamedTileImages),
      Win32Path::RoamingTiles => known_folder(&FOLDERID_RoamingTiles),
      Win32Path::SampleMusic => known_folder(&FOLDERID_SampleMusic),
      Win32Path::SamplePictures => known_folder(&FOLDERID_SamplePictures),
      Win32Path::SamplePlaylists => known_folder(&FOLDERID_SamplePlaylists),
      Win32Path::SampleVideos => known_folder(&FOLDERID_SampleVideos),
      Win32Path::SavedGames => known_folder(&FOLDERID_SavedGames),
      Win32Path::SavedPictures => known_folder(&FOLDERID_SavedPictures),
      Win32Path::SavedPicturesLibrary => known_folder(&FOLDERID_SAVED_PICTURES_LIBRARY),
      Win32Path::Searches => known_folder(&FOLDERID_SavedSearches),
      Win32Path::Screenshots => known_folder(&FOLDERID_Screenshots),
      Win32Path::SearchHistory => known_folder(&FOLDERID_SearchHistory),
      Win32Path::SearchTemplates => known_folder(&FOLDERID_SearchTemplates),
      Win32Path::SendTo => known_folder(&FOLDERID_SendTo),
      Win32Path::SidebarDefaultParts => known_folder(&FOLDERID_SidebarDefaultParts),
      Win32Path::SidebarParts => known_folder(&FOLDERID_SidebarParts),
      Win32Path::SkyDrive => known_folder(&FOLDERID_SkyDrive),
      Win32Path::SkyDriveCameraRoll => known_folder(&FOLDERID_SkyDriveCameraRoll),
      Win32Path::SkyDriveDocuments => known_folder(&FOLDERID_SkyDriveDocuments),
      Win32Path::SkyDrivePictures => known_folder(&FOLDERID_SkyDrivePictures),
      Win32Path::StartMenu => known_folder(&FOLDERID_StartMenu),
      Win32Path::Startup => known_folder(&FOLDERID_Startup),
      Win32Path::System32 => known_folder(&FOLDERID_System),
      Win32Path::SystemX86 => known_folder(&FOLDERID_SystemX86),
      Win32Path::Templates => known_folder(&FOLDERID_Templates),
      Win32Path::UserPinned => known_folder(&FOLDERID_UserPinned),
      Win32Path::Users => known_folder(&FOLDERID_UserProfiles),
      Win32Path::UserProgramFiles => known_folder(&FOLDERID_UserProgramFiles),
      Win32Path::UserProgramFilesCommon => known_folder(&FOLDERID_UserProgramFilesCommon),
      Win32Path::Videos => known_folder(&FOLDERID_Videos),
      Win32Path::VideoLibrary => known_folder(&FOLDERID_VideosLibrary),
      Win32Path::Windows => known_folder(&FOLDERID_Windows),
   }
}

// FOLDERID_SavedPicturesLibrary is not exported by windows-sys.
// GUID from: https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid
#[cfg(target_os = "windows")]
const FOLDERID_SAVED_PICTURES_LIBRARY: windows_sys::core::GUID = windows_sys::core::GUID {
   data1: 0xE25B5812,
   data2: 0xBE88,
   data3: 0x4BD9,
   data4: [0x94, 0xB0, 0x29, 0x23, 0x34, 0x77, 0xB6, 0xC3],
};

/// Returns whether ApplicationData::Current() succeeds, i.e. the process runs
/// in a packaged (MSIX) app context. Windows provides no API to detect MSI
/// specifically; unpackaged Win32 (including MSI installs) all lack package
/// identity.
/// So we can safely state that if ApplicationData::Current() succeeds, the
/// process is running in an MSIX app context.
/// If it fails, the process is running in a Win32 app context.
///
/// In practice, this is used to determine if the `WindowsPath::ApplicationData` or
/// `WindowsPath::Win32` variant should be used.
/// If this returns true, the `WindowsPath::ApplicationData` variant should be used.
/// If this returns false, the `WindowsPath::Win32` variant should be used.
#[cfg(target_os = "windows")]
fn get_application_data() -> Result<windows::Storage::ApplicationData> {
   windows::Storage::ApplicationData::Current()
      .map_err(|_| Error::WindowsApplicationDataPathInvokedFromWin32Context)
}
