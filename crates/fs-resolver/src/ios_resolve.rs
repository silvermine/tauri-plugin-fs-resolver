use std::path::PathBuf;

use crate::error::Error;
use crate::error::Result;
use crate::ios_paths::IosPath;

pub(crate) fn resolve_ios_path(path: &IosPath, _bundle_identifier: &str) -> Result<PathBuf> {
   #[cfg(target_os = "ios")]
   {
      resolve_ios_path_inner(path, _bundle_identifier)
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
pub(crate) fn resolve_ios_path_inner(path: &IosPath, bundle_identifier: &str) -> Result<PathBuf> {
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
         // For Mac and Linux, we have tests for this logic of appending the bundle
         // identifier to verify we're correctly appending the bundle identifier.
         // However, we can't do this for iOS presently.
         // If we were to hide the tests behind a #[cfg(target_os = "ios")], that cfg only
         // compiles for an iOS target, and cargo test cannot execute iOS binaries on host
         // CI without an iOS Simulator runner (aarch64-apple-ios-sim + simctl/target
         // runner).
         // The append itself is PathBuf::join, and it would be useless to add a test that
         // is only testing that `join` works.
         // So until CI supports running iOS binaries, we won't test this logic.
         let application_support_directory =
            resolve_search_path_directory(NSSearchPathDirectory::ApplicationSupportDirectory)?;
         Ok(application_support_directory.join(bundle_identifier))
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
