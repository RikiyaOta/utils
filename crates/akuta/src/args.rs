use crate::error::Error;
use std::ffi::OsString;
use std::path::PathBuf;

/// What the user asked `akuta` to do.
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    /// Move each path to the trash: `akuta <path>...`.
    Put(Vec<PathBuf>),

    /// List the trash, newest first: `akuta --list`.
    List,

    /// Restore one entry, by its name inside the trash:
    /// `akuta --restore <name>`.
    Restore(OsString),
}

/// Parses command-line arguments (excluding argv\[0\]) into a [`Command`].
///
/// The rules, in the order they apply:
///
/// - `--list` must be the only argument, and gives [`Command::List`].
/// - `--restore` must be followed by exactly one name, and gives
///   [`Command::Restore`].
/// - Otherwise every argument is a path, giving [`Command::Put`], except:
///   - `rm`'s flags are accepted and ignored, so `alias rm=akuta` keeps
///     `rm -rf dir` working. That is a short cluster made only of the
///     letters `r`, `R`, `f`, `i`, `v`, `d` (`-r`, `-rf`, `-Rfv`, …), and
///     the long flags `--recursive`, `--force`, `--interactive`, `--dir`
///     and `--verbose`.
///   - `--` ends flag parsing: every argument after it is a path, even one
///     that starts with `-`.
///   - A lone `-` is a path, as it is for `rm`.
///   - Any other argument starting with `-` is an unknown flag.
///   - At least one path is required.
///
/// # Errors
///
/// Returns [`Error::Usage`] for an unknown flag, a missing path or
/// `--restore` name, or extra arguments after `--list` / `--restore`.
#[expect(unused_variables, reason = "body is todo!(); delete once implemented")]
pub fn parse_args<I: IntoIterator<Item = String>>(args: I) -> Result<Command, Error> {
    todo!("turn the argument list into a Command, following the rules above")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parses string literals, for brevity.
    fn parse(args: &[&str]) -> Result<Command, Error> {
        parse_args(args.iter().map(|arg| (*arg).to_string()))
    }

    /// Builds the [`Command::Put`] expected for `paths`.
    fn put(paths: &[&str]) -> Command {
        Command::Put(paths.iter().map(PathBuf::from).collect())
    }

    #[test]
    fn no_arguments_is_a_usage_error() {
        assert!(matches!(parse(&[]), Err(Error::Usage(_))));
    }

    #[test]
    fn plain_arguments_are_paths() {
        assert_eq!(parse(&["a.txt", "dir"]).unwrap(), put(&["a.txt", "dir"]));
    }

    #[test]
    fn separate_rm_flags_are_ignored() {
        assert_eq!(
            parse(&["-r", "-f", "-i", "-v", "-d", "-R", "a.txt"]).unwrap(),
            put(&["a.txt"])
        );
    }

    #[test]
    fn clustered_rm_flags_are_ignored() {
        assert_eq!(parse(&["-rf", "dir"]).unwrap(), put(&["dir"]));
    }

    #[test]
    fn long_rm_flags_are_ignored() {
        assert_eq!(
            parse(&[
                "--recursive",
                "--force",
                "--interactive",
                "--dir",
                "--verbose",
                "a.txt"
            ])
            .unwrap(),
            put(&["a.txt"])
        );
    }

    #[test]
    fn flags_may_follow_paths() {
        assert_eq!(parse(&["a.txt", "-rf"]).unwrap(), put(&["a.txt"]));
    }

    #[test]
    fn only_rm_flags_is_a_usage_error() {
        assert!(matches!(parse(&["-rf"]), Err(Error::Usage(_))));
    }

    #[test]
    fn double_dash_makes_the_rest_paths() {
        assert_eq!(
            parse(&["--", "-rf", "--list"]).unwrap(),
            put(&["-rf", "--list"])
        );
    }

    #[test]
    fn lone_dash_is_a_path() {
        assert_eq!(parse(&["-"]).unwrap(), put(&["-"]));
    }

    #[test]
    fn unknown_short_flag_is_a_usage_error() {
        assert!(matches!(parse(&["-x", "a.txt"]), Err(Error::Usage(_))));
    }

    #[test]
    fn short_cluster_with_an_unknown_letter_is_a_usage_error() {
        assert!(matches!(parse(&["-rx", "a.txt"]), Err(Error::Usage(_))));
    }

    #[test]
    fn unknown_long_flag_is_a_usage_error() {
        assert!(matches!(parse(&["--bogus", "a.txt"]), Err(Error::Usage(_))));
    }

    #[test]
    fn list_flag_gives_list() {
        assert_eq!(parse(&["--list"]).unwrap(), Command::List);
    }

    #[test]
    fn list_with_extra_arguments_is_a_usage_error() {
        assert!(matches!(parse(&["--list", "a.txt"]), Err(Error::Usage(_))));
    }

    #[test]
    fn restore_flag_takes_one_name() {
        assert_eq!(
            parse(&["--restore", "a.txt_1"]).unwrap(),
            Command::Restore(OsString::from("a.txt_1"))
        );
    }

    #[test]
    fn restore_without_a_name_is_a_usage_error() {
        assert!(matches!(parse(&["--restore"]), Err(Error::Usage(_))));
    }

    #[test]
    fn restore_with_two_names_is_a_usage_error() {
        assert!(matches!(
            parse(&["--restore", "a", "b"]),
            Err(Error::Usage(_))
        ));
    }
}
