extern crate chrono;
use chrono::{Local, Date, Datelike, Duration, TimeZone};

pub fn str_next_monday() -> String {
    let now = Local::now();
    let date_next_monday = next_monday(now.date());
    format!("{}", date_next_monday.format("%d%B%y"))
}

// Returns date if date is monday, otherwise return next monday of date
fn next_monday<T>(date: Date<T>) -> Date<T>
    where T: TimeZone{
    let d = date.weekday().num_days_from_monday();
    match d {
        0 => date,
        d => date.checked_add_signed(Duration::days((7-d) as i64)).unwrap()
    }
}
