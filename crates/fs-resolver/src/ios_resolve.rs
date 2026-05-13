use std::path::PathBuf;

use crate::error::Error;
use crate::error::Result;
use crate::ios_paths::IosPath;

pub(crate) fn resolve_ios_path(path: &IosPath) -> Result<PathBuf> {
   // TODO: Implement path resolution, with cfg-gated compilation
   // for platform-specific operations.
   Err(Error::NotImplemented(format!(
      "resolve_ios_path: {:?}",
      path
   )))
}
