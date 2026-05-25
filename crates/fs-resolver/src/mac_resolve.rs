use crate::error::Error;
use crate::error::Result;
use crate::mac_paths::MacPath;
use std::path::PathBuf;

pub(crate) fn resolve_mac_path(path: &MacPath) -> Result<PathBuf> {
   // TODO: Implement path resolution, with cfg-gated compilation
   // for platform-specific operations.
   Err(Error::NotImplemented(format!(
      "resolve_mac_path: {:?}",
      path
   )))
}
