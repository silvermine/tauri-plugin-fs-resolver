use std::path::Component;

use crate::android_paths::AndroidPath;
use crate::error::{Error, Result};
use crate::ios_paths::IosPath;
use crate::linux_paths::LinuxPath;
use crate::mac_paths::MacPath;
use crate::windows_paths::WindowsPath;

/// Per-platform path mapping: a platform path enum plus an optional relative suffix.
///
/// Resolved in-process via [`PathResolver::resolve_mapping`](crate::PathResolver::resolve_mapping).
/// Not serialized over IPC — only individual path enums cross the plugin boundary.
///
/// When `relative_path` is set, the resolver joins it to the resolved platform path.
/// The relative path must contain only normal path segments (no absolute paths, `.`, `..`,
/// or empty segments).
#[derive(Debug)]
pub struct PlatformMapping<T: PlatformPath> {
   pub platform_path: T,
   pub relative_path: Option<String>,
}

/// Cross-platform path definition with optional per-OS mappings.
///
/// Parallel to the TypeScript `CrossPlatformMapping` type. Each platform field holds a
/// [`PlatformMapping`] for that OS. At resolution time, only the mapping for the current
/// platform is used. Not sent across IPC.
#[derive(Debug)]
pub struct CrossPlatformMapping {
   pub android: Option<PlatformMapping<AndroidPath>>,
   pub ios: Option<PlatformMapping<IosPath>>,
   pub macos: Option<PlatformMapping<MacPath>>,
   pub linux: Option<PlatformMapping<LinuxPath>>,
   pub windows: Option<PlatformMapping<WindowsPath>>,
}

/// Validates that `relative_path` contains only normal path segments.
///
/// Rejects absolute paths, `.`, `..`, and empty segments so `PathBuf::join` cannot
/// silently replace the resolved base path.
pub fn validate_relative_path(relative_path: &str) -> Result<()> {
   if relative_path.is_empty() {
      return Err(Error::InvalidPath(
         "Relative path must not be empty".to_string(),
      ));
   }

   for segment in relative_path.split(['/', '\\']) {
      if segment.is_empty() || segment == "." || segment == ".." {
         return Err(Error::InvalidPath(format!(
            "Relative path must contain only normal path segments: {relative_path}"
         )));
      }
   }

   for component in std::path::Path::new(relative_path).components() {
      if !matches!(component, Component::Normal(_)) {
         return Err(Error::InvalidPath(format!(
            "Relative path must contain only normal path segments: {relative_path}"
         )));
      }
   }

   Ok(())
}

/// Validates that `bundle_identifier` is safe to append as a single path segment.
///
/// Rejects empty identifiers, path separators, `.`, `..`, and absolute paths so
/// `PathBuf::join` cannot silently return the shared base directory or escape it.
pub fn validate_bundle_identifier(bundle_identifier: &str) -> Result<()> {
   if bundle_identifier.contains('/') || bundle_identifier.contains('\\') {
      return Err(Error::InvalidPath(format!(
         "Bundle identifier must be a single safe path segment: {bundle_identifier}"
      )));
   }

   validate_relative_path(bundle_identifier).map_err(|_| {
      Error::InvalidPath(format!(
         "Bundle identifier must be a single safe path segment: {bundle_identifier}"
      ))
   })
}

// This is a marker to ensure that PlatformMapping can only be instantiated with a type
// that is defined in this module.
pub trait PlatformPath: private::Sealed {}
mod private {
   pub trait Sealed {}
}
impl private::Sealed for AndroidPath {}
impl private::Sealed for IosPath {}
impl private::Sealed for MacPath {}
impl private::Sealed for LinuxPath {}
impl private::Sealed for WindowsPath {}

impl PlatformPath for AndroidPath {}
impl PlatformPath for IosPath {}
impl PlatformPath for MacPath {}
impl PlatformPath for LinuxPath {}
impl PlatformPath for WindowsPath {}

#[cfg(test)]
mod tests {
   use super::*;
   use test_case::test_case;

   #[test_case("data")]
   #[test_case("the/android/path")]
   #[test_case("nested/deep/path")]
   fn validate_relative_path_accepts_normal_segments(segment: &str) {
      validate_relative_path(segment).unwrap();
   }

   #[test_case("" ; "Relative path must not be empty")]
   #[test_case("." ; "Relative path must not contain current directory segment")]
   #[test_case(".." ; "Relative path must not contain parent directory segment")]
   #[test_case("/absolute" ; "Relative path must not contain absolute path")]
   #[test_case("foo/.." ; "Relative path must not contain parent directory segment after normal segment")]
   #[test_case("../foo" ; "Relative path must not contain parent directory segment before normal segment")]
   #[test_case("foo/." ; "Relative path must not contain current directory segment after normal segment")]
   #[test_case("./foo" ; "Relative path must not contain current directory segment before normal segment")]
   #[test_case("foo//bar" ; "Relative path must not contain multiple consecutive slashes")]
   fn validate_relative_path_rejects_invalid_segments(segment: &str) {
      assert!(
         validate_relative_path(segment).is_err(),
         "expected {segment} to be rejected"
      );
   }

   #[test_case("com.example.app")]
   fn validate_bundle_identifier_accepts_valid_ids(id: &str) {
      validate_bundle_identifier(id).unwrap();
   }

   #[test_case("" ; "empty")]
   #[test_case(".." ; "parent segment")]
   #[test_case("/absolute" ; "absolute path")]
   #[test_case("com/example" ; "path separator")]
   fn validate_bundle_identifier_rejects_invalid_ids(id: &str) {
      assert!(
         validate_bundle_identifier(id).is_err(),
         "expected {id} to be rejected"
      );
   }
}
