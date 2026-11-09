use crate::Win32Path;
use crate::WindowsApplicationDataPath;
use crate::WindowsLibrary;
use crate::error::Error;
use crate::error::Result;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
use windows::core::HRESULT;

/// Resolves Win32 known-folder paths via SHGetKnownFolderPath.
///
/// Each Win32Path variant maps 1:1 to a KNOWNFOLDERID GUID. This is the standard
/// resolution mechanism for MSI-packaged (non-sandboxed) Win32 desktop applications.
/// Implementation mirrors the pattern used by the `dirs-sys` crate.
///
/// See: https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid
pub(crate) fn resolve_win32_path(path: &Win32Path, _bundle_identifier: &str) -> Result<PathBuf> {
   #[cfg(target_os = "windows")]
   {
      resolve_win32_path_inner(path, _bundle_identifier)
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

/// Resolves packaged app paths via the WinRT ApplicationData API.
///
/// Packaged apps (MSIX/APPX with package identity) expose per-package folders under
/// `AppData\Local\Packages\<id>\...`. Those folders are obtained from a WinRT
/// `ApplicationData` instance — see [`get_application_data`] for why that acquisition
/// branches on AppContainer vs MediumIL.
///
/// Fails with IncorrectOS on non-Windows, or an ApplicationData-related error if the
/// process lacks usable package identity / cannot open the package data store.
pub(crate) fn resolve_win_packaged_path(path: &WindowsApplicationDataPath) -> Result<PathBuf> {
   #[cfg(target_os = "windows")]
   {
      resolve_win_packaged_path_inner(path)
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

/// Returns filesystem paths of folders **pinned in** a Windows shell library (Documents,
/// Music, Pictures, etc.).
///
/// This is not the same as [`Win32Path`] `*Library` variants, which resolve the library
/// **definition** (the `.library-ms` file via `SHGetKnownFolderPath`). Libraries are
/// virtual aggregations; member paths come from the Shell Library API (`IShellLibrary`).
///
/// # Returns
///
/// * `Ok(paths)` — pinned member folders (may be empty if the user removed every location).
/// * `Err(MissingWindowsLibrary)` — the library definition is absent (user deleted it, etc.).
/// * `Err(WindowsShellLibraryResolutionFailed)` — COM/shell failure (not a missing library).
///
/// See: <https://learn.microsoft.com/en-us/windows/win32/shell/library-be-library-aware>
pub(crate) fn resolve_dirs_in_windows_library(library: &WindowsLibrary) -> Result<Vec<PathBuf>> {
   #[cfg(target_os = "windows")]
   {
      library_filesystem_dirs(library)
   }

   #[cfg(not(target_os = "windows"))]
   {
      Err(Error::IncorrectOS {
         path: format!("{library:?}"),
         current_os: std::env::consts::OS.to_string(),
         expected_os: "windows".to_string(),
      })
   }
}

/// Maps [`WindowsLibrary`] to the shell library `KNOWNFOLDERID`.
#[cfg(target_os = "windows")]
fn known_folder_id_for_library(library: WindowsLibrary) -> windows::core::GUID {
   use windows::Win32::UI::Shell::{
      FOLDERID_CameraRollLibrary, FOLDERID_DocumentsLibrary, FOLDERID_MusicLibrary,
      FOLDERID_PicturesLibrary, FOLDERID_SavedPicturesLibrary, FOLDERID_VideosLibrary,
   };

   match library {
      WindowsLibrary::Documents => FOLDERID_DocumentsLibrary,
      WindowsLibrary::Music => FOLDERID_MusicLibrary,
      WindowsLibrary::Pictures => FOLDERID_PicturesLibrary,
      WindowsLibrary::CameraRoll => FOLDERID_CameraRollLibrary,
      WindowsLibrary::SavedPictures => FOLDERID_SavedPicturesLibrary,
      WindowsLibrary::Video => FOLDERID_VideosLibrary,
   }
}

#[cfg(target_os = "windows")]
fn core_guid_to_sys(g: windows::core::GUID) -> windows_sys::core::GUID {
   windows_sys::core::GUID {
      data1: g.data1,
      data2: g.data2,
      data3: g.data3,
      data4: g.data4,
   }
}

/// Returns the `KNOWNFOLDERID` for a [`Win32Path`] `*Library` definition variant by routing
/// through [`known_folder_id_for_library`] (the same table used for [`WindowsLibrary`] member
/// resolution).
#[cfg(target_os = "windows")]
fn known_folder_id_for_win32_library_definition(
   path: &Win32Path,
) -> Option<windows_sys::core::GUID> {
   let library = match path {
      Win32Path::DocumentsLibrary => WindowsLibrary::Documents,
      Win32Path::MusicLibrary => WindowsLibrary::Music,
      Win32Path::PicturesLibrary => WindowsLibrary::Pictures,
      Win32Path::CameraRollLibrary => WindowsLibrary::CameraRoll,
      Win32Path::SavedPicturesLibrary => WindowsLibrary::SavedPictures,
      Win32Path::VideoLibrary => WindowsLibrary::Video,
      _ => return None,
   };
   Some(core_guid_to_sys(known_folder_id_for_library(library)))
}

// --- Windows shell library: COM interop helpers ---

/// Holds an STA COM apartment for the duration of a library resolution call.
///
/// Tauri/WebView2 may already have initialized COM in another apartment; in that case
/// `CoInitializeEx` returns `RPC_E_CHANGED_MODE` and we leave the existing mode in place.
#[cfg(target_os = "windows")]
struct ComApartment {
   uninit_on_drop: bool,
}

#[cfg(target_os = "windows")]
impl ComApartment {
   fn enter(library: &WindowsLibrary) -> Result<Self> {
      use windows::Win32::Foundation::RPC_E_CHANGED_MODE;
      use windows::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx};

      unsafe {
         let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
         if hr.is_ok() {
            Ok(Self {
               uninit_on_drop: true,
            })
         } else if hr == RPC_E_CHANGED_MODE {
            Ok(Self {
               uninit_on_drop: false,
            })
         } else {
            Err(library_resolution_err(
               library,
               "CoInitializeEx",
               windows::core::Error::from_hresult(hr),
            ))
         }
      }
   }
}

#[cfg(target_os = "windows")]
impl Drop for ComApartment {
   fn drop(&mut self) {
      if self.uninit_on_drop {
         unsafe {
            windows::Win32::System::Com::CoUninitialize();
         }
      }
   }
}

#[cfg(target_os = "windows")]
fn is_missing_windows_library_hresult(hr: HRESULT) -> bool {
   use windows::Win32::Foundation::{
      ERROR_FILE_NOT_FOUND, ERROR_PATH_NOT_FOUND, STG_E_FILENOTFOUND,
   };

   hr == STG_E_FILENOTFOUND
      || hr == HRESULT::from_win32(ERROR_FILE_NOT_FOUND.0)
      || hr == HRESULT::from_win32(ERROR_PATH_NOT_FOUND.0)
}

#[cfg(target_os = "windows")]
fn library_resolution_err(
   library: &WindowsLibrary,
   step: &'static str,
   err: windows::core::Error,
) -> Error {
   Error::WindowsShellLibraryResolutionFailed {
      library: library.to_string(),
      step,
      reason: err.to_string(),
   }
}

/// Converts a COM-allocated wide string (`PWSTR`) to `PathBuf` and frees the buffer.
#[cfg(target_os = "windows")]
fn pwstr_to_path_buf(psz: windows::core::PWSTR) -> Result<PathBuf> {
   use std::ffi::OsString;
   use std::ffi::c_void;
   use std::os::windows::ffi::OsStringExt;

   use windows_sys::Win32::System::Com::CoTaskMemFree;

   unsafe {
      let len = windows_sys::Win32::Globalization::lstrlenW(psz.0);
      if len == 0 {
         CoTaskMemFree(psz.0 as *const c_void);
         return Ok(PathBuf::new());
      }
      let slice = std::slice::from_raw_parts(psz.0, len as usize);
      let path = PathBuf::from(OsString::from_wide(slice));
      CoTaskMemFree(psz.0 as *const c_void);
      Ok(path)
   }
}

// --- Windows shell library: business logic ---

/// Orchestrates library member-path resolution: COM init, load library, enumerate folders.
#[cfg(target_os = "windows")]
fn library_filesystem_dirs(library: &WindowsLibrary) -> Result<Vec<PathBuf>> {
   use windows::Win32::UI::Shell::LFF_FORCEFILESYSTEM;

   let _com = ComApartment::enter(library)?;
   let known_folder = known_folder_id_for_library(*library);
   let shell_library = load_shell_library_from_known_folder(library, &known_folder)?;
   let folders = unsafe {
      shell_library
         .GetFolders(LFF_FORCEFILESYSTEM)
         .map_err(|e| library_resolution_err(library, "IShellLibrary::GetFolders", e))?
   };
   shell_item_array_filesystem_paths(library, &folders)
}

/// Creates `IShellLibrary` and loads the library described by `known_folder`.
#[cfg(target_os = "windows")]
fn load_shell_library_from_known_folder(
   library: &WindowsLibrary,
   known_folder: &windows::core::GUID,
) -> Result<windows::Win32::UI::Shell::IShellLibrary> {
   use windows::Win32::System::Com::{CLSCTX_INPROC_SERVER, CoCreateInstance, STGM_READ};
   use windows::Win32::UI::Shell::{IShellLibrary, ShellLibrary};

   unsafe {
      let shell_library: IShellLibrary =
         CoCreateInstance(&ShellLibrary, None, CLSCTX_INPROC_SERVER).map_err(|e| {
            library_resolution_err(library, "CoCreateInstance(CLSID_ShellLibrary)", e)
         })?;
      shell_library
         .LoadLibraryFromKnownFolder(known_folder, STGM_READ.0)
         .map_err(|e| {
            if is_missing_windows_library_hresult(e.code()) {
               Error::MissingWindowsLibrary {
                  library: library.to_string(),
               }
            } else {
               library_resolution_err(library, "IShellLibrary::LoadLibraryFromKnownFolder", e)
            }
         })?;
      Ok(shell_library)
   }
}

/// Maps each shell item in `folders` to a filesystem path (`SIGDN_FILESYSPATH`).
#[cfg(target_os = "windows")]
fn shell_item_array_filesystem_paths(
   library: &WindowsLibrary,
   folders: &windows::Win32::UI::Shell::IShellItemArray,
) -> Result<Vec<PathBuf>> {
   use windows::Win32::UI::Shell::SIGDN_FILESYSPATH;

   unsafe {
      let count = folders
         .GetCount()
         .map_err(|e| library_resolution_err(library, "IShellItemArray::GetCount", e))?;

      let mut paths = Vec::with_capacity(count as usize);
      for i in 0..count {
         let item = folders
            .GetItemAt(i)
            .map_err(|e| library_resolution_err(library, "IShellItemArray::GetItemAt", e))?;

         let psz = item
            .GetDisplayName(SIGDN_FILESYSPATH)
            .map_err(|e| library_resolution_err(library, "IShellItem::GetDisplayName", e))?;

         let path = pwstr_to_path_buf(psz)?;
         if !path.as_os_str().is_empty() {
            paths.push(path);
         }
      }

      Ok(paths)
   }
}

#[cfg(target_os = "windows")]
fn resolve_win_packaged_path_inner(path: &WindowsApplicationDataPath) -> Result<PathBuf> {
   use std::ffi::OsString;
   use std::os::windows::ffi::OsStringExt;

   // See get_application_data for AppContainer vs MediumIL API selection.
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

#[cfg(target_os = "windows")]
fn resolve_win32_path_inner(path: &Win32Path, bundle_identifier: &str) -> Result<PathBuf> {
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
      FOLDERID_DeviceMetadataStore, FOLDERID_Documents, FOLDERID_Downloads, FOLDERID_Favorites,
      FOLDERID_Fonts, FOLDERID_GameTasks, FOLDERID_History, FOLDERID_ImplicitAppShortcuts,
      FOLDERID_InternetCache, FOLDERID_Libraries, FOLDERID_Links, FOLDERID_LocalAppData,
      FOLDERID_LocalAppDataLow, FOLDERID_LocalizedResourcesDir, FOLDERID_Music, FOLDERID_NetHood,
      FOLDERID_Objects3D, FOLDERID_OriginalImages, FOLDERID_PhotoAlbums, FOLDERID_Pictures,
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
      FOLDERID_UserProgramFilesCommon, FOLDERID_Videos, FOLDERID_Windows, KF_FLAG_DONT_VERIFY,
      SHGetKnownFolderPath,
   };
   use windows_sys::core::GUID;

   /// Resolves a [`Win32Path`] `*Library` variant (the `.library-ms` definition path) via
   /// `SHGetKnownFolderPath`, using the shared library GUID mapping in
   /// [`known_folder_id_for_library`].
   ///
   /// Shell libraries are identified by one `KNOWNFOLDERID` whether you resolve the
   /// definition file (`DocumentsLibrary`, …) or pinned member folders (`WindowsLibrary`,
   /// `IShellLibrary`). This helper keeps those GUIDs in a single place.
   ///
   /// Without it, each `*Library` arm in the match below would call `known_folder` with its
   /// own `FOLDERID_*` import from `windows_sys`, duplicating the six IDs already maintained
   /// for `resolve_dirs_in_windows_library` — and a fix on one side (e.g. `SavedPictures` or
   /// `CameraRollLibrary`) could be missed on the other. `RecordedTVLibrary` is intentionally
   /// excluded (no [`WindowsLibrary`] counterpart) and still uses `FOLDERID_RecordedTVLibrary`
   /// directly.
   fn known_folder_from_win32_library_definition(path: &Win32Path) -> Result<PathBuf> {
      let folder_id = known_folder_id_for_win32_library_definition(path)
         .ok_or_else(|| Error::InvalidPath(format!("Not a library definition path: {path}")))?;
      known_folder(&folder_id)
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
      Win32Path::CameraRollLibrary => known_folder_from_win32_library_definition(path),
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
      Win32Path::DocumentsLibrary => known_folder_from_win32_library_definition(path),
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
      Win32Path::LocalAppDataForCurrentApp => {
         let path = known_folder(&FOLDERID_LocalAppData)?;
         Ok(path.join(bundle_identifier))
      }
      Win32Path::LocalAppDataLow => known_folder(&FOLDERID_LocalAppDataLow),
      Win32Path::LocalizedResourcesDir => known_folder(&FOLDERID_LocalizedResourcesDir),
      Win32Path::Music => known_folder(&FOLDERID_Music),
      Win32Path::MusicLibrary => known_folder_from_win32_library_definition(path),
      Win32Path::NetworkShortcuts => known_folder(&FOLDERID_NetHood),
      Win32Path::Objects3D => known_folder(&FOLDERID_Objects3D),
      Win32Path::OriginalImages => known_folder(&FOLDERID_OriginalImages),
      Win32Path::PhotoAlbums => known_folder(&FOLDERID_PhotoAlbums),
      Win32Path::PicturesLibrary => known_folder_from_win32_library_definition(path),
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
      Win32Path::RoamingAppDataForCurrentApp => {
         let path = known_folder(&FOLDERID_RoamingAppData)?;
         Ok(path.join(bundle_identifier))
      }
      Win32Path::RoamedTileImages => known_folder(&FOLDERID_RoamedTileImages),
      Win32Path::RoamingTiles => known_folder(&FOLDERID_RoamingTiles),
      Win32Path::SampleMusic => known_folder(&FOLDERID_SampleMusic),
      Win32Path::SamplePictures => known_folder(&FOLDERID_SamplePictures),
      Win32Path::SamplePlaylists => known_folder(&FOLDERID_SamplePlaylists),
      Win32Path::SampleVideos => known_folder(&FOLDERID_SampleVideos),
      Win32Path::SavedGames => known_folder(&FOLDERID_SavedGames),
      Win32Path::SavedPictures => known_folder(&FOLDERID_SavedPictures),
      Win32Path::SavedPicturesLibrary => known_folder_from_win32_library_definition(path),
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
      Win32Path::VideoLibrary => known_folder_from_win32_library_definition(path),
      Win32Path::Windows => known_folder(&FOLDERID_Windows),
   }
}

/// Obtains the WinRT [`ApplicationData`](https://learn.microsoft.com/en-us/uwp/api/windows.storage.applicationdata)
/// instance for the current packaged process.
///
/// # Why this is not a single API call
///
/// On Windows, “packaged” and “sandboxed” are **orthogonal**. A process can have
/// [package identity](https://learn.microsoft.com/en-us/windows/msix/detect-package-identity)
/// (linked to an MSIX/APPX registration — what this crate’s `FsEnvironment::WinPackaged`
/// detects via `GetCurrentPackageFullName`) without running inside an
/// [AppContainer](https://learn.microsoft.com/en-us/windows/win32/secauthz/appcontainer-isolation)
/// sandbox. Trust level is a separate manifest knob (`uap10:TrustLevel`).
///
/// Microsoft’s documented contract (see the
/// [Windows App SDK ApplicationData spec](https://github.com/microsoft/WindowsAppSDK/blob/main/specs/applicationdata/ApplicationData.md))
/// is:
///
/// | Process trust | Documented API |
/// |---|---|
/// | AppContainer (`uap10:TrustLevel="appContainer"`) | [`ApplicationData::Current`](https://learn.microsoft.com/en-us/uwp/api/windows.storage.applicationdata.current) |
/// | Not AppContainer (e.g. MediumIL) | [`ApplicationDataManager::CreateForPackageFamily`](https://learn.microsoft.com/en-us/uwp/api/windows.management.core.applicationdatamanager.createforpackagefamily) |
///
/// In practice the line is softer than that table: many MediumIL Desktop Bridge /
/// Tauri MSIX processes (`EntryPoint="Windows.FullTrustApplication"`,
/// `RuntimeBehavior="packagedClassicApp"`) **do** succeed with `Current()` (including
/// this repo’s `examples/tauri-app` `dev:msix` flow). Other MediumIL hosts — notably
/// some WinUI 3 packaged apps — fail `Current()` and need `CreateForPackageFamily`.
/// So we do not assume MediumIL ⇒ fallback; we **try `Current` first**, then fall back.
///
/// The two APIs are still complementary, not interchangeable:
/// - `Current` is the historical UWP “self” accessor.
/// - `CreateForPackageFamily` is a [management API](https://learn.microsoft.com/en-us/uwp/api/windows.management.core.applicationdatamanager)
///   for MediumIL+ callers (and tools). Docs state it **cannot** be used from an
///   AppContainer process, so AppContainer apps must take the `Current` path.
///
/// The Windows App SDK’s newer `Microsoft.Windows.Storage.ApplicationData.GetDefault()`
/// performs the same Current-then-fallback branch; we mirror it with inbox WinRT types.
///
/// # Packaging terms (brief)
///
/// - **MSIX / APPX** — the package *format* and installer registration. Having an MSIX does
///   **not** imply AppContainer; trust level is declared separately in the manifest
///   (`uap10:TrustLevel`, `uap10:RuntimeBehavior`). See
///   [uap10 extension](https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-uap10-application).
/// - **WinRT** — the *API surface* (`Windows.Storage.*`, `Windows.Management.Core.*`, …).
///   Both branches below are WinRT; they differ only in how the `ApplicationData` object
///   is obtained.
/// - **`packagedClassicApp` vs `win32App`** — `RuntimeBehavior` knobs. `packagedClassicApp`
///   is the common Desktop Bridge / Tauri MSIX path (often MediumIL). `win32App` is used by
///   sparse / external-location identity packages that keep classic Win32 data layout; those
///   apps usually should map paths via Win32 known folders instead of ApplicationData.
///
/// # Branches
///
/// 1. **`ApplicationData::Current()`** — preferred path. Works for AppContainer apps, and
///    often for MediumIL packaged desktop apps as well. Returns immediately on success.
/// 2. **`Package::Current` → family name → `CreateForPackageFamily`** — defensive fallback
///    when `Current` fails despite package identity (documented MediumIL / non-AppContainer
///    cases, including some WinUI 3 packaged hosts). Resolves the package family name via
///    [`Package.Id.FamilyName`](https://learn.microsoft.com/en-us/uwp/api/windows.applicationmodel.packageid.familyname)
///    and opens the same per-user package data store.
///
/// Unpackaged processes (no package identity) fail both paths; callers should not invoke
/// this when `FsEnvironment` is plain Win32.
#[cfg(target_os = "windows")]
fn get_application_data() -> Result<windows::Storage::ApplicationData> {
   use windows::Management::Core::ApplicationDataManager;

   // Preferred: works for AppContainer, and often for MediumIL packaged desktop (e.g. Tauri MSIX).
   let app_data_result = windows::Storage::ApplicationData::Current();

   if let Ok(app_data) = app_data_result {
      return Ok(app_data);
   }

   let app_data_error_string = app_data_result
      .err()
      .map_or("".to_string(), |e| e.to_string());

   // Fallback when Current fails despite package identity (MS-documented MediumIL path;
   // CreateForPackageFamily is blocked inside AppContainer — only reached when Current failed).
   let family_name = windows::ApplicationModel::Package::Current()
      .map_err(|e| {
         Error::CouldNotRetrieveCurrentPackage(e.to_string(), app_data_error_string.to_owned())
      })?
      .Id()
      .map_err(|e| {
         Error::CouldNotRetrieveIdFromPackage(e.to_string(), app_data_error_string.to_owned())
      })?
      .FamilyName()
      .map_err(|e| {
         Error::CouldNotRetrieveFamilyNameFromPackage(
            e.to_string(),
            app_data_error_string.to_owned(),
         )
      })?;

   ApplicationDataManager::CreateForPackageFamily(&family_name).map_err(|e| {
      Error::CouldNotCreateApplicationDataForPackageFamily(
         e.to_string(),
         app_data_error_string.to_owned(),
      )
   })
}

#[cfg(target_os = "windows")]
#[cfg(test)]
// Platform-specific resolution tests; CI runs these on windows-latest (see .github/workflows/ci.yml).
mod tests {

   use super::*;
   use crate::Error;
   use rstest::rstest;

   const BUNDLE_ID: &str = "com.example.app";

   #[rstest]
   #[case::local_app_data(Win32Path::LocalAppData, Win32Path::LocalAppDataForCurrentApp)]
   #[case::roaming_app_data(Win32Path::RoamingAppData, Win32Path::RoamingAppDataForCurrentApp)]
   fn for_current_app_appends_bundle_identifier(
      #[case] base_path: Win32Path,
      #[case] app_path: Win32Path,
   ) {
      let base = resolve_win32_path(&base_path, "").unwrap();
      let path = resolve_win32_path(&app_path, BUNDLE_ID).unwrap();

      assert_eq!(path, base.join(BUNDLE_ID));
   }

   #[test]
   fn resolve_documents_library_twice_on_same_thread() {
      let first = resolve_dirs_in_windows_library(&WindowsLibrary::Documents);
      let second = resolve_dirs_in_windows_library(&WindowsLibrary::Documents);
      assert!(first.is_ok(), "first call failed: {first:?}");
      assert!(second.is_ok(), "second call failed: {second:?}");
   }

   #[rstest]
   #[case::documents(WindowsLibrary::Documents)]
   #[case::music(WindowsLibrary::Music)]
   #[case::pictures(WindowsLibrary::Pictures)]
   #[case::camera_roll(WindowsLibrary::CameraRoll)]
   #[case::saved_pictures(WindowsLibrary::SavedPictures)]
   #[case::video(WindowsLibrary::Video)]
   fn resolve_dirs_in_windows_library_returns_ok_or_missing(#[case] library: WindowsLibrary) {
      match resolve_dirs_in_windows_library(&library) {
         Ok(paths) => {
            for path in paths {
               assert!(
                  path.is_absolute(),
                  "expected absolute path for {library}, got {path:?}"
               );
            }
         }
         Err(Error::MissingWindowsLibrary { .. }) => {}
         Err(other) => panic!("unexpected error for {library}: {other}"),
      }
   }
}
