use std::ffi::OsString;
use std::fmt;
use std::path::PathBuf;

/// Errors that can occur while trashing, listing or restoring files.
#[derive(Debug)]
pub enum Error {
    /// A filesystem operation failed.
    Io(std::io::Error),

    /// The command line could not be understood. The message says why.
    Usage(String),

    /// The config file could not be parsed.
    Config {
        /// 1-based line number of the offending line.
        line: usize,
        /// What is wrong with it.
        message: String,
    },

    /// Neither the config file, `$XDG_DATA_HOME` nor `$HOME` says where the
    /// trash directory is.
    NoTrashDir,

    /// Refused to trash a path that is too dangerous to touch: `/`, `/etc`,
    /// the home directory, the trash itself, or a path ending in `.`/`..`.
    Protected,

    /// A `.trashinfo` file is malformed. The message says how.
    InvalidTrashInfo(String),

    /// `--restore` named an entry that is not in the trash.
    NotInTrash(OsString),

    /// The directory an entry should be restored into no longer exists.
    RestoreDirMissing(PathBuf),

    /// Standard input closed while waiting for an answer to a prompt.
    Aborted,

    /// A timestamp predates the Unix epoch (1970-01-01).
    SystemTime(std::time::SystemTimeError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "{err}"),
            Self::Usage(message) => write!(f, "{message}"),
            Self::Config { line, message } => write!(f, "config file, line {line}: {message}"),
            Self::NoTrashDir => write!(
                f,
                "cannot locate the trash directory: set trash_dir in the config file, or $XDG_DATA_HOME or $HOME"
            ),
            Self::Protected => write!(f, "refusing to remove a protected path"),
            Self::InvalidTrashInfo(message) => write!(f, "invalid .trashinfo file: {message}"),
            Self::NotInTrash(name) => {
                write!(
                    f,
                    "no entry named '{}' in the trash",
                    name.to_string_lossy()
                )
            }
            Self::RestoreDirMissing(dir) => {
                write!(
                    f,
                    "cannot restore into '{}': no such directory",
                    dir.display()
                )
            }
            Self::Aborted => write!(f, "aborted: no answer on standard input"),
            Self::SystemTime(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::SystemTime(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(err: std::time::SystemTimeError) -> Self {
        Self::SystemTime(err)
    }
}
