use crate::error::Error;
use crate::error::Result;
use crate::linux_paths::LinuxPath;
use std::path::PathBuf;

pub(crate) fn resolve_linux_path(path: &LinuxPath) -> Result<PathBuf> {
   // TODO: Implement path resolution, with cfg-gated compilation
   // for platform-specific operations.
   Err(Error::NotImplemented(format!(
      "resolve_linux_path: {:?}",
      path
   )))
}
