use crate::error::Error;
use crate::error::Result;
use crate::windows_paths::WindowsPath;
use std::path::PathBuf;

pub(crate) fn resolve_windows_path(path: &WindowsPath) -> Result<PathBuf> {
   // TODO: Implement path resolution, with cfg-gated compilation
   // for platform-specific operations.
   Err(Error::NotImplemented(format!(
      "resolve_windows_path: {:?}",
      path
   )))
}
