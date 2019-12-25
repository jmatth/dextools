use regex::Regex;

use chrono::TimeZone;
use chrono::Weekday;
use chrono_tz::Tz;

mod tests;

const TIME_REGEX: &str = "(?P<day>Wednesday|Thursday|Friday|Saturday|Sunday), (?P<startHrs>[0-9]{1,2}):(?P<startMins>[0-9]{2})(?P<startAmPm>AM|PM) - (?P<endHrs>[0-9]{1,2}):(?P<endMins>[0-9]{2})(?P<endAmPm>AM|PM)";

pub struct DateParser {
    y: i32,
    m: u32,
    d: u32,
    base_day: Weekday,
    tz: Tz,
}

impl DateParser {
    pub fn new(y: i32, m: u32, d: u32, base_day: Weekday, tz: Tz) -> DateParser {
        DateParser{ y, m, d, base_day, tz }
    }

    pub fn parse_time_slot(&self, slot: &String) -> Option<(String, String)> {
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
        let start_offset = self.get_day_offset(&day);
        let end_offset = start_offset + if start_is_pm && !end_is_pm { 1 } else { 0 };
        let start_time = self.tz.ymd(self.y, self.m, self.d + start_offset).and_hms(start_hrs, start_mins, 0);
        let end_time = self.tz.ymd(self.y, self.m, self.d + end_offset).and_hms(end_hrs, end_mins, 0);
        Some((start_time.to_rfc2822(), end_time.to_rfc2822()))

    }

    fn get_day_offset(&self, day: &str) -> u32 {
        let parsed_day: Weekday = day.parse().unwrap();
        ((parsed_day as i32 - self.base_day as i32) % 7 as i32) as u32
    }
}

fn fix_day_overflow(hrs: u32) -> u32 {
    if hrs >= 24 {
        hrs - 24
    } else {
        hrs
    }
}
