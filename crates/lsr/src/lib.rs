//! Core logic for the `lsr` command line tool.
//!
//! `main.rs` only parses arguments and maps [`Error`] to an exit code;
//! everything else lives here so it can be unit tested directly.

use std::fmt;
use std::path::Path;

/// Errors that can occur while listing a directory.
#[derive(Debug)]
pub enum Error {
    /// Reading the directory itself, or one of its entries, failed.
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

/// Options controlling how a directory listing is produced.
#[derive(Debug, Default, Clone, Copy)]
pub struct Options {
    /// Include entries whose name starts with `.` (the `-a` flag).
    ///
    /// `false` is the default, matching plain `ls`.
    pub all: bool,
}

/// Parses command-line arguments (excluding argv\[0\]) into [`Options`].
///
/// Only `-a` is recognised for now; anything else is silently ignored.
/// Rejecting unknown flags is a separate step.
pub fn parse_options<I: IntoIterator<Item = String>>(args: I) -> Options {
    let mut options = Options::default();
    for arg in args {
        if arg == "-a" {
            options.all = true;
        }
    }
    options
}

/// Lists the names of the entries directly inside `dir`, sorted
/// alphabetically.
///
/// With `options.all == false` (the default), entries whose name starts
/// with `.` are skipped, matching plain `ls`. With `options.all == true`
/// they are included, matching `ls -a`.
///
/// No `-l` metadata yet; that is a separate step.
///
/// # Errors
///
/// Returns [`Error::Io`] if `dir` cannot be read, or if reading any entry
/// inside it fails partway through the traversal.
pub fn list_names(dir: &Path, options: Options) -> Result<Vec<String>, Error> {
    let mut names = vec![];
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();

        if !options.all && name.starts_with('.') {
            continue;
        }
        names.push(name);
    }

    names.sort();
    Ok(names)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    /// Creates a fresh, empty temporary directory unique to `name` and
    /// returns its path. Callers are responsible for removing it again.
    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("lsr-test-{name}-{}", std::process::id()));
        fs::create_dir_all(&dir).expect("failed to create temp dir for test");
        dir
    }

    #[test]
    fn empty_directory_lists_nothing() {
        let dir = temp_dir("empty");

        let names = list_names(&dir, Options::default()).expect("list_names should succeed");

        assert_eq!(names, Vec::<String>::new());

        fs::remove_dir_all(&dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn entries_are_sorted_alphabetically() {
        let dir = temp_dir("sorted");
        fs::write(dir.join("b.txt"), b"").expect("failed to create b.txt");
        fs::write(dir.join("a.txt"), b"").expect("failed to create a.txt");
        fs::write(dir.join("c.txt"), b"").expect("failed to create c.txt");

        let names = list_names(&dir, Options::default()).expect("list_names should succeed");

        assert_eq!(names, vec!["a.txt", "b.txt", "c.txt"]);

        fs::remove_dir_all(&dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn nonexistent_directory_is_an_error() {
        let dir = std::env::temp_dir().join("lsr-test-this-should-not-exist");

        let result = list_names(&dir, Options::default());

        assert!(result.is_err());
    }

    #[test]
    fn hidden_entries_are_skipped_by_default() {
        let dir = temp_dir("hidden-default");
        fs::write(dir.join("regular.txt"), b"").expect("failed to create regular.txt");
        fs::write(dir.join(".hidden"), b"").expect("failed to create .hidden");

        let names = list_names(&dir, Options::default()).expect("list_names should succeed");

        assert_eq!(names, vec!["regular.txt"]);

        fs::remove_dir_all(&dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn hidden_entries_are_included_with_all_option() {
        let dir = temp_dir("hidden-all");
        fs::write(dir.join("regular.txt"), b"").expect("failed to create regular.txt");
        fs::write(dir.join(".hidden"), b"").expect("failed to create .hidden");

        let names = list_names(&dir, Options { all: true }).expect("list_names should succeed");

        assert_eq!(names, vec![".hidden", "regular.txt"]);

        fs::remove_dir_all(&dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn parse_options_defaults_to_all_false() {
        let options = parse_options(Vec::new());

        assert!(!options.all);
    }

    #[test]
    fn parse_options_recognises_dash_a() {
        let options = parse_options(vec!["-a".to_string()]);

        assert!(options.all);
    }
}
