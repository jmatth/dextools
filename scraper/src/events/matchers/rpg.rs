use regex::Match;
use regex::Regex;

use super::Event;
use super::dateparse::DateParser;

const EVENT_REGEX: &str = "^(?P<code>[A-Z][0-9]+): \
                           ((?P<system>.*); )?\
                           \"(?P<title>.*?)\"\
                           ( by (?P<authors>[a-zA-Z ]+))?\
                           (( written and|;)? presented by (?P<presenters>([a-zA-Z ]+(, )?)+))?. \
                           (?P<description>.*) \
                           (?P<time>(Wednesday|Thursday|Friday|Saturday|Sunday), [0-9]{1,2}:[0-9]{2}(AM|PM) - [0-9]{1,2}:[0-9]{2}(AM|PM))";

pub fn parse_event(input: &String) -> Option<Event> {
    lazy_static! {
        static ref RE: Regex = Regex::new(EVENT_REGEX).unwrap();
        static ref parser: DateParser = DateParser::new(2018, 2, 22, 5 * 3600);
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
            let (start_time, end_time) = parser.parse_time_slot(&raw_time)?;
            Some(Event {
                code,
                title,
                system,
                authors,
                presenters,
                description,
                start_time,
                end_time,
                ..Default::default()
            })
        }
    }
}
