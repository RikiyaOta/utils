//! Turning a user-supplied path into something safe to trash.

use crate::error::Error;
use std::path::{Path, PathBuf};

/// Turns `raw`, a path as typed by the user, into the absolute path of the
/// entry to trash.
///
/// - Refuses a path whose last component is `.` or `..`, and a path with no
///   last component at all (`/`), before anything else. This must happen on
///   `raw` itself: once made absolute, `.` would silently become the
///   current directory.
/// - Resolves symlinks in the *parent* directories (with
///   `std::fs::canonicalize`) but not in the last component, so that a
///   symlink is trashed itself rather than whatever it points to. A
///   relative `raw` is taken relative to the current directory.
/// - Checks with `std::fs::symlink_metadata` (`lstat`) that the entry
///   exists. Unlike `metadata` (`stat`), this does not follow the last
///   component, so a broken symlink still counts as existing.
///
/// # Errors
///
/// Returns [`Error::Protected`] for `.`, `..` and `/`, and [`Error::Io`] if
/// the parent directory cannot be resolved or the entry does not exist.
#[expect(unused_variables, reason = "body is todo!(); delete once implemented")]
pub fn resolve_target(raw: &Path) -> Result<PathBuf, Error> {
    todo!("reject ./../, canonicalize the parent, re-attach the file name, lstat it")
}

/// Refuses to trash `target` if it is one of the paths that must never be
/// touched:
///
/// - `/` and `/etc`;
/// - `home`, the user's home directory, if known;
/// - `trash_root`, and anything inside it.
///
/// `target` is expected to come from [`resolve_target`], and `home` and
/// `trash_root` to be canonical too, so that plain path comparison is
/// enough: no `..` or symlinks are left to sneak past it.
///
/// # Errors
///
/// Returns [`Error::Protected`] if `target` is one of the paths above.
#[expect(unused_variables, reason = "body is todo!(); delete once implemented")]
pub fn check_target(target: &Path, home: Option<&Path>, trash_root: &Path) -> Result<(), Error> {
    todo!("compare target against each protected path")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::temp_dir;
    use std::fs;
    use std::os::unix::fs::symlink;

    // resolve_target

    #[test]
    fn dot_is_protected() {
        assert!(matches!(
            resolve_target(Path::new(".")),
            Err(Error::Protected)
        ));
    }

    #[test]
    fn dot_dot_is_protected() {
        assert!(matches!(
            resolve_target(Path::new("..")),
            Err(Error::Protected)
        ));
    }

    #[test]
    fn trailing_dot_dot_is_protected() {
        let dir = temp_dir("trailing-dot-dot");

        assert!(matches!(
            resolve_target(&dir.join("..")),
            Err(Error::Protected)
        ));

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn root_is_protected() {
        assert!(matches!(
            resolve_target(Path::new("/")),
            Err(Error::Protected)
        ));
    }

    #[test]
    fn existing_file_resolves_to_its_absolute_path() {
        let dir = temp_dir("existing");
        fs::write(dir.join("a.txt"), b"").unwrap();

        let target = resolve_target(&dir.join("a.txt")).unwrap();

        // The temp dir itself may sit behind a symlink (/tmp → /private/tmp
        // on macOS), so compare against its canonical form.
        assert_eq!(target, fs::canonicalize(&dir).unwrap().join("a.txt"));

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn symlinked_parent_directory_is_resolved() {
        let dir = temp_dir("symlinked-parent");
        fs::create_dir(dir.join("real")).unwrap();
        fs::write(dir.join("real").join("a.txt"), b"").unwrap();
        symlink(dir.join("real"), dir.join("link")).unwrap();

        let target = resolve_target(&dir.join("link").join("a.txt")).unwrap();

        assert_eq!(
            target,
            fs::canonicalize(&dir).unwrap().join("real").join("a.txt")
        );

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn symlink_itself_is_the_target_not_what_it_points_to() {
        let dir = temp_dir("symlink-itself");
        fs::write(dir.join("real.txt"), b"").unwrap();
        symlink(dir.join("real.txt"), dir.join("link.txt")).unwrap();

        let target = resolve_target(&dir.join("link.txt")).unwrap();

        assert_eq!(target, fs::canonicalize(&dir).unwrap().join("link.txt"));

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn broken_symlink_can_be_resolved() {
        let dir = temp_dir("broken-symlink");
        symlink(dir.join("nowhere"), dir.join("broken")).unwrap();

        let target = resolve_target(&dir.join("broken")).unwrap();

        assert_eq!(target, fs::canonicalize(&dir).unwrap().join("broken"));

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn missing_path_is_an_error() {
        let dir = temp_dir("missing");

        assert!(matches!(
            resolve_target(&dir.join("nope")),
            Err(Error::Io(_))
        ));

        fs::remove_dir_all(&dir).unwrap();
    }

    // check_target

    const HOME: &str = "/home/me";
    const TRASH: &str = "/home/me/.local/share/Trash";

    fn check(target: &str) -> Result<(), Error> {
        check_target(Path::new(target), Some(Path::new(HOME)), Path::new(TRASH))
    }

    #[test]
    fn ordinary_file_is_allowed() {
        assert!(check("/home/me/a.txt").is_ok());
    }

    #[test]
    fn root_is_refused() {
        assert!(matches!(check("/"), Err(Error::Protected)));
    }

    #[test]
    fn etc_is_refused() {
        assert!(matches!(check("/etc"), Err(Error::Protected)));
    }

    #[test]
    fn a_file_inside_etc_is_allowed() {
        // Only /etc itself is protected; trashing a single file under it is
        // still an ordinary (if unusual) request.
        assert!(check("/etc/hosts").is_ok());
    }

    #[test]
    fn home_is_refused() {
        assert!(matches!(check(HOME), Err(Error::Protected)));
    }

    #[test]
    fn home_is_not_checked_when_unknown() {
        assert!(check_target(Path::new(HOME), None, Path::new(TRASH)).is_ok());
    }

    #[test]
    fn trash_root_is_refused() {
        assert!(matches!(check(TRASH), Err(Error::Protected)));
    }

    #[test]
    fn anything_inside_the_trash_is_refused() {
        assert!(matches!(
            check("/home/me/.local/share/Trash/files/a.txt"),
            Err(Error::Protected)
        ));
    }

    #[test]
    fn a_sibling_sharing_the_trash_prefix_is_allowed() {
        // "Trash-old" starts with the same characters as "Trash" but is not
        // inside it; the comparison must be by path component, not by text.
        assert!(check("/home/me/.local/share/Trash-old").is_ok());
    }
}
