use std::path::PathBuf;

use crate::error::Error;
use crate::error::Result;
use crate::ios_paths::IosPath;

pub(crate) fn resolve_ios_path(path: &IosPath) -> Result<PathBuf> {
   #[cfg(target_os = "ios")]
   {
      resolve_ios_path_inner(path)
   }

   #[cfg(not(target_os = "ios"))]
   {
      Err(Error::IncorrectOS {
         path: path.to_string(),
         current_os: std::env::consts::OS.to_string(),
         expected_os: "ios".to_string(),
      })
   }
}

#[cfg(target_os = "ios")]
pub(crate) fn resolve_ios_path_inner(path: &IosPath) -> Result<PathBuf> {
   use objc2_foundation::{
      NSHomeDirectory, NSSearchPathDirectory, NSSearchPathDomainMask,
      NSSearchPathForDirectoriesInDomains, NSTemporaryDirectory,
   };

   fn resolve_search_path_directory(directory: NSSearchPathDirectory) -> Result<PathBuf> {
      let arr = NSSearchPathForDirectoriesInDomains(directory, NS_USER_DOMAIN_MASK, true);
      let first = arr
         .firstObject()
         .ok_or_else(|| Error::InvalidPath(format!("{:?} not found", directory)))?;

      Ok(PathBuf::from(first.to_string()))
   }

   const NS_USER_DOMAIN_MASK: NSSearchPathDomainMask = NSSearchPathDomainMask::UserDomainMask;

   match path {
      IosPath::DocumentDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::DocumentDirectory)
      }
      IosPath::LibraryDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::LibraryDirectory)
      }
      IosPath::CachesDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::CachesDirectory)
      }
      IosPath::ApplicationSupportDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::ApplicationSupportDirectory)
      }
      IosPath::AutosavedInformationDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::AutosavedInformationDirectory)
      }
      IosPath::DownloadsDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::DownloadsDirectory)
      }
      IosPath::MoviesDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::MoviesDirectory)
      }
      IosPath::MusicDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::MusicDirectory)
      }
      IosPath::PicturesDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::PicturesDirectory)
      }
      IosPath::ItemReplacementDirectory => {
         resolve_search_path_directory(NSSearchPathDirectory::ItemReplacementDirectory)
      }
      IosPath::HomeDirectory => Ok(PathBuf::from(NSHomeDirectory().to_string())),
      IosPath::TemporaryDirectory => Ok(PathBuf::from(NSTemporaryDirectory().to_string())),
   }
}
