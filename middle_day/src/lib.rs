use chrono::{NaiveDate, Datelike};

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {

    let date = NaiveDate::from_ymd_opt(year as i32, 1, 1)?;

    if date.leap_year() {
        return None;
    }

    let middle_day = 183;

    let middle_day_year = NaiveDate::from_yo_opt(year as i32, middle_day)?;

    Some(middle_day_year.weekday())
}