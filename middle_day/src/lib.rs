use chrono::{Datelike, NaiveDate, Weekday as wd};

/// Returns the weekday of the middle day of the given year.
/// Returns None if the year has an even number of days (e.g., non-leap years have 365 days).
pub fn middle_day(year: i32) -> Option<wd> {
    // Determine if it's a leap year (366 days) or regular year (365 days)
    let is_leap_year = NaiveDate::from_ymd_opt(year, 2, 29).is_some();
    
    // Regular years have 365 days (odd) and leap years have 366 days (even)
    if is_leap_year {
        // Leap years have an even number of days, so no middle day
        None
    } else {
        // For non-leap years with 365 days, the middle day is day 183
        // (183 days before + middle day + 181 days after = 365 days)
        let middle_day_of_year = 183;
        
        // Create a date for this day and get the weekday
        if let Some(date) = NaiveDate::from_yo_opt(year, middle_day_of_year) {
            Some(date.weekday())
        } else {
            None // This should never happen for valid years
        }
    }
}