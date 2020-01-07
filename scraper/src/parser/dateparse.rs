use std::convert::TryInto;

use regex::Regex;

use chrono::naive::NaiveDate;
use chrono::Datelike;
use chrono::Duration;
use chrono::LocalResult;
use chrono::TimeZone;
use chrono::Weekday;
use chrono_tz::Tz;
use lazy_static::lazy_static;

const TIME_REGEX: &str = "(?P<day>Monday|Tuesday|Wednesday|Thursday|Friday|Saturday|Sunday), (?P<startHrs>[0-9]{1,2}):(?P<startMins>[0-9]{2})(?P<startAmPm>AM|PM) - (?P<endHrs>[0-9]{1,2}):(?P<endMins>[0-9]{2})(?P<endAmPm>AM|PM)";

#[derive(Debug, Copy, Clone)]
pub struct DateParser {
	y: i32,
	m: u32,
	d: u32,
	tz: Tz,
	base_date: NaiveDate,
}

impl DateParser {
	pub fn new(y: i32, m: u32, d: u32, tz: Tz) -> DateParser {
		let base_date = NaiveDate::from_ymd(y, m, d);
		DateParser {
			y,
			m,
			d,
			tz,
			base_date,
		}
	}

	pub fn parse_time_slot(&self, slot: &String) -> Option<(String, String)> {
		lazy_static! {
			static ref RE: Regex = Regex::new(TIME_REGEX).unwrap();
		}

		// Parse the relevant values from the input string
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

		// Construct date objects from the extracted values
		let parsed_day: Weekday = day.parse().unwrap();
		let start_offset: i64 = self.get_day_offset(&parsed_day).try_into().unwrap();
		let end_offset = start_offset + if start_is_pm && !end_is_pm { 1 } else { 0 };
		let start_time_naive =
			(self.base_date + Duration::days(start_offset)).and_hms(start_hrs, start_mins, 0);
		let start_time_opt = self.tz.from_local_datetime(&start_time_naive);
		let end_time_naive =
			(self.base_date + Duration::days(end_offset)).and_hms(end_hrs, end_mins, 0);
		let end_time_opt = self.tz.from_local_datetime(&end_time_naive);

		// Resolve any times that may be ambiguous due to daylight savings
		let start_time = match start_time_opt {
			LocalResult::None => panic!("Got None for start_time"),
			LocalResult::Ambiguous(earlier, _) => earlier,
			LocalResult::Single(only) => only,
		};
		let end_time = match end_time_opt {
			LocalResult::None => panic!("Got None for end_time"),
			LocalResult::Ambiguous(_, later) => later,
			LocalResult::Single(only) => only,
		};

		Some((start_time.to_rfc2822(), end_time.to_rfc2822()))
	}

	fn get_day_offset(&self, day: &Weekday) -> u32 {
		let base_day_num = self.base_date.weekday().number_from_monday();
		let target_day_num = day.number_from_monday();
		if base_day_num <= target_day_num {
			target_day_num - base_day_num
		} else {
			Weekday::Sun.number_from_monday() - base_day_num + target_day_num
		}
	}
}

fn fix_day_overflow(hrs: u32) -> u32 {
	if hrs >= 24 {
		hrs - 24
	} else {
		hrs
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn get_parser() -> DateParser {
		DateParser::new(2018, 2, 22, chrono_tz::America::New_York)
	}

	fn test_parser(slot: &str, expected_start: &str, expected_end: &str) {
		let parsed = get_parser().parse_time_slot(&slot.to_string());
		assert!(parsed.is_some(), "Parser returned None");
		let (start, end) = parsed.unwrap();
		assert_eq!(expected_start, start);
		assert_eq!(expected_end, end);
	}

	#[test]
	fn test_am_am() {
		test_parser(
			&"Thursday, 8:00AM - 10:00AM",
			&"Thu, 22 Feb 2018 08:00:00 -0500",
			&"Thu, 22 Feb 2018 10:00:00 -0500",
		);
	}

	#[test]
	fn test_am_pm() {
		test_parser(
			&"Thursday, 10:00AM - 2:00PM",
			&"Thu, 22 Feb 2018 10:00:00 -0500",
			&"Thu, 22 Feb 2018 14:00:00 -0500",
		);
	}

	#[test]
	fn test_start_at_noon() {
		test_parser(
			&"Thursday, 12:00PM - 2:00PM",
			&"Thu, 22 Feb 2018 12:00:00 -0500",
			&"Thu, 22 Feb 2018 14:00:00 -0500",
		);
	}

	#[test]
	fn test_end_at_noon() {
		test_parser(
			&"Thursday, 10:00AM - 12:00PM",
			&"Thu, 22 Feb 2018 10:00:00 -0500",
			&"Thu, 22 Feb 2018 12:00:00 -0500",
		);
	}

	#[test]
	fn test_cross_day_boundary() {
		test_parser(
			&"Thursday, 10:00PM - 02:00AM",
			&"Thu, 22 Feb 2018 22:00:00 -0500",
			&"Fri, 23 Feb 2018 02:00:00 -0500",
		);
	}

	#[test]
	fn test_day_offset() {
		test_parser(
			&"Friday, 1:00PM - 5:00PM",
			&"Fri, 23 Feb 2018 13:00:00 -0500",
			&"Fri, 23 Feb 2018 17:00:00 -0500",
		);
	}

	#[test]
	fn test_start_at_midnight() {
		test_parser(
			&"Friday, 12:00AM - 4:00AM",
			&"Fri, 23 Feb 2018 00:00:00 -0500",
			&"Fri, 23 Feb 2018 04:00:00 -0500",
		);
	}

	#[test]
	fn test_end_at_midnight() {
		test_parser(
			&"Thursday, 10:00PM - 12:00AM",
			&"Thu, 22 Feb 2018 22:00:00 -0500",
			&"Fri, 23 Feb 2018 00:00:00 -0500",
		);
	}

	#[test]
	fn test_last_into_next_week() {
		test_parser(
			&"Tuesday, 10:00AM - 11:00AM",
			&"Tue, 27 Feb 2018 10:00:00 -0500",
			&"Tue, 27 Feb 2018 11:00:00 -0500",
		);
	}

	#[test]
	fn test_daylight_savings_cutover() {
		let parser = DateParser::new(2019, 11, 1, chrono_tz::America::New_York);
		{
			let parsed = parser.parse_time_slot(&"Sunday, 1:00AM - 3:00AM".to_string());
			assert!(parsed.is_some(), "Parser returned None");
			let (start, end) = parsed.unwrap();
			assert_eq!("Sun, 03 Nov 2019 01:00:00 -0400", start);
			assert_eq!("Sun, 03 Nov 2019 03:00:00 -0500", end);
		}
		{
			let parsed = parser.parse_time_slot(&"Sunday, 12:00AM - 1:00AM".to_string());
			assert!(parsed.is_some(), "Parser returned None");
			let (start, end) = parsed.unwrap();
			assert_eq!("Sun, 03 Nov 2019 00:00:00 -0400", start);
			assert_eq!("Sun, 03 Nov 2019 01:00:00 -0500", end);
		}
		{
			let parsed = parser.parse_time_slot(&"Sunday, 1:00AM - 1:00AM".to_string());
			assert!(parsed.is_some(), "Parser returned None");
			let (start, end) = parsed.unwrap();
			assert_eq!("Sun, 03 Nov 2019 01:00:00 -0400", start);
			assert_eq!("Sun, 03 Nov 2019 01:00:00 -0500", end);
		}
	}
}
