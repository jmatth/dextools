extern crate reqwest;
extern crate select;
extern crate serde_json;
extern crate chrono;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

use select::document::Document;
use select::node::Node;
use std::fs::File;
use select::predicate as pred;

const NUM_RULES: usize = 5;
const SCHED_AFTER_RULES: usize = 5;

mod events;

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
        .map(|n| *n)
        .peekable();
    let events = events::parse_events(&mut iter);
    let output = File::create("./con.json").unwrap();
    serde_json::to_writer_pretty(output, &events);
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
