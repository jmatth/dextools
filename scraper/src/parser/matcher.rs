use regex::Match;
use regex::Regex;

use super::dateparse::DateParser;
use super::Event;

#[cfg(feature = "debug-json")]
use super::EventDebug;

const NEXT_ROUND_REGEX: &str = "Next (Session|Round): ";
const PREV_ROUND_REGEX: &str = "Previous (Session|Round)\\(s\\): ";
const SEE_ALSO_PREFIX: &str = "See Also: ";

pub struct Matcher<'a> {
	#[allow(dead_code)]
	id: &'a str,
	regex: Regex,
	date_parser: DateParser,
	// TODO: move these into lazy_static
	next_round_re: Regex,
	prev_round_re: Regex,
}

impl Matcher<'_> {
	pub fn new<'a>(id: &'a str, regex: &'a str, parser: DateParser) -> Matcher<'a> {
		#[cfg(feature = "dump-regexes")]
		println!("matcher: {}\nregex:{}\n", id, regex);

		Matcher {
			id: id,
			regex: Regex::new(regex).unwrap(),
			date_parser: parser,
			next_round_re: Regex::new(NEXT_ROUND_REGEX).unwrap(),
			prev_round_re: Regex::new(PREV_ROUND_REGEX).unwrap(),
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
				let next_rounds = if related.starts_with(&self.next_round_re) {
					related
						.replace(&self.next_round_re, "")
						.split(",")
						.map(|s| s.trim().to_string())
						.collect::<Vec<String>>()
				} else {
					Vec::new()
				};
				let previous_rounds = if related.starts_with(&self.prev_round_re) {
					related
						.replace(&self.prev_round_re, "")
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
				let remaining = captures
					.name("remaining")
					.map(as_string)
					.unwrap_or("".to_string());
				let filled = remaining.to_lowercase().contains("this event has been filled!");
				let advancement = remaining
					.to_lowercase()
					.contains("this is an advancement session and cannot be selected.");
				let (start_time, end_time) = self.date_parser.parse_time_slot(&raw_time)?;
				Some(Event {
					code,
					title,
					system,
					authors,
					presenters,
					description,
					start_time,
					end_time,
					filled,
					advancement,
					round,
					materials,
					experience,
					mood,
					age,
					next_rounds,
					previous_rounds,
					see_also,
					#[cfg(feature = "debug-json")]
					debug: EventDebug {
						matched_by: self.id.to_string(),
						remaining,
						raw: input.to_string(),
					},
					..Default::default()
				})
			}
		}
	}
}
