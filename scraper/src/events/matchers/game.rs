use regex::Regex;
use regex::Match;

use super::Event;
use super::dateparse;

const EVENT_REGEX: &str = "^(?P<code>[A-Z][0-9]+): \
                           (?P<system>[^.]+)\\. \
                           (?P<description>.*) \
                           (?P<time>(Wednesday|Thursday|Friday|Saturday|Sunday), [0-9]{1,2}:[0-9]{2}(AM|PM) - [0-9]{1,2}:[0-9]{2}(AM|PM))";

pub fn parse_event(input: &String) -> Option<Event> {
    lazy_static! {
        static ref RE: Regex = Regex::new(EVENT_REGEX).unwrap();
    }
    match RE.captures(&input.as_str()) {
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
            let description = captures
                .name("description")
                .map(as_string)
                .unwrap_or("".to_string());
            let raw_time = captures
                .name("time")
                .map(as_string)
                .unwrap_or("".to_string());
            let (start_time, end_time) = dateparse::parse_time_slot(&raw_time)?;
            Some(Event {
                code,
                system,
                description,
                start_time,
                end_time,
                ..Default::default()
            })
        }
    }
}
