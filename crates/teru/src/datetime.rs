/// Converts a day count since the Unix epoch (1970-01-01) into a
/// `(year, month, day)` civil-calendar date (proleptic Gregorian).
///
/// Ported from Howard Hinnant's public-domain `civil_from_days` algorithm
/// (<http://howardhinnant.github.io/date_algorithms.html>), simplified to
/// assume `days_since_epoch >= 0`. Callers only ever reach this after
/// `SystemTime::duration_since(UNIX_EPOCH)` has already rejected dates
/// before 1970, so the original algorithm's negative-`z` branch never
/// applies here and was dropped.
///
/// The core trick: shift the epoch from 1970-01-01 to 0000-03-01, so every
/// "civil year" ends in February -- the one month whose length varies. That
/// turns "is this a leap year" from a special case into an arithmetic
/// property of a 400-year cycle (146097 days, exactly), which the rest of
/// the formula exploits directly.
pub fn civil_from_days(days_since_epoch: u64) -> (u64, u32, u32) {
    let z = days_since_epoch + 719_468;

    let era = z / 146_097; // 146097 days = 400 civil years, exactly.
    let day_of_era = z % 146_097; // [0, 146096]

    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365; // [0, 399]
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100); // [0, 365]

    let month_prime = (5 * day_of_year + 2) / 153; // [0, 11], with March = 0
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1; // [1, 31]
    let month = if month_prime < 10 {
        month_prime + 3
    } else {
        month_prime - 9
    }; // [1, 12]
    let year = year_of_era + era * 400 + if month <= 2 { 1 } else { 0 };

    (year, month as u32, day as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Reference values below were cross-checked against the real `date`
    // command, e.g. `date -j -u -f "%Y-%m-%d %H:%M:%S" "2024-02-29 00:00:00" "+%s"`.

    #[test]
    fn civil_from_days_epoch() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
    }

    #[test]
    fn civil_from_days_first_of_february() {
        assert_eq!(civil_from_days(31), (1970, 2, 1));
    }

    #[test]
    fn civil_from_days_leap_day_2024() {
        // 2024 is divisible by 4 (and not by 100), so it is a leap year.
        assert_eq!(civil_from_days(19_782), (2024, 2, 29));
    }

    #[test]
    fn civil_from_days_leap_day_2000() {
        // 2000 is divisible by 400, so it IS a leap year (unlike 1900/2100).
        assert_eq!(civil_from_days(11_016), (2000, 2, 29));
    }

    #[test]
    fn civil_from_days_not_a_leap_day_2100() {
        // 2100 is divisible by 100 but not by 400, so it is NOT a leap year.
        assert_eq!(civil_from_days(47_540), (2100, 2, 28));
    }
}
