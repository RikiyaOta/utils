use std::fmt;

/// Errors that can occur while listing a directory.
#[derive(Debug)]
pub enum Error {
    /// Reading the directory itself, or one of its entries, failed.
    Io(std::io::Error),

    /// A file's modification time predates the Unix epoch (1970-01-01).
    SystemTime(std::time::SystemTimeError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "{err}"),
            Self::SystemTime(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::SystemTime(err) => Some(err),
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
