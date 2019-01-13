use regex::Regex;

use chrono::FixedOffset;
use chrono::TimeZone;
use chrono::DateTime;

const TIME_REGEX: &str = "(?P<day>Wednesday|Thursday|Friday|Saturday|Sunday), (?P<startHrs>[0-9]{1,2}):(?P<startMins>[0-9]{2})(?P<startAmPm>AM|PM) - (?P<endHrs>[0-9]{1,2}):(?P<endMins>[0-9]{2})(?P<endAmPm>AM|PM)";

pub fn parse_time_slot(slot: &String) -> Option<(String, String)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(TIME_REGEX).unwrap();
    }
    let captures = RE.captures(slot)?;
    let start_is_pm = captures.name("startAmPm")?.as_str() == "PM";
    let end_is_pm = captures.name("endAmPm")?.as_str() == "PM";
    let start_hrs: u32 = match captures.name("startHrs")?.as_str().parse::<u32>() {
        Err(_) => return None,
        Ok(num) => {
            let num = if num == 12 { 0 } else { num };
            if start_is_pm {
                fix_day_overflow(num + 12)
            } else {
                num
            }
        }
    };
    let start_mins: u32 = match captures.name("startMins")?.as_str().parse() {
        Err(_) => return None,
        Ok(num) => num,
    };
    let end_hrs: u32 = match captures.name("endHrs")?.as_str().parse::<u32>() {
        Err(_) => return None,
        Ok(num) => {
            let num = if num == 12 { 0 } else { num };
            if end_is_pm {
                fix_day_overflow(num + 12)
            } else {
                num
            }
        }
    };
    let end_mins: u32 = match captures.name("endMins")?.as_str().parse() {
        Err(_) => return None,
        Ok(num) => num,
    };
    let day = captures.name("day")?.as_str();
    let start_offset = get_day_offset(&day);
    let end_offset = start_offset + if start_is_pm && !end_is_pm { 1 } else { 0 };
    let start_time = FixedOffset::east(9 * 3600).ymd(2018, 2, 22 + start_offset).and_hms(start_hrs, start_mins, 0);
    let end_time = FixedOffset::east(9 * 3600).ymd(2018, 2, 22 + end_offset).and_hms(end_hrs, end_mins, 0);
    Some((start_time.to_rfc2822(), end_time.to_rfc2822()))
}

fn fix_day_overflow(hrs: u32) -> u32 {
    if hrs >= 24 {
        hrs - 24
    } else {
        hrs
    }
}

fn get_day_offset(day: &str) -> u32 {
    match day {
        "Friday" => 1,
        "Saturday" => 2,
        "Sunday" => 3,
        _ => 0,
    }
}
