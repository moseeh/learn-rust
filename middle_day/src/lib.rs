extern crate chrono;
pub use chrono::prelude::*; // Bring commonly used chrono types and traits into scope
pub use chrono::Weekday as wd; // Alias Weekday enum to 'wd' for brevity

// Returns the weekday of the middle day of the given year, unless it's a leap year
pub fn middle_day(year: usize) -> Option<wd> {
    // Check if the year is a leap year; if so, return None
    if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) {
        return None;
    }

    // Construct the date for July 2nd (the middle of a non-leap year) and return its weekday
    Some(
        Utc.with_ymd_and_hms(year as i32, 7, 2, 0, 0, 0)
            .unwrap()
            .weekday(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leap_years() {
        // Ensure that leap years return None
        assert!(middle_day(1892).is_none(), "1892 was a leap year!");
        assert!(middle_day(1904).is_none(), "1904 was a leap year!");
        assert!(middle_day(2012).is_none(), "2012 was a leap year!");
    }

    #[test]
    fn weekdays() {
        // Check that the correct weekday is returned for various non-leap years
        assert_eq!(wd::Tue, middle_day(2019).unwrap());
        assert_eq!(wd::Wed, middle_day(1997).unwrap());
        assert_eq!(wd::Mon, middle_day(1663).unwrap());
        assert_eq!(wd::Wed, middle_day(1873).unwrap());
        assert_eq!(wd::Thu, middle_day(1953).unwrap());
        assert_eq!(wd::Wed, middle_day(1879).unwrap());
    }
}