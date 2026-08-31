//! Test-only utilities shared across this crate's `#[cfg(test)]` modules.

use std::path::PathBuf;

/// Creates a fresh, empty temporary directory unique to this call site and
/// `name`, and returns its path. Callers are responsible for removing it
/// again.
///
/// Uniqueness across the whole crate relies on `#[track_caller]`: the
/// directory name embeds the caller's file and line, so two tests in
/// different modules (or the same one) can both pass e.g. `"empty"`
/// without colliding.
#[track_caller]
pub(crate) fn temp_dir(name: &str) -> PathBuf {
    let location = std::panic::Location::caller();
    let file = location.file().replace(['/', '\\'], "_");

    let dir = std::env::temp_dir().join(format!(
        "lsr-test-{name}-{file}-{}-{}",
        location.line(),
        std::process::id()
    ));
    std::fs::create_dir_all(&dir).expect("failed to create temp dir for test");
    dir
}
