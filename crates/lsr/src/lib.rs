//! Core logic for the `lsr` command line tool.
//!
//! `main.rs` only parses arguments and maps [`Error`] to an exit code;
//! everything else lives here so it can be unit tested directly.

use std::fmt;
use std::os::unix::fs::MetadataExt;
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

    /// Show permissions and size for each entry (the `-l` flag).
    ///
    /// `false` is the default, matching plain `ls`.
    pub long: bool,
}

/// Parses command-line arguments (excluding argv\[0\]) into [`Options`].
///
/// Only `-a` and `-l` are recognised for now; anything else is silently
/// ignored. Rejecting unknown flags is a separate step.
pub fn parse_options<I: IntoIterator<Item = String>>(args: I) -> Options {
    let mut options = Options::default();
    for arg in args {
        match arg.as_str() {
            "-a" => options.all = true,
            "-l" => options.long = true,
            _ => {}
        }
    }
    options
}

/// One entry in a directory listing.
#[derive(Debug)]
pub struct Entry {
    /// The entry's file name (not a full path).
    pub name: String,

    /// Metadata for this entry, present only when [`Options::long`] was set.
    ///
    /// Fetching it costs an extra `stat` call per entry, so plain listings
    /// skip it rather than fetching and discarding it.
    pub metadata: Option<std::fs::Metadata>,
}

/// Lists the entries directly inside `dir`, sorted alphabetically by name.
///
/// With `options.all == false` (the default), entries whose name starts
/// with `.` are skipped, matching plain `ls`. With `options.all == true`
/// they are included, matching `ls -a`.
///
/// # Errors
///
/// Returns [`Error::Io`] if `dir` cannot be read, or if reading any entry
/// (or its metadata, when `options.long` is set) fails partway through the
/// traversal.
pub fn list_entries(dir: &Path, options: Options) -> Result<Vec<Entry>, Error> {
    let mut entries = vec![];
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();

        if !options.all && name.starts_with('.') {
            continue;
        }

        let metadata = if options.long {
            Some(entry.metadata()?)
        } else {
            None
        };

        entries.push(Entry { name, metadata });
    }

    entries.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(entries)
}

/// Formats `entry` the way `main` prints it: just the name normally, or
/// `<permissions> <size> <name>` when `entry.metadata` is `Some` (i.e.
/// [`Options::long`] was set).
pub fn format_entry(entry: &Entry) -> String {
    match &entry.metadata {
        Some(metadata) => format!(
            "{} {} {}",
            format_permissions(metadata),
            metadata.len(),
            entry.name
        ),
        None => entry.name.clone(),
    }
}

/// Formats the 10-character permission string `ls -l` shows first, e.g.
/// `-rw-r--r--` for a regular file or `drwxr-xr-x` for a directory.
///
/// Owner/group *names* and the hard-link count are out of scope: turning a
/// uid into a username needs `libc`, which this project does not depend on.
fn format_permissions(metadata: &std::fs::Metadata) -> String {
    let file_type_char = if metadata.is_dir() {
        'd'
    } else if metadata.is_symlink() {
        'l'
    } else {
        '-'
    };

    fn permission_text(r: bool, w: bool, x: bool) -> String {
        format!(
            "{}{}{}",
            if r { 'r' } else { '-' },
            if w { 'w' } else { '-' },
            if x { 'x' } else { '-' }
        )
    }

    fn has_permission(mode: u32, mask: u32) -> bool {
        mode & mask != 0
    }

    let mode = metadata.mode();

    format!(
        "{}{}{}{}",
        file_type_char,
        permission_text(
            has_permission(mode, 0o400),
            has_permission(mode, 0o200),
            has_permission(mode, 0o100),
        ),
        permission_text(
            has_permission(mode, 0o040),
            has_permission(mode, 0o020),
            has_permission(mode, 0o010),
        ),
        permission_text(
            has_permission(mode, 0o004),
            has_permission(mode, 0o002),
            has_permission(mode, 0o001),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;

    /// Creates a fresh, empty temporary directory unique to `name` and
    /// returns its path. Callers are responsible for removing it again.
    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("lsr-test-{name}-{}", std::process::id()));
        fs::create_dir_all(&dir).expect("failed to create temp dir for test");
        dir
    }

    /// Extracts just the names from `entries`, in order.
    fn names_of(entries: &[Entry]) -> Vec<String> {
        entries.iter().map(|entry| entry.name.clone()).collect()
    }

    #[test]
    fn empty_directory_lists_nothing() {
        let dir = temp_dir("empty");

        let entries = list_entries(&dir, Options::default()).expect("list_entries should succeed");

        assert_eq!(names_of(&entries), Vec::<String>::new());

        fs::remove_dir_all(&dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn entries_are_sorted_alphabetically() {
        let dir = temp_dir("sorted");
        fs::write(dir.join("b.txt"), b"").expect("failed to create b.txt");
        fs::write(dir.join("a.txt"), b"").expect("failed to create a.txt");
        fs::write(dir.join("c.txt"), b"").expect("failed to create c.txt");

        let entries = list_entries(&dir, Options::default()).expect("list_entries should succeed");

        assert_eq!(names_of(&entries), vec!["a.txt", "b.txt", "c.txt"]);

        fs::remove_dir_all(&dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn nonexistent_directory_is_an_error() {
        let dir = std::env::temp_dir().join("lsr-test-this-should-not-exist");

        let result = list_entries(&dir, Options::default());

        assert!(result.is_err());
    }

    #[test]
    fn hidden_entries_are_skipped_by_default() {
        let dir = temp_dir("hidden-default");
        fs::write(dir.join("regular.txt"), b"").expect("failed to create regular.txt");
        fs::write(dir.join(".hidden"), b"").expect("failed to create .hidden");

        let entries = list_entries(&dir, Options::default()).expect("list_entries should succeed");

        assert_eq!(names_of(&entries), vec!["regular.txt"]);

        fs::remove_dir_all(&dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn hidden_entries_are_included_with_all_option() {
        let dir = temp_dir("hidden-all");
        fs::write(dir.join("regular.txt"), b"").expect("failed to create regular.txt");
        fs::write(dir.join(".hidden"), b"").expect("failed to create .hidden");

        let entries = list_entries(
            &dir,
            Options {
                all: true,
                ..Options::default()
            },
        )
        .expect("list_entries should succeed");

        assert_eq!(names_of(&entries), vec![".hidden", "regular.txt"]);

        fs::remove_dir_all(&dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn parse_options_defaults_to_all_false_and_long_false() {
        let options = parse_options(Vec::new());

        assert!(!options.all);
        assert!(!options.long);
    }

    #[test]
    fn parse_options_recognises_dash_a() {
        let options = parse_options(vec!["-a".to_string()]);

        assert!(options.all);
    }

    #[test]
    fn parse_options_recognises_dash_l() {
        let options = parse_options(vec!["-l".to_string()]);

        assert!(options.long);
    }

    #[test]
    fn long_option_populates_metadata() {
        let dir = temp_dir("long-metadata");
        fs::write(dir.join("file.txt"), b"").expect("failed to create file.txt");

        let entries = list_entries(
            &dir,
            Options {
                long: true,
                ..Options::default()
            },
        )
        .expect("list_entries should succeed");

        assert!(entries[0].metadata.is_some());

        fs::remove_dir_all(&dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn default_option_does_not_populate_metadata() {
        let dir = temp_dir("no-metadata");
        fs::write(dir.join("file.txt"), b"").expect("failed to create file.txt");

        let entries = list_entries(&dir, Options::default()).expect("list_entries should succeed");

        assert!(entries[0].metadata.is_none());

        fs::remove_dir_all(&dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn regular_file_permissions_are_formatted() {
        let dir = temp_dir("perm-file");
        let path = dir.join("file.txt");
        fs::write(&path, b"").expect("failed to create file.txt");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644))
            .expect("failed to set permissions");

        let metadata = fs::symlink_metadata(&path).expect("failed to read metadata");

        assert_eq!(format_permissions(&metadata), "-rw-r--r--");

        fs::remove_dir_all(&dir).expect("failed to clean up temp dir");
    }

    #[test]
    fn directory_permissions_start_with_d() {
        let dir = temp_dir("perm-dir");

        let metadata = fs::symlink_metadata(&dir).expect("failed to read metadata");

        assert_eq!(&format_permissions(&metadata)[..1], "d");

        fs::remove_dir_all(&dir).expect("failed to clean up temp dir");
    }
}
