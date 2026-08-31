use super::Entry;
use super::datetime::civil_from_days;
use super::error::Error;
use std::os::unix::fs::MetadataExt;

/// Formats `entry` the way `main` prints it: just the name normally, or
/// `<permissions> <size> <modified> <name>` when `entry.metadata` is `Some`
/// (i.e. [`Options::long`] was set).
///
/// # Errors
///
/// Returns [`Error::SystemTime`] if `entry.metadata` is `Some` and its
/// modification time predates the Unix epoch.
pub fn format_entry(entry: &Entry) -> Result<String, Error> {
    match &entry.metadata {
        Some(metadata) => Ok(format!(
            "{} {} {} {}",
            format_permissions(metadata),
            metadata.len(),
            format_modified(metadata)?,
            entry.name
        )),
        None => Ok(entry.name.clone()),
    }
}

/// Formats the 10-character permission string `ls -l` shows first, e.g.
/// `-rw-r--r--` for a regular file or `drwxr-xr-x` for a directory.
///
/// Owner/group *names* and the hard-link count are out of scope: turning a
/// uid into a username needs `libc`, which this project does not depend on.
pub fn format_permissions(metadata: &std::fs::Metadata) -> String {
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

/// Formats `metadata`'s modification time as `YYYY-MM-DD HH:MM`, in UTC.
///
/// # Errors
///
/// Returns [`Error::Io`] if the platform cannot report a modification time,
/// or [`Error::SystemTime`] if that time predates the Unix epoch.
pub fn format_modified(metadata: &std::fs::Metadata) -> Result<String, Error> {
    let total_seconds = metadata
        .modified()?
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    let days_since_epoch = total_seconds / 86_400;
    let seconds_of_day = total_seconds % 86_400;

    let (year, month, day) = civil_from_days(days_since_epoch);
    let hour = seconds_of_day / 3600;
    let minute = (seconds_of_day % 3600) / 60;

    Ok(format!(
        "{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::temp_dir;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::time::{Duration, UNIX_EPOCH};

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

    #[test]
    fn format_modified_formats_utc_date_and_time() {
        let dir = temp_dir("mtime");
        let path = dir.join("file.txt");
        let file = fs::File::create(&path).expect("failed to create file.txt");

        // 2026-08-30 00:00:00 UTC, cross-checked with `date -u -r 1788048000`.
        let mtime = UNIX_EPOCH + Duration::from_secs(1_788_048_000);
        file.set_times(fs::FileTimes::new().set_modified(mtime))
            .expect("failed to set mtime");
        drop(file);

        let metadata = fs::symlink_metadata(&path).expect("failed to read metadata");

        assert_eq!(
            format_modified(&metadata).expect("format_modified should succeed"),
            "2026-08-30 00:00"
        );

        fs::remove_dir_all(&dir).expect("failed to clean up temp dir");
    }
}
