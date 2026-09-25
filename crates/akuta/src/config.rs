//! Locating the trash directory: the config file, then the XDG variables.

use crate::error::Error;
use std::path::{Path, PathBuf};

/// The environment variables `akuta` reads, captured once in `main`.
///
/// Passing these around explicitly, instead of calling `std::env::var_os`
/// deep inside the library, is what lets the tests try every combination
/// without touching the real process environment.
#[derive(Debug, Default, Clone)]
pub struct Env {
    /// `$HOME`.
    pub home: Option<PathBuf>,
    /// `$XDG_CONFIG_HOME`.
    pub xdg_config_home: Option<PathBuf>,
    /// `$XDG_DATA_HOME`.
    pub xdg_data_home: Option<PathBuf>,
}

impl Env {
    /// Reads the variables from the current process. Values are taken as
    /// they are; deciding which ones are usable is left to the functions
    /// that consume them.
    pub fn from_process() -> Self {
        Self {
            home: std::env::var_os("HOME").map(PathBuf::from),
            xdg_config_home: std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from),
            xdg_data_home: std::env::var_os("XDG_DATA_HOME").map(PathBuf::from),
        }
    }
}

/// Settings read from the config file.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Config {
    /// Where the trash lives, overriding the XDG default. Always absolute.
    pub trash_dir: Option<PathBuf>,
}

/// Returns where the config file should be:
/// `$XDG_CONFIG_HOME/akuta/config`, or `$HOME/.config/akuta/config` when
/// `$XDG_CONFIG_HOME` is unset, empty or relative (the XDG Base Directory
/// spec says to ignore a relative value). Returns `None` when neither
/// variable gives an absolute path.
///
/// This only builds the path; it does not check that the file exists.
#[expect(unused_variables, reason = "body is todo!(); delete once implemented")]
pub fn config_file_path(env: &Env) -> Option<PathBuf> {
    todo!("pick $XDG_CONFIG_HOME if usable, otherwise fall back to $HOME/.config")
}

/// Reads and parses the config file at `path`. A file that does not exist
/// is not an error: it gives [`Config::default`].
///
/// # Errors
///
/// Returns [`Error::Io`] if the file exists but cannot be read, and
/// whatever [`parse_config`] returns if it cannot be parsed.
#[expect(unused_variables, reason = "body is todo!(); delete once implemented")]
pub fn load_config(path: &Path) -> Result<Config, Error> {
    todo!("read the file, treating 'not found' as an empty config")
}

/// Parses config file contents.
///
/// Each line is one of:
///
/// - blank, or starting with `#` (after leading whitespace): ignored;
/// - `key = value`: whitespace around the key and the value is trimmed.
///
/// The only key is `trash_dir`, whose value must be an absolute path. If a
/// key appears twice, the last one wins.
///
/// # Errors
///
/// Returns [`Error::Config`], carrying the 1-based line number, for a line
/// with no `=`, an unknown key, or a `trash_dir` that is not absolute.
#[expect(unused_variables, reason = "body is todo!(); delete once implemented")]
pub fn parse_config(contents: &str) -> Result<Config, Error> {
    todo!("go line by line, skipping comments, and split each line on '='")
}

/// Decides where the trash directory is, in this order of preference:
///
/// 1. `config.trash_dir`;
/// 2. `$XDG_DATA_HOME/Trash`, if `$XDG_DATA_HOME` is set, non-empty and
///    absolute;
/// 3. `$HOME/.local/share/Trash`, if `$HOME` is set, non-empty and absolute.
///
/// # Errors
///
/// Returns [`Error::NoTrashDir`] if none of them applies.
#[expect(unused_variables, reason = "body is todo!(); delete once implemented")]
pub fn resolve_trash_dir(config: &Config, env: &Env) -> Result<PathBuf, Error> {
    todo!("try the config, then $XDG_DATA_HOME, then $HOME")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::temp_dir;
    use std::fs;

    fn env(home: Option<&str>, xdg_config: Option<&str>, xdg_data: Option<&str>) -> Env {
        Env {
            home: home.map(PathBuf::from),
            xdg_config_home: xdg_config.map(PathBuf::from),
            xdg_data_home: xdg_data.map(PathBuf::from),
        }
    }

    // config_file_path

    #[test]
    fn config_file_path_prefers_xdg_config_home() {
        let env = env(Some("/home/me"), Some("/xdg/config"), None);

        assert_eq!(
            config_file_path(&env),
            Some(PathBuf::from("/xdg/config/akuta/config"))
        );
    }

    #[test]
    fn config_file_path_falls_back_to_home() {
        let env = env(Some("/home/me"), None, None);

        assert_eq!(
            config_file_path(&env),
            Some(PathBuf::from("/home/me/.config/akuta/config"))
        );
    }

    #[test]
    fn config_file_path_ignores_empty_xdg_config_home() {
        let env = env(Some("/home/me"), Some(""), None);

        assert_eq!(
            config_file_path(&env),
            Some(PathBuf::from("/home/me/.config/akuta/config"))
        );
    }

    #[test]
    fn config_file_path_ignores_relative_xdg_config_home() {
        let env = env(Some("/home/me"), Some("relative"), None);

        assert_eq!(
            config_file_path(&env),
            Some(PathBuf::from("/home/me/.config/akuta/config"))
        );
    }

    #[test]
    fn config_file_path_is_none_without_home() {
        assert_eq!(config_file_path(&env(None, None, None)), None);
    }

    // load_config

    #[test]
    fn load_config_of_a_missing_file_is_the_default() {
        let dir = temp_dir("missing-config");

        let config = load_config(&dir.join("config")).unwrap();

        assert_eq!(config, Config::default());

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn load_config_reads_the_file() {
        let dir = temp_dir("config");
        fs::write(dir.join("config"), "trash_dir = /elsewhere/Trash\n").unwrap();

        let config = load_config(&dir.join("config")).unwrap();

        assert_eq!(config.trash_dir, Some(PathBuf::from("/elsewhere/Trash")));

        fs::remove_dir_all(&dir).unwrap();
    }

    // parse_config

    #[test]
    fn empty_config_is_the_default() {
        assert_eq!(parse_config("").unwrap(), Config::default());
    }

    #[test]
    fn trash_dir_is_read() {
        let config = parse_config("trash_dir=/elsewhere/Trash\n").unwrap();

        assert_eq!(config.trash_dir, Some(PathBuf::from("/elsewhere/Trash")));
    }

    #[test]
    fn whitespace_around_key_and_value_is_trimmed() {
        let config = parse_config("  trash_dir   =   /elsewhere/Trash  \n").unwrap();

        assert_eq!(config.trash_dir, Some(PathBuf::from("/elsewhere/Trash")));
    }

    #[test]
    fn comments_and_blank_lines_are_ignored() {
        let config = parse_config("# a comment\n\n   # indented\ntrash_dir = /t\n").unwrap();

        assert_eq!(config.trash_dir, Some(PathBuf::from("/t")));
    }

    #[test]
    fn last_value_wins() {
        let config = parse_config("trash_dir = /first\ntrash_dir = /second\n").unwrap();

        assert_eq!(config.trash_dir, Some(PathBuf::from("/second")));
    }

    #[test]
    fn unknown_key_is_an_error_with_its_line_number() {
        let result = parse_config("# comment\ncolour = red\n");

        assert!(matches!(result, Err(Error::Config { line: 2, .. })));
    }

    #[test]
    fn line_without_equals_is_an_error() {
        let result = parse_config("trash_dir /t\n");

        assert!(matches!(result, Err(Error::Config { line: 1, .. })));
    }

    #[test]
    fn relative_trash_dir_is_an_error() {
        let result = parse_config("trash_dir = Trash\n");

        assert!(matches!(result, Err(Error::Config { line: 1, .. })));
    }

    // resolve_trash_dir

    #[test]
    fn trash_dir_from_config_wins() {
        let config = Config {
            trash_dir: Some(PathBuf::from("/configured")),
        };
        let env = env(Some("/home/me"), None, Some("/xdg/data"));

        assert_eq!(
            resolve_trash_dir(&config, &env).unwrap(),
            PathBuf::from("/configured")
        );
    }

    #[test]
    fn trash_dir_under_xdg_data_home() {
        let env = env(Some("/home/me"), None, Some("/xdg/data"));

        assert_eq!(
            resolve_trash_dir(&Config::default(), &env).unwrap(),
            PathBuf::from("/xdg/data/Trash")
        );
    }

    #[test]
    fn trash_dir_falls_back_to_home() {
        let env = env(Some("/home/me"), None, None);

        assert_eq!(
            resolve_trash_dir(&Config::default(), &env).unwrap(),
            PathBuf::from("/home/me/.local/share/Trash")
        );
    }

    #[test]
    fn trash_dir_ignores_relative_xdg_data_home() {
        let env = env(Some("/home/me"), None, Some("relative"));

        assert_eq!(
            resolve_trash_dir(&Config::default(), &env).unwrap(),
            PathBuf::from("/home/me/.local/share/Trash")
        );
    }

    #[test]
    fn no_trash_dir_without_home() {
        let result = resolve_trash_dir(&Config::default(), &env(None, None, None));

        assert!(matches!(result, Err(Error::NoTrashDir)));
    }
}
