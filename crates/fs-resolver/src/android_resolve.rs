use crate::android_paths::AndroidPath;
use crate::android_paths::AndroidPathCollection;
use crate::error::Error;
use crate::error::Result;
use std::path::PathBuf;

pub(crate) fn resolve_android_path(path: &AndroidPath) -> Result<PathBuf> {
   // TODO: Implement path resolution, with cfg-gated compilation
   // for platform-specific operations.
   Err(Error::NotImplemented(format!(
      "resolve_android_path: {:?}",
      path
   )))
}

pub(crate) fn resolve_android_path_collection(
   collection: &AndroidPathCollection,
) -> Result<Vec<PathBuf>> {
   // TODO: Implement path resolution, with cfg-gated compilation
   // for platform-specific operations.
   Err(Error::NotImplemented(format!(
      "resolve_android_path_collection: {:?}",
      collection
   )))
}
