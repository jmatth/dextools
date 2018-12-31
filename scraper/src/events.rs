use regex::Captures;
use regex::Match;
use regex::Regex;
use select::node::Node;
use select::predicate::*;
use std::iter::Peekable;

const BODY_REGEX: &str = "^(?P<code>[A-Z][0-9]+)(: )?((?P<system>.*); )?\"(?P<title>.*)\"( by (?P<authors>[a-zA-Z ]+))?(( written and)? presented by (?P<presenters>[a-zA-Z ]+))?. (?P<description>.*)";
const CODE_REGEX: &str = "^[A-Z][0-9]+$";

#[derive(Default, Debug)]
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
            None => return events,
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
                    println!("{:?}", event);
                    events.push(event);
                }
            };
            continue;
        }

        if node.is(Name("p")) {
            match extract_tag_event(node) {
                None => (),
                Some(event) => {
                    println!("{:?}", event);
                    events.push(event);
                }
            };
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
}

fn extract_flat_event<'a, I>(nodes: &mut I) -> Option<Event>
where
    I: Iterator<Item = Node<'a>>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(BODY_REGEX).unwrap();
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

fn extract_tag_event(node: &Node) -> Option<Event> {
    let body = node.text().replace('\n', "");
    match_full_event(body)
}

fn match_full_event(body: String) -> Option<Event> {
    lazy_static! {
        static ref RE: Regex = Regex::new(BODY_REGEX).unwrap();
    }
    RE.captures(&body.as_str()).map(|c| captures_to_event(&c))
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
        code: code,
        title: title,
        system: system,
        authors: authors,
        presenters: presenters,
        description: description,
        ..Default::default()
    }
}

fn parse_main_body<'a>(body: String) -> Event {
    lazy_static! {
        static ref RE: Regex = Regex::new(BODY_REGEX).unwrap();
    }
    let oneline_body = &body.replace('\n', "");
    let captures = match RE.captures(oneline_body) {
        Some(cap) => cap,
        None => {
            panic!("<{}>", oneline_body);
        }
    };
    let as_string = |m: Match| m.as_str().to_string();
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
        title: title,
        system: system,
        authors: authors,
        presenters: presenters,
        description: description,
        ..Default::default()
    }
}
