use regex::Captures;
use regex::Match;
use regex::Regex;

use super::dateparse::DateParser;
use super::Event;

const EVENT_REGEX: &str = "^((?P<filled>\\[FILLED\\]) )?\
                           (?P<code>[A-Z][0-9]+): \
                           (\\[(?P<test_type>.*)\\] )?\
                           (?P<title>CANCELED BY (DESIGNER|GAMEMASTER)). +\
                           (?P<time>(Wednesday|Thursday|Friday|Saturday|Sunday), [0-9]{1,2}:[0-9]{2}(AM|PM) - [0-9]{1,2}:[0-9]{2}(AM|PM))\
                           (?P<misc>.*)$";

pub fn parse_event(input: &String, parser: &DateParser) -> Option<Event> {
	lazy_static! {
		static ref RE: Regex = Regex::new(EVENT_REGEX).unwrap();
	}
	match RE.captures(&input.as_str()) {
		None => None,
		Some(captures) => {
			let as_string = |c: &Captures, n: &str| {
				c.name(n)
					.map(|m: Match| m.as_str().to_string())
					.unwrap_or("".to_string())
			};
			let code = as_string(&captures, "code");
			let title = as_string(&captures, "title");
			let test_type = as_string(&captures, "test_type");
			let raw_time = as_string(&captures, "time");
			let filled = true;
			let (start_time, end_time) = parser.parse_time_slot(&raw_time)?;
			Some(Event {
				#[cfg(debug_assertions)]
				matched_by: "cancelled".to_string(),
				code,
				title,
				start_time,
				end_time,
				filled,
				test_type,
				..Default::default()
			})
		}
	}
}
