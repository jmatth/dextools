use regex::Captures;
use regex::Match;
use regex::Regex;
use select::node::Node;
use select::predicate::*;
use std::iter::Peekable;

mod matchers;

const FULL_EVENT_REGEX: &str = "\
                           ^(?P<code>[A-Z][0-9]+): \
                           ((?P<system>.*); )?\
                           \"(?P<title>.*)\"\
                           ( by (?P<authors>[a-zA-Z ]+))?\
                           (( written and|;)? presented by (?P<presenters>[a-zA-Z ]+))?. \
                           (?P<description>.*)";
const CODE_REGEX: &str = "^[A-Z][0-9]+$";

const MATCHERS: [fn(&String) -> Option<Event>; 2] = [
    matchers::rpg::parse_event,
    matchers::game::parse_event,
];

#[derive(Default, Debug, Serialize)]
pub struct Event {
    code: String,
    title: String,
    system: String,
    description: String,
    presenters: String,
    authors: String,
    time: String,
    tags: String,
}

pub fn parse_events<'a, I>(nodes: &mut Peekable<I>) -> Vec<Event>
where
    I: Iterator<Item = Node<'a>>,
{
    lazy_static! {
        static ref CODE_RE: Regex = Regex::new(CODE_REGEX).unwrap();
    }
    let mut events = Vec::new();
    loop {
        let node = match nodes.peek() {
            None => break,
            Some(node) => node,
        };
        match node.as_text() {
            Some(_) => {
                nodes.next();
                continue;
            }
            _ => (),
        }

        // Found the start of an event
        if node.is(Or(Name("font"), Name("b"))) && CODE_RE.is_match(node.text().as_str()) {
            match extract_flat_event(nodes) {
                None => (),
                Some(event) => {
                    events.push(event);
                }
            };
            continue;
        }

        if node.is(Name("p")) {
            let body = node.text().replace('\n', "").trim().to_string();
            match MATCHERS.into_iter().find_map(|f| f(&body)) {
                None => {
                    println!("<{}>", body);
                },
                Some(event) => {
                    events.push(event);
                }
            }
            nodes.next();
            continue;
        }

        println!(
            "Don't know how to handle node {}: {}",
            node.index(),
            node.html()
        );
        nodes.next();
    }
    println!("{}", events.len());
    events
}

fn extract_flat_event<'a, I>(nodes: &mut I) -> Option<Event>
where
    I: Iterator<Item = Node<'a>>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(FULL_EVENT_REGEX).unwrap();
    }
    let iter_limit = 8;
    let mut event_block = String::new();
    for _ in 0..iter_limit {
        event_block.push_str(
            nodes
                .next()
                .map(|n| n.text())
                .unwrap_or("".to_string())
                .replace('\n', "")
                .as_str(),
        );
        let captures = match RE.captures(event_block.as_str()) {
            None => continue,
            Some(c) => c,
        };
        return Some(captures_to_event(&captures));
    }
    None
}

fn captures_to_event(captures: &Captures) -> Event {
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
    Event {
        code,
        title,
        system,
        authors,
        presenters,
        description,
        ..Default::default()
    }
}
