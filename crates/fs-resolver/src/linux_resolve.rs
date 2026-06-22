use crate::error::Error;
use crate::error::Result;
use crate::linux_paths::LinuxPath;
use std::path::PathBuf;

#[cfg(any(target_os = "linux", test))]
use std::path::Path;

pub(crate) fn resolve_linux_path(path: &LinuxPath, _bundle_identifier: &str) -> Result<PathBuf> {
   #[cfg(target_os = "linux")]
   {
      resolve_linux_path_inner(path, _bundle_identifier)
   }

   #[cfg(not(target_os = "linux"))]
   {
      Err(Error::IncorrectOS {
         path: path.to_string(),
         current_os: std::env::consts::OS.to_string(),
         expected_os: "linux".to_string(),
      })
   }
}

#[cfg(target_os = "linux")]
fn resolve_linux_path_inner(path: &LinuxPath, bundle_identifier: &str) -> Result<PathBuf> {
   match path {
      LinuxPath::Home => home_dir(path),
      LinuxPath::DataHome => xdg_base("XDG_DATA_HOME", ".local/share", path),
      LinuxPath::DataHomeForCurrentApp => {
         let path = xdg_base("XDG_DATA_HOME", ".local/share", path)?;
         Ok(path.join(bundle_identifier))
      }
      LinuxPath::ConfigHome => xdg_base("XDG_CONFIG_HOME", ".config", path),
      LinuxPath::ConfigHomeForCurrentApp => {
         let path = xdg_base("XDG_CONFIG_HOME", ".config", path)?;
         Ok(path.join(bundle_identifier))
      }
      LinuxPath::CacheHome => xdg_base("XDG_CACHE_HOME", ".cache", path),
      LinuxPath::CacheHomeForCurrentApp => {
         let path = xdg_base("XDG_CACHE_HOME", ".cache", path)?;
         Ok(path.join(bundle_identifier))
      }
      LinuxPath::StateHome => xdg_base("XDG_STATE_HOME", ".local/state", path),
      LinuxPath::StateHomeForCurrentApp => {
         let path = xdg_base("XDG_STATE_HOME", ".local/state", path)?;
         Ok(path.join(bundle_identifier))
      }
      // XDG_BIN_HOME / ~/.local/bin is a de-facto convention, not part of the XDG
      // Base Directory Specification (which only defines DATA/CONFIG/STATE/CACHE/RUNTIME).
      LinuxPath::ExecutableDir => xdg_base("XDG_BIN_HOME", ".local/bin", path),
      LinuxPath::FontDir => {
         let path = xdg_base("XDG_DATA_HOME", ".local/share", path)?;
         Ok(path.join("fonts"))
      }
      LinuxPath::RuntimeDir => runtime_dir(path),
      LinuxPath::DesktopDir => xdg_user_dir("DESKTOP", path),
      LinuxPath::DocumentDir => xdg_user_dir("DOCUMENTS", path),
      LinuxPath::DownloadDir => xdg_user_dir("DOWNLOAD", path),
      LinuxPath::MusicDir => xdg_user_dir("MUSIC", path),
      LinuxPath::PictureDir => xdg_user_dir("PICTURES", path),
      LinuxPath::VideoDir => xdg_user_dir("VIDEOS", path),
      LinuxPath::TemplateDir => xdg_user_dir("TEMPLATES", path),
      LinuxPath::PublicDir => xdg_user_dir("PUBLICSHARE", path),
   }
}

/// Resolves $HOME with getpwuid_r fallback (same pattern as dirs-sys).
#[cfg(target_os = "linux")]
fn home_dir(path: &LinuxPath) -> Result<PathBuf> {
   use std::env;

   if let Some(home) = env::var_os("HOME") {
      let home = PathBuf::from(home);
      if home.is_absolute() {
         return Ok(home);
      }
   }

   // Fallback: read home directory from /etc/passwd via getpwuid_r
   unsafe {
      let uid = libc::getuid();
      // _SC_GETPW_R_SIZE_MAX is only a hint and may be unavailable (-1). Start from it
      // (or 512) and double the buffer on ERANGE, matching the dirs-sys retry pattern,
      // so entries larger than the initial guess don't spuriously fail.
      let mut bufsize = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
         n if n < 0 => 512,
         n => n as usize,
      };
      const MAX_BUFSIZE: usize = 1 << 20;

      loop {
         let mut buf = vec![0u8; bufsize];
         let mut passwd: libc::passwd = std::mem::zeroed();
         let mut result: *mut libc::passwd = std::ptr::null_mut();

         let ret = libc::getpwuid_r(
            uid,
            &mut passwd,
            buf.as_mut_ptr() as *mut libc::c_char,
            buf.len(),
            &mut result,
         );

         if ret == libc::ERANGE && bufsize < MAX_BUFSIZE {
            bufsize *= 2;
            continue;
         }

         if ret == 0 && !result.is_null() {
            let dir = std::ffi::CStr::from_ptr((*result).pw_dir);
            let bytes = dir.to_bytes();
            if !bytes.is_empty() {
               use std::os::unix::ffi::OsStringExt;
               let ostr = std::ffi::OsString::from_vec(bytes.to_vec());
               return Ok(PathBuf::from(ostr));
            }
         }

         break;
      }
   }

   Err(Error::LinuxEnvironmentMissing {
      variable: "HOME".to_string(),
      path: path.to_string(),
      hint: "Ensure $HOME is exported or /etc/passwd contains the user entry.".to_string(),
   })
}

/// Resolves an XDG base directory: env var if set and absolute, else $HOME + fallback_relative.
#[cfg(target_os = "linux")]
fn xdg_base(env_var: &str, fallback_relative: &str, path: &LinuxPath) -> Result<PathBuf> {
   use std::env;

   if let Some(val) = env::var_os(env_var) {
      let p = PathBuf::from(val);
      if p.is_absolute() {
         return Ok(p);
      }
      // XDG spec: relative paths are invalid, ignore and use fallback
   }

   let home = home_dir(path)?;
   Ok(home.join(fallback_relative))
}

/// $XDG_RUNTIME_DIR has no fallback per spec; error if unset.
#[cfg(target_os = "linux")]
fn runtime_dir(path: &LinuxPath) -> Result<PathBuf> {
   use std::env;

   match env::var_os("XDG_RUNTIME_DIR") {
      Some(val) => {
         let p = PathBuf::from(val);
         if p.is_absolute() {
            Ok(p)
         } else {
            Err(Error::LinuxEnvironmentMissing {
               variable: "XDG_RUNTIME_DIR".to_string(),
               path: path.to_string(),
               hint: "XDG_RUNTIME_DIR is set but not absolute.".to_string(),
            })
         }
      }
      None => Err(Error::LinuxEnvironmentMissing {
         variable: "XDG_RUNTIME_DIR".to_string(),
         path: path.to_string(),
         hint: "This should be set by pam_systemd at login.".to_string(),
      }),
   }
}

/// Parses ~/.config/user-dirs.dirs to find XDG user directories.
/// File format: `XDG_{KEY}_DIR="$HOME/DirName"` or `XDG_{KEY}_DIR="/absolute/path"`
#[cfg(target_os = "linux")]
fn xdg_user_dir(key: &str, path: &LinuxPath) -> Result<PathBuf> {
   let config_home = xdg_base("XDG_CONFIG_HOME", ".config", path)?;
   let dirs_file = config_home.join("user-dirs.dirs");

   if let Ok(contents) = std::fs::read_to_string(&dirs_file) {
      let home = home_dir(path)?;
      if let Some(resolved) = parse_user_dirs(&contents, key, &home) {
         return Ok(resolved);
      }
   }

   Err(Error::LinuxEnvironmentMissing {
      variable: format!("XDG_{}_DIR", key),
      path: path.to_string(),
      hint: "Install xdg-user-dirs or create ~/.config/user-dirs.dirs.".to_string(),
   })
}

/// Pure parser for the `user-dirs.dirs` format: scans `contents` for the line
/// `XDG_{key}_DIR=<value>` and resolves the value relative to `home`.
///
/// Returns `None` when the key is absent. `$HOME` is only expanded on a path
/// boundary (`$HOME` or `$HOME/...`), so a value like `$HOMEBASE/x` is treated
/// as a literal path rather than `home + BASE/x`. Trailing inline comments and
/// surrounding double quotes are stripped.
#[cfg(any(target_os = "linux", test))]
fn parse_user_dirs(contents: &str, key: &str, home: &Path) -> Option<PathBuf> {
   let target_key = format!("XDG_{}_DIR", key);

   for line in contents.lines() {
      let line = line.trim();
      if line.starts_with('#') || line.is_empty() {
         continue;
      }

      let Some(rest) = line.strip_prefix(&target_key) else {
         continue;
      };
      // Require `=` immediately after the key (modulo whitespace) so keys like
      // XDG_DESKTOP_DIRECTORY don't match XDG_DESKTOP_DIR.
      let Some(rest) = rest.trim_start().strip_prefix('=') else {
         continue;
      };

      let value = extract_value(rest);
      if value.is_empty() {
         continue;
      }

      let resolved = if value == "$HOME" {
         home.to_path_buf()
      } else if let Some(rel) = value.strip_prefix("$HOME/") {
         home.join(rel)
      } else {
         PathBuf::from(value)
      };
      return Some(resolved);
   }

   None
}

/// Extracts the raw value after `=`: handles a double-quoted value (taking the
/// content up to the closing quote, ignoring any trailing inline comment) or an
/// unquoted value (up to the first whitespace or `#`).
#[cfg(any(target_os = "linux", test))]
fn extract_value(rest: &str) -> &str {
   let rest = rest.trim_start();
   if let Some(after) = rest.strip_prefix('"') {
      match after.find('"') {
         Some(end) => &after[..end],
         None => after,
      }
   } else {
      let end = rest
         .find(|c: char| c.is_whitespace() || c == '#')
         .unwrap_or(rest.len());
      rest[..end].trim_end()
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[cfg(target_os = "linux")]
   const BUNDLE_ID: &str = "com.example.app";

   fn home() -> PathBuf {
      PathBuf::from("/home/alice")
   }

   #[test]
   fn quoted_home_expansion() {
      let c = r#"XDG_DESKTOP_DIR="$HOME/Desktop""#;
      assert_eq!(
         parse_user_dirs(c, "DESKTOP", &home()),
         Some(PathBuf::from("/home/alice/Desktop"))
      );
   }

   #[test]
   fn bare_home_value() {
      let c = r#"XDG_DOWNLOAD_DIR="$HOME""#;
      assert_eq!(
         parse_user_dirs(c, "DOWNLOAD", &home()),
         Some(PathBuf::from("/home/alice"))
      );
   }

   #[test]
   fn absolute_path_value() {
      let c = r#"XDG_MUSIC_DIR="/mnt/media/music""#;
      assert_eq!(
         parse_user_dirs(c, "MUSIC", &home()),
         Some(PathBuf::from("/mnt/media/music"))
      );
   }

   #[test]
   fn unquoted_value() {
      let c = "XDG_PICTURES_DIR=$HOME/Pictures";
      assert_eq!(
         parse_user_dirs(c, "PICTURES", &home()),
         Some(PathBuf::from("/home/alice/Pictures"))
      );
   }

   #[test]
   fn trailing_inline_comment() {
      let c = r#"XDG_VIDEOS_DIR="$HOME/Videos" # my videos"#;
      assert_eq!(
         parse_user_dirs(c, "VIDEOS", &home()),
         Some(PathBuf::from("/home/alice/Videos"))
      );
   }

   #[test]
   fn home_prefix_is_boundary_aware() {
      // $HOMEBASE must NOT expand as $HOME + "BASE/x"; treated as literal.
      let c = r#"XDG_TEMPLATES_DIR="$HOMEBASE/templates""#;
      assert_eq!(
         parse_user_dirs(c, "TEMPLATES", &home()),
         Some(PathBuf::from("$HOMEBASE/templates"))
      );
   }

   #[test]
   fn skips_comments_and_blank_lines() {
      let c = "# user-dirs.dirs\n\n  # comment\nXDG_PUBLICSHARE_DIR=\"$HOME/Public\"\n";
      assert_eq!(
         parse_user_dirs(c, "PUBLICSHARE", &home()),
         Some(PathBuf::from("/home/alice/Public"))
      );
   }

   #[test]
   fn missing_key_returns_none() {
      let c = r#"XDG_DESKTOP_DIR="$HOME/Desktop""#;
      assert_eq!(parse_user_dirs(c, "DOWNLOAD", &home()), None);
   }

   #[test]
   fn similar_key_does_not_match() {
      // XDG_DESKTOP_DIRECTORY must not satisfy a lookup for DESKTOP.
      let c = r#"XDG_DESKTOP_DIRECTORY="$HOME/Nope""#;
      assert_eq!(parse_user_dirs(c, "DESKTOP", &home()), None);
   }

   #[test]
   fn empty_value_skipped() {
      let c = "XDG_MUSIC_DIR=\nXDG_MUSIC_DIR=\"$HOME/Music\"";
      assert_eq!(
         parse_user_dirs(c, "MUSIC", &home()),
         Some(PathBuf::from("/home/alice/Music"))
      );
   }

   #[cfg(target_os = "linux")]
   #[test]
   fn data_home_for_current_app_appends_bundle_identifier() {
      let root = resolve_linux_path(&LinuxPath::DataHome, "").unwrap();
      let path = resolve_linux_path(&LinuxPath::DataHomeForCurrentApp, BUNDLE_ID).unwrap();
      assert_eq!(path, root.join(BUNDLE_ID));
   }

   #[cfg(target_os = "linux")]
   #[test]
   fn config_home_for_current_app_appends_bundle_identifier() {
      let root = resolve_linux_path(&LinuxPath::ConfigHome, "").unwrap();
      let path = resolve_linux_path(&LinuxPath::ConfigHomeForCurrentApp, BUNDLE_ID).unwrap();
      assert_eq!(path, root.join(BUNDLE_ID));
   }

   #[cfg(target_os = "linux")]
   #[test]
   fn cache_home_for_current_app_appends_bundle_identifier() {
      let root = resolve_linux_path(&LinuxPath::CacheHome, "").unwrap();
      let path = resolve_linux_path(&LinuxPath::CacheHomeForCurrentApp, BUNDLE_ID).unwrap();
      assert_eq!(path, root.join(BUNDLE_ID));
   }

   #[cfg(target_os = "linux")]
   #[test]
   fn state_home_for_current_app_appends_bundle_identifier() {
      let root = resolve_linux_path(&LinuxPath::StateHome, "").unwrap();
      let path = resolve_linux_path(&LinuxPath::StateHomeForCurrentApp, BUNDLE_ID).unwrap();
      assert_eq!(path, root.join(BUNDLE_ID));
   }
}
