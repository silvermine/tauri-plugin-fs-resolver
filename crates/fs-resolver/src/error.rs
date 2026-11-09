use serde::{Serialize, ser::Serializer};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
   #[error("Initialization error: {0}")]
   Initialization(String),

   #[error("Unsupported environment: {0}")]
   UnsupportedEnvironment(String),

   #[error("Invalid path: {0}")]
   InvalidPath(String),

   #[error("Path mapping undefined for OS: {0}")]
   PathMappingUndefined(String),

   #[error("win_packaged mapping is missing while running as WinPackaged.
resolve_mapping does not fall back to win32. Set win_packaged explicitly:
WinPackagedPathMapping::WindowsApplicationDataPath(...) or
WinPackagedPathMapping::Win32Path(...) (same known folder as unpackaged if that is intentional).
Note: resolve_win32 still works under WinPackaged; only cross-platform mapping has this specific requirement.")]
   WinPackagedPathMappingUndefined,

   #[error("Incorrect OS for path {path}: current OS is {current_os}, invoked for {expected_os}")]
   IncorrectOS {
      path: String,
      current_os: String,
      expected_os: String,
   },

   #[error("Not implemented: {0}")]
   NotImplemented(String),

   #[error("JSON serialization error: {0}")]
   JsonSerialization(String),

   #[error("Android path resolution not configured")]
   AndroidPathResolutionNotConfigured,

   #[error("Plugin invocation error: {0}")]
   PluginInvocation(String),

   #[error("Linux environment missing: ${variable} is not set (resolving {path}). {hint}")]
   LinuxEnvironmentMissing {
      variable: String,
      path: String,
      hint: String,
   },

   #[error("Backup exclusion failed for {path}: {reason}")]
   BackupExclusionFailed { path: String, reason: String },

   #[error("Could not determine Windows packaging environment: {0}")]
   CouldNotDetermineWindowsPackagingEnvironment(String),

   #[error("Attempted determining Windows packaging environment on non-Windows platform")]
   AttemptedDeterminingWindowsPackagingEnvironmentOnNonWindowsPlatform,

   #[error("Could not retrieve current package: {0}; Current Package retrieval error: {1}")]
   CouldNotRetrieveCurrentPackage(String, String),

   #[error("Could not retrieve ID from package: {0}; Current Package retrieval error: {1}")]
   CouldNotRetrieveIdFromPackage(String, String),

   #[error(
      "Could not retrieve family name from package: {0}; Current Package retrieval error: {1}"
   )]
   CouldNotRetrieveFamilyNameFromPackage(String, String),

   #[error(
      "Could not create ApplicationData for package family: {0}; Current Package retrieval error: {1}"
   )]
   CouldNotCreateApplicationDataForPackageFamily(String, String),

   /// The shell library definition does not exist or could not be loaded (distinct from a
   /// library that exists but has zero pinned member folders).
   #[error("Windows shell library not found: {library}")]
   MissingWindowsLibrary { library: String },

   /// COM or Shell Library API failure while resolving pinned member folders.
   #[error("Windows shell library resolution failed for {library} at {step}: {reason}")]
   WindowsShellLibraryResolutionFailed {
      library: String,
      step: &'static str,
      reason: String,
   },
}

/// Serialize errors as plain strings for the Tauri IPC bridge.
/// The TypeScript layer receives these as rejected promise messages.
impl Serialize for Error {
   fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
   where
      S: Serializer,
   {
      serializer.serialize_str(self.to_string().as_ref())
   }
}
