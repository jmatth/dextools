use regex::Regex;
use select::node::Node;
use select::predicate::*;
use std::iter::Peekable;
use select::predicate as pred;

pub mod matchers;

use crate::events::matchers::dateparse::DateParser;

const CODE_REGEX: &str = "^[A-Z][0-9]+$";

const MATCHERS: [fn(&String, &DateParser) -> Option<Event>; 4] = [
    matchers::metatopia::parse_event,
    matchers::rpg::parse_event,
    matchers::game::parse_event,
    matchers::canceled::parse_event,
];

#[derive(Default, Debug, Serialize)]
pub struct Event {
    code: String,
    title: String,
    system: String,
    description: String,
    presenters: String,
    authors: String,
    start_time: String,
    end_time: String,
    filled: bool,
    tags: String,
    mtype: String,
}

pub fn parse_events<'a, I>(nodes: &mut Peekable<I>, date_parser: &DateParser) -> Vec<Event>
where
    I: Iterator<Item = Node<'a>>,
{
    lazy_static! {
        static ref CODE_RE: Regex = Regex::new(CODE_REGEX).unwrap();
    }
    let mut events = Vec::new();
    loop {
        // Hack because the borrow checker hates me, come back
        // and figure out how to remove this eventually.
        let mut should_step = false;
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
        let body =
        if node.is(Or(Name("font"), Name("b"))) && CODE_RE.is_match(node.text().as_str()) {
            // Event isn't wrapped in a <p> tag, usually just the first event is like this.
            match extract_flat_event(nodes) {
                Some(body) => body,
                None => {
                    nodes.next();
                    continue;
                }
            }
        } else if node.is(Name("p")) {
            // Most events are wrapped in a <p> tag, get its contents as text.
            should_step = true;
            node.text().replace('\n', "").trim().to_string()
        } else {
            // Found something we don't know how to handle, ignore and keep going.
            // println!(
            //     "Don't know how to handle node {}: {}",
            //     node.index(),
            //     node.html()
            // );
            nodes.next();
            continue
        };

        match MATCHERS.into_iter().find_map(|f| f(&body, date_parser)) {
            None => {
                if body.len() > 0 {
                    println!("<{}>", body);
                }
            },
            Some(event) => {
                events.push(event);
            }
        }
        if should_step {
            nodes.next();
        }
    }
    println!("Successfully parsed {} events.", events.len());
    events
}

fn extract_flat_event<'a, I>(nodes: &mut Peekable<I>) -> Option<String>
where
    I: Iterator<Item = Node<'a>>,
{
    let iter_limit = 8;
    let mut event_block = String::new();
    for _ in 0..iter_limit {
        if nodes.peek().map_or(false, |n| n.is(pred::Name("p"))) {
            return if event_block.is_empty() { None } else { Some(event_block) };
        }
        event_block.push_str(
            nodes.next()
                .map(|n| n.text())
                .unwrap_or("".to_string())
                .replace('\n', "")
                .as_str(),
        );
    }
    None
}
