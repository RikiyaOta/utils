//! Core logic for the `akuta` command line tool.
//!
//! `main.rs` only parses arguments and maps [`Error`] to an exit code;
//! everything else lives here so it can be unit tested directly.
//!
//! The specification is issue #41:
//! <https://github.com/RikiyaOta/utils/issues/41>.

pub mod args;
pub mod config;
pub mod datetime;
pub mod error;
pub mod naming;
pub mod safety;
#[cfg(test)]
mod test_support;
pub mod trash;
pub mod trashinfo;

use error::Error;
use std::ffi::OsString;
use std::path::Path;
use std::time::SystemTime;
use trash::{Trash, TrashEntry};

/// Moves `raw`, a path as the user typed it, into `trash`: resolves it,
/// refuses it if it is protected, then puts it in the trash. Returns the
/// name it was given there.
///
/// `home` is the user's home directory, if known; it is protected from
/// deletion.
///
/// # Errors
///
/// Returns whatever [`safety::resolve_target`], [`safety::check_target`] or
/// [`Trash::put`] returns, and [`Error::Io`] if the trash root cannot be
/// canonicalized.
pub fn trash_path(
    raw: &Path,
    trash: &Trash,
    home: Option<&Path>,
    now: SystemTime,
) -> Result<OsString, Error> {
    let target = safety::resolve_target(raw)?;
    // Canonical forms, so the comparisons in `check_target` cannot be fooled
    // by symlinks. The trash root exists by now: `main` calls `ensure_dirs`
    // first. A home directory that cannot be canonicalized cannot be the
    // target either, so it is simply left out of the check.
    let trash_root = std::fs::canonicalize(trash.root())?;
    let home = home.and_then(|home| std::fs::canonicalize(home).ok());
    safety::check_target(&target, home.as_deref(), &trash_root)?;
    trash.put(&target, now)
}

/// Formats one line of `akuta --list`: the deletion date, the name in the
/// trash (what `--restore` takes), and the original path, separated by two
/// spaces:
///
/// ```text
/// 2026-09-24T22:00:00  a.txt_1  /home/me/a.txt
/// ```
#[expect(unused_variables, reason = "body is todo!(); delete once implemented")]
pub fn format_entry(entry: &TrashEntry) -> String {
    todo!("join the three fields with two spaces")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::temp_dir;
    use crate::trashinfo::TrashInfo;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn format_entry_shows_date_name_and_original_path() {
        let entry = TrashEntry {
            name: OsString::from("a.txt_1"),
            info: TrashInfo {
                original_path: PathBuf::from("/home/me/a.txt"),
                deletion_date: "2026-09-24T22:00:00".to_string(),
            },
        };

        assert_eq!(
            format_entry(&entry),
            "2026-09-24T22:00:00  a.txt_1  /home/me/a.txt"
        );
    }

    #[test]
    fn trash_path_moves_a_file_into_the_trash() {
        let dir = temp_dir("trash-path");
        let trash = Trash::new(dir.join("Trash"));
        trash.ensure_dirs().unwrap();
        fs::write(dir.join("a.txt"), "").unwrap();

        let name = trash_path(&dir.join("a.txt"), &trash, None, SystemTime::now()).unwrap();

        assert_eq!(name, "a.txt");
        assert!(!dir.join("a.txt").exists());

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn trash_path_refuses_the_trash_itself() {
        let dir = temp_dir("trash-path-protected");
        let trash = Trash::new(dir.join("Trash"));
        trash.ensure_dirs().unwrap();

        let result = trash_path(&trash.files_dir(), &trash, None, SystemTime::now());

        assert!(matches!(result, Err(Error::Protected)));
        assert!(trash.files_dir().exists());

        fs::remove_dir_all(&dir).unwrap();
    }
}
