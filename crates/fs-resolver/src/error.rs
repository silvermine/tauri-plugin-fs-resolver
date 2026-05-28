use serde::{Serialize, ser::Serializer};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
   #[error("Initialization error: {0}")]
   Initialization(String),

   #[error("Invalid path: {0}")]
   InvalidPath(String),

   #[error("Path mapping undefined for OS: {0}")]
   PathMappingUndefined(String),

   #[error("Unsupported platform: {0}")]
   UnsupportedPlatform(String),

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

   #[error(
      "Win32 path invoked from MSIX packaged context; make sure to use the WindowsPath::WindowsApplicationDataPath variant instead"
   )]
   Win32PathInvokedFromMsixPackagedContext,

   #[error(
      "WindowsApplicationDataPath invoked from unpackaged context; make sure to use the WindowsPath::Win32Path variant instead"
   )]
   WindowsApplicationDataPathInvokedFromWin32Context,
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
