use chrono::{Datelike, Duration, NaiveDate, Weekday};

use crate::models::DueState;

pub fn format_date(d: NaiveDate, today: NaiveDate) -> String {
    let diff = (d - today).num_days();
    if diff == 0 {
        "Today".to_string()
    } else if diff == 1 {
        "Tomorrow".to_string()
    } else if diff == -1 {
        "Yesterday".to_string()
    } else if diff < 0 {
        format!("{}d overdue", diff.unsigned_abs())
    } else if diff < 7 {
        weekday_short(d.weekday()).to_string()
    } else {
        format!("{} {}", month_short(d.month()), d.day())
    }
}

pub fn due_state(d: NaiveDate, today: NaiveDate) -> DueState {
    let diff = (d - today).num_days();
    if diff < 0 {
        DueState::Overdue
    } else if diff == 0 {
        DueState::Today
    } else {
        DueState::Scheduled
    }
}

pub fn next_saturday(today: NaiveDate) -> NaiveDate {
    let dow = today.weekday().num_days_from_sunday();
    // Saturday is num_days_from_sunday() == 6.
    let days = if dow == 6 { 7 } else { (6 - dow as i64).rem_euclid(7) };
    let days = if days == 0 { 7 } else { days };
    today + Duration::days(days)
}

pub fn weekday_short(w: Weekday) -> &'static str {
    match w {
        Weekday::Mon => "Mon",
        Weekday::Tue => "Tue",
        Weekday::Wed => "Wed",
        Weekday::Thu => "Thu",
        Weekday::Fri => "Fri",
        Weekday::Sat => "Sat",
        Weekday::Sun => "Sun",
    }
}

pub fn weekday_long(w: Weekday) -> &'static str {
    match w {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    }
}

pub fn month_short(m: u32) -> &'static str {
    match m {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "",
    }
}

pub fn month_long(m: u32) -> &'static str {
    match m {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "",
    }
}

pub fn long_header_label(today: NaiveDate) -> String {
    format!(
        "{}, {} {}",
        weekday_long(today.weekday()),
        month_long(today.month()),
        today.day()
    )
}
