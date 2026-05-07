use serde::{Serialize, ser::Serializer};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
   #[error("Invalid path: {0}")]
   InvalidPath(String),

   #[error("Unsupported platform: {0}")]
   UnsupportedPlatform(String),

   #[error("Incorrect OS: current OS is {0}, invoked for {1}")]
   IncorrectOS(String, String),

   #[error("Not implemented: {0}")]
   NotImplemented(String),
}

pub(crate) fn check_os(expected: &[&str], actual: &str) -> Result<()> {
   if !expected.contains(&actual) {
      return Err(Error::IncorrectOS(actual.to_string(), expected.join(", ")));
   }

   Ok(())
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
