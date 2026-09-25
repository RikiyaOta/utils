use crate::error::Error;
use std::time::SystemTime;

/// Formats `time` as an ISO 8601 date and time, in UTC, to the second:
/// `YYYY-MM-DDThh:mm:ss` (e.g. `2026-09-24T22:00:00`). This is the format of
/// a `.trashinfo`'s `DeletionDate`.
///
/// The Trash spec asks for local time, but the standard library cannot read
/// the system time zone, so this is UTC (see issue #41).
///
/// Because every field is zero-padded to a fixed width, comparing two of
/// these strings as text orders them by time; `--list` relies on that.
///
/// # Errors
///
/// Returns [`Error::SystemTime`] if `time` is before the Unix epoch.
#[expect(unused_variables, reason = "body is todo!(); delete once implemented")]
pub fn format_iso8601(time: SystemTime) -> Result<String, Error> {
    todo!("split seconds since the epoch into a calendar date and a time of day")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, UNIX_EPOCH};

    // Reference values were computed with Python's `calendar.timegm`.

    #[test]
    fn epoch() {
        assert_eq!(format_iso8601(UNIX_EPOCH).unwrap(), "1970-01-01T00:00:00");
    }

    #[test]
    fn time_of_day_is_zero_padded() {
        let time = UNIX_EPOCH + Duration::from_secs(1_790_287_200);

        assert_eq!(format_iso8601(time).unwrap(), "2026-09-24T22:00:00");
    }

    #[test]
    fn last_second_of_a_leap_day() {
        let time = UNIX_EPOCH + Duration::from_secs(1_709_251_199);

        assert_eq!(format_iso8601(time).unwrap(), "2024-02-29T23:59:59");
    }

    #[test]
    fn sub_second_part_is_dropped() {
        let time = UNIX_EPOCH + Duration::from_millis(1_999);

        assert_eq!(format_iso8601(time).unwrap(), "1970-01-01T00:00:01");
    }

    #[test]
    fn before_the_epoch_is_an_error() {
        let time = UNIX_EPOCH - Duration::from_secs(1);

        assert!(matches!(format_iso8601(time), Err(Error::SystemTime(_))));
    }
}
