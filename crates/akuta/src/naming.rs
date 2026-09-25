use std::ffi::{OsStr, OsString};

/// Returns the `n`th candidate name derived from `base`.
///
/// `n == 0` is `base` itself; `n >= 1` appends `_n` at the very end, after
/// any extension: `a.txt` → `a.txt_1`, `a.txt_2`, ….
///
/// Used both to pick a free name inside the trash and to suggest a new name
/// when restoring onto an existing file.
#[expect(unused_variables, reason = "body is todo!(); delete once implemented")]
pub fn numbered(base: &OsStr, n: u32) -> OsString {
    todo!("return base unchanged for 0, base followed by `_n` otherwise")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_the_base_name() {
        assert_eq!(numbered(OsStr::new("a.txt"), 0), "a.txt");
    }

    #[test]
    fn number_is_appended_after_an_underscore() {
        assert_eq!(numbered(OsStr::new("file"), 1), "file_1");
    }

    #[test]
    fn number_goes_after_the_extension() {
        assert_eq!(numbered(OsStr::new("a.txt"), 2), "a.txt_2");
    }

    #[test]
    fn dotfiles_are_not_special() {
        assert_eq!(numbered(OsStr::new(".bashrc"), 1), ".bashrc_1");
    }

    #[test]
    fn multi_digit_numbers_are_written_in_full() {
        assert_eq!(numbered(OsStr::new("a"), 12), "a_12");
    }
}
