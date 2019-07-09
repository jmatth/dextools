use regex::Match;
use regex::Regex;
use regex::Captures;

use super::Event;
use super::dateparse::DateParser;

const EVENT_REGEX: &str = "^((?P<filled>\\[FILLED\\]) )?\
                           (?P<code>[A-Z][0-9]+): \
                           (\\[(?P<mtype>.*)\\] )?\
                           \"(?P<title>.*?)\"\
                           ( by (?P<authors>.*?))?\
                           (( written and|;)? presented by (?P<presenters>(.*?(, )?)+))?\\. \
                           (?P<description>.*) \
                           (?P<time>(Wednesday|Thursday|Friday|Saturday|Sunday), [0-9]{1,2}:[0-9]{2}(AM|PM) - [0-9]{1,2}:[0-9]{2}(AM|PM))\
                           (?P<misc>.*)$";

pub fn parse_event(input: &String, parser: &DateParser) -> Option<Event> {
    lazy_static! {
        static ref RE: Regex = Regex::new(EVENT_REGEX).unwrap();
    }
    match RE.captures(&input.as_str()) {
        None => None,
        Some(captures) => {
            let as_string = |c: &Captures, n: &str| c.name(n).map(|m: Match| m.as_str().to_string()).unwrap_or("".to_string());
            let filled = as_string(&captures, "filled").len() > 0;
            let code = as_string(&captures, "code");
            let system = as_string(&captures, "system");
            let title = as_string(&captures, "title");
            let authors = as_string(&captures, "authors");
            let presenters = as_string(&captures, "presenters");
            let description = as_string(&captures,"description");
            let mtype = as_string(&captures,"mtype");
            let raw_time = as_string(&captures,"time");
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
                filled,
                mtype,
                ..Default::default()
            })
        }
    }
}
