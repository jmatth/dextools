use regex::Match;
use regex::Regex;

use super::dateparse::DateParser;
use super::Event;

const NEXT_ROUND_PREFIX: &str = "Next Round: ";
const PREV_ROUND_PREFIX: &str = "Previous Round(s): ";
const SEE_ALSO_PREFIX: &str = "See Also: ";

pub struct Matcher<'a> {
	id: &'a str,
	regex: Regex,
	date_parser: DateParser,
}

impl Matcher<'_> {
	pub fn new<'a>(id: &'a str, regex_string: &'a str, parser: DateParser) -> Matcher<'a> {
		Matcher {
			id: id,
			regex: Regex::new(regex_string).unwrap(),
			date_parser: parser,
		}
	}

	pub fn parse_event(&self, input: &str) -> Option<Event> {
		match self.regex.captures(input) {
			None => None,
			Some(captures) => {
				let as_string = |m: Match| m.as_str().to_string();
				let code = captures
					.name("code")
					.map(as_string)
					.unwrap_or("".to_string());
				let system = captures
					.name("system")
					.map(as_string)
					.unwrap_or("".to_string());
				let title = captures
					.name("title")
					.map(as_string)
					.unwrap_or("".to_string());
				let authors = captures
					.name("authors")
					.map(as_string)
					.unwrap_or("".to_string());
				let presenters = captures
					.name("presenters")
					.map(as_string)
					.unwrap_or("".to_string());
				let description = captures
					.name("description")
					.map(as_string)
					.unwrap_or("".to_string());
				let raw_time = captures
					.name("time")
					.map(as_string)
					.unwrap_or("".to_string());
				let round = captures
					.name("round")
					.map(as_string)
					.unwrap_or("".to_string());
				let materials = captures
					.name("materials")
					.map(as_string)
					.unwrap_or("".to_string());
				let experience = captures
					.name("experience")
					.map(as_string)
					.unwrap_or("".to_string());
				let mood = captures
					.name("mood")
					.map(as_string)
					.unwrap_or("".to_string());
				let age = captures
					.name("age")
					.map(as_string)
					.unwrap_or("".to_string());
				let related = captures
					.name("related")
					.map(as_string)
					.unwrap_or("".to_string());
				let next_rounds = if related.starts_with(NEXT_ROUND_PREFIX) {
					related
						.replace(NEXT_ROUND_PREFIX, "")
						.split(",")
						.map(|s| s.trim().to_string())
						.collect::<Vec<String>>()
				} else {
					Vec::new()
				};
				let previous_rounds = if related.starts_with(PREV_ROUND_PREFIX) {
					related
						.replace(PREV_ROUND_PREFIX, "")
						.split(",")
						.map(|s| s.trim().to_string())
						.collect::<Vec<String>>()
				} else {
					Vec::new()
				};
				let see_also = if related.starts_with(SEE_ALSO_PREFIX) {
					related
						.replace(SEE_ALSO_PREFIX, "")
						.split(",")
						.map(|s| s.trim().to_string())
						.collect::<Vec<String>>()
				} else {
					Vec::new()
				};
				let misc = captures
					.name("misc")
					.map(as_string)
					.unwrap_or("".to_string());
				let filled = misc.to_lowercase().contains("this event has been filled!");
				let (start_time, end_time) = self.date_parser.parse_time_slot(&raw_time)?;
				Some(Event {
					#[cfg(debug_assertions)]
					matched_by: self.id.to_string(),
					code,
					title,
					system,
					authors,
					presenters,
					description,
					start_time,
					end_time,
					filled,
					round,
					materials,
					experience,
					mood,
					age,
					next_rounds,
					previous_rounds,
					see_also,
					#[cfg(debug_assertions)]
					misc,
					..Default::default()
				})
			}
		}
	}
}
