#![type_length_limit = "4194304"]

extern crate reqwest;
extern crate select;
#[macro_use]
extern crate lazy_static;

use select::document::Document;
use select::node::Node;
use std::fs::File;
use std::iter::Peekable;
// use select::selection::Selection;
use select::predicate as pred;

const NUM_RULES: usize = 5;
const SCHED_AFTER_RULES: usize = 5;

fn main() {
    scrape_dexposure()
}

fn scrape_dexposure() {
    // let resp = reqwest::get(url).unwrap();
    // assert!(resp.status().is_success());

    let input = File::open("./dreamation2018.html").unwrap();
    let doc = Document::from_read(input).unwrap();
    let mut bg3_nodes = doc.find(pred::And(pred::Name("td"), pred::Class("bg3")));
    let container = bg3_nodes.next().unwrap();
    // Bail if there's more than one possible container
    assert_eq!(None, bg3_nodes.next());

    assert_eq!(
        NUM_RULES,
        container
            .children()
            .filter(|n| n.is(pred::Name("hr")))
            .count()
    );
    let event_children = filter_event_nodes(&container);

    let mut iter = event_children
        .iter()
        // .filter(|n| match n.as_text() {
        //     None => true,
        //     Some(text) => text.split_whitespace().count() > 0,
        // })
        // .filter(|n| match n.as_comment() {
        //     None => true,
        //     Some(_) => false,
        // })
        .map(|n| *n)
        .peekable();
    let events = matchers::do_match(&mut iter);
}

fn filter_event_nodes<'a>(container: &'a Node) -> Vec<Node<'a>> {
    let mut seen_hrs = 0;
    container
        .children()
        .skip_while(|n| {
            if seen_hrs >= SCHED_AFTER_RULES {
                false
            } else if n.is(pred::Name("hr")) {
                seen_hrs += 1;
                true
            } else {
                true
            }
        })
        .collect()
}

pub mod matchers {
    use regex::Match;
    use regex::Regex;
    use regex::Captures;
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

    pub fn do_match<'a, I>(nodes: &mut Peekable<I>) -> Vec<Event>
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
        let mut iter_limit = 8;
        let mut event_block = String::new();
        for _ in 0..iter_limit {
            event_block.push_str(nodes.next().map(|n| n.text()).unwrap_or("".to_string()).replace('\n', "").as_str());
            let captures = match RE.captures(event_block.as_str()) {
                None => continue,
                Some(c) => c,
            };
            return Some(captures_to_event(&captures))
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
}
