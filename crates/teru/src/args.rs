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

#[cfg(test)]
mod tests {
    use super::*;

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
}
