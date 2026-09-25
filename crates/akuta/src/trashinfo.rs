//! The `.trashinfo` file format, from the freedesktop.org Trash spec:
//!
//! ```text
//! [Trash Info]
//! Path=/home/me/a%20b.txt
//! DeletionDate=2026-09-24T22:00:00
//! ```

use crate::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// The contents of one `.trashinfo` file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrashInfo {
    /// Where the entry was before it was trashed, as an absolute path.
    pub original_path: PathBuf,

    /// When it was trashed, as formatted by
    /// [`format_iso8601`](crate::datetime::format_iso8601).
    pub deletion_date: String,
}

/// Writes the `.trashinfo` file contents: the `[Trash Info]` header, then
/// `Path=` (percent-encoded with [`percent_encode`]) and `DeletionDate=`,
/// one per line, each line ending in `\n`.
impl fmt::Display for TrashInfo {
    #[expect(unused_variables, reason = "body is todo!(); delete once implemented")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("write the three lines of a .trashinfo file")
    }
}

/// Parses `.trashinfo` file contents.
///
/// - The first non-empty line must be `[Trash Info]`.
/// - `Path=` and `DeletionDate=` must both be present. `Path` is decoded
///   with [`percent_decode`] and must be absolute.
/// - Other `key=value` lines and blank lines are ignored, since other tools
///   may add keys of their own.
///
/// # Errors
///
/// Returns [`Error::InvalidTrashInfo`] if any rule above is broken.
impl FromStr for TrashInfo {
    type Err = Error;

    #[expect(unused_variables, reason = "body is todo!(); delete once implemented")]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!("read the header, then pick out the Path and DeletionDate values")
    }
}

/// Percent-encodes `path` for the `Path=` line of a `.trashinfo` file.
///
/// Works on the path's raw bytes. Bytes that are ASCII letters, digits, or
/// one of `-`, `.`, `_`, `~`, `/` are kept as they are; every other byte
/// becomes `%` followed by two uppercase hex digits. A multi-byte UTF-8
/// character therefore becomes several `%XX` groups.
#[expect(unused_variables, reason = "body is todo!(); delete once implemented")]
pub fn percent_encode(path: &Path) -> String {
    todo!("escape every byte outside the unreserved set as %XX")
}

/// Reverses [`percent_encode`]: every `%XX` becomes the byte `0xXX`, and
/// every other character is kept as it is. Hex digits may be upper- or
/// lowercase.
///
/// # Errors
///
/// Returns [`Error::InvalidTrashInfo`] if a `%` is not followed by two hex
/// digits.
#[expect(unused_variables, reason = "body is todo!(); delete once implemented")]
pub fn percent_decode(encoded: &str) -> Result<PathBuf, Error> {
    todo!("turn each %XX back into its byte, and the bytes into a PathBuf")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> TrashInfo {
        TrashInfo {
            original_path: PathBuf::from("/home/me/a b.txt"),
            deletion_date: "2026-09-24T22:00:00".to_string(),
        }
    }

    #[test]
    fn encode_keeps_unreserved_characters_and_slashes() {
        assert_eq!(
            percent_encode(Path::new("/home/me/A-z_0.9~")),
            "/home/me/A-z_0.9~"
        );
    }

    #[test]
    fn encode_escapes_space_and_percent() {
        assert_eq!(percent_encode(Path::new("/tmp/a b%c")), "/tmp/a%20b%25c");
    }

    #[test]
    fn encode_escapes_each_byte_of_a_multibyte_character() {
        // 芥 is U+82A5, which is E8 8A A5 in UTF-8.
        assert_eq!(percent_encode(Path::new("/tmp/芥")), "/tmp/%E8%8A%A5");
    }

    #[test]
    fn decode_reverses_encode() {
        let path = Path::new("/tmp/芥 a%b");

        assert_eq!(percent_decode(&percent_encode(path)).unwrap(), path);
    }

    #[test]
    fn decode_accepts_lowercase_hex() {
        assert_eq!(
            percent_decode("/tmp/%e8%8a%a5").unwrap(),
            PathBuf::from("/tmp/芥")
        );
    }

    #[test]
    fn decode_rejects_a_truncated_escape() {
        assert!(matches!(
            percent_decode("/tmp/a%2"),
            Err(Error::InvalidTrashInfo(_))
        ));
    }

    #[test]
    fn decode_rejects_non_hex_digits() {
        assert!(matches!(
            percent_decode("/tmp/%zz"),
            Err(Error::InvalidTrashInfo(_))
        ));
    }

    #[test]
    fn display_writes_the_spec_format() {
        assert_eq!(
            sample().to_string(),
            "[Trash Info]\nPath=/home/me/a%20b.txt\nDeletionDate=2026-09-24T22:00:00\n"
        );
    }

    #[test]
    fn parse_reads_path_and_date() {
        let info: TrashInfo =
            "[Trash Info]\nPath=/home/me/a%20b.txt\nDeletionDate=2026-09-24T22:00:00\n"
                .parse()
                .unwrap();

        assert_eq!(info, sample());
    }

    #[test]
    fn parse_reverses_display() {
        assert_eq!(sample().to_string().parse::<TrashInfo>().unwrap(), sample());
    }

    #[test]
    fn parse_ignores_unknown_keys_and_blank_lines() {
        let info: TrashInfo =
            "[Trash Info]\n\nX-Other=1\nDeletionDate=2026-09-24T22:00:00\nPath=/home/me/a%20b.txt\n"
                .parse()
                .unwrap();

        assert_eq!(info, sample());
    }

    #[test]
    fn parse_rejects_a_missing_header() {
        let result = "Path=/a\nDeletionDate=2026-09-24T22:00:00\n".parse::<TrashInfo>();

        assert!(matches!(result, Err(Error::InvalidTrashInfo(_))));
    }

    #[test]
    fn parse_rejects_a_missing_path() {
        let result = "[Trash Info]\nDeletionDate=2026-09-24T22:00:00\n".parse::<TrashInfo>();

        assert!(matches!(result, Err(Error::InvalidTrashInfo(_))));
    }

    #[test]
    fn parse_rejects_a_missing_deletion_date() {
        let result = "[Trash Info]\nPath=/a\n".parse::<TrashInfo>();

        assert!(matches!(result, Err(Error::InvalidTrashInfo(_))));
    }

    #[test]
    fn parse_rejects_a_relative_path() {
        let result =
            "[Trash Info]\nPath=a\nDeletionDate=2026-09-24T22:00:00\n".parse::<TrashInfo>();

        assert!(matches!(result, Err(Error::InvalidTrashInfo(_))));
    }
}
