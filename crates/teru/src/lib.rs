//! Core logic for the `teru` command line tool.
//!
//! `main.rs` only parses arguments and maps [`Error`] to an exit code;
//! everything else lives here so it can be unit tested directly.

pub mod args;
mod datetime;
pub mod error;
pub mod format;
#[cfg(test)]
mod test_support;

use args::Options;
use error::Error;
use std::path::Path;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::temp_dir;
    use std::fs;

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
        let dir = std::env::temp_dir().join("teru-test-this-should-not-exist");

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
}
