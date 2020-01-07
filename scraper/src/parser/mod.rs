use indexmap::IndexMap;

use lazy_static::lazy_static;
use regex::Regex;
use select::node::Node;
use select::predicate as pred;
use select::predicate::*;
use serde::Serialize;

pub mod dateparse;
pub mod matcher;

use dateparse::DateParser;

const CODE_REGEX: &str = "^[A-Z][0-9]+$";

#[derive(Default, Debug, Serialize)]
pub struct Event {
	#[serde(skip_serializing_if = "String::is_empty")]
	matched_by: String,
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
	test_type: String,
	hi_test: bool,
	round: String,
	materials: String,
	experience: String,
	mood: String,
	age: String,
	related: String,
	#[serde(skip_serializing_if = "String::is_empty")]
	misc: String,
	#[serde(skip_serializing_if = "String::is_empty")]
	raw: String,
}

const RPG_REGEX: &str = "^\
                         (?P<code>[A-Z][0-9]+): \
                         ((?P<system>.*); )?\
                         \"(?P<title>.*?)\"\
                         ( by (?P<authors>.*?))?\
                         (( written and|;)? presented by (?P<presenters>(.*?(, )?)+))?\\. \
                         (?P<description>.*) \
                         (?P<time>(Wednesday|Thursday|Friday|Saturday|Sunday), [0-9]{1,2}:[0-9]{2}(AM|PM) - [0-9]{1,2}:[0-9]{2}(AM|PM)); \
                         (?P<round>[^;\\.]+)\
                         (\
                          ; \
                          (?P<materials>[^\\.]+)\\. \
                          (?P<experience>[^;]+); \
                          (?P<mood>[^,]+), \
                          (?P<age>[^\\.]+)\\. ?\
                          |\
                          \\.( +)?\
                         )\
                         ((?P<related>(Next Round|Previous Round\\(s\\)|See Also): [^\\.]+)\\. ?)?\
                         (?P<misc>.*)?\
						 $";

const GAME_REGEX: &str = "^\
                          (?P<code>[A-Z][0-9]+): \
                          (?P<title>.*?)\\. \
                          (?P<description>.*) \
                          (?P<time>(Wednesday|Thursday|Friday|Saturday|Sunday), [0-9]{1,2}:[0-9]{2}(AM|PM) - [0-9]{1,2}:[0-9]{2}(AM|PM)); \
                          (?P<round>[^;\\.]+)\
                          (\
                           ; \
                           (?P<materials>[^\\.]+)\\. \
                           (?P<experience>[^;]+); \
                           (?P<mood>[^,]+), \
                           (?P<age>[^\\.]+)\\. ?\
                           |\
                           \\.( +)?\
                          )\
                          ((?P<related>(Next Round|Previous Round\\(s\\)|See Also): [^\\.]+)\\. ?)?\
                          (?P<misc>.*)\
						  $";

const CANCELLED_REGEX: &str = "^\
                               ((?P<filled>\\[FILLED\\]) )?\
                               (?P<code>[A-Z][0-9]+): \
                               (\\[(?P<test_type>.*)\\] )?\
                               (?P<title>CANCELED BY (DESIGNER|GAMEMASTER)). +\
                               (?P<time>(Wednesday|Thursday|Friday|Saturday|Sunday), [0-9]{1,2}:[0-9]{2}(AM|PM) - [0-9]{1,2}:[0-9]{2}(AM|PM))\
                               (?P<misc>.*)\
							   $";

const METATOPIA_REGEX: &str = "^\
                               ((?P<filled>\\[FILLED\\]) )?\
                               (?P<code>[A-Z][0-9]+): \
                               (\\[(?P<test_type>.*)\\] )?\
                               \"(?P<title>.*?)\"\
                               ( by (?P<authors>.*?))?\
                               (( written and|;)? presented by (?P<presenters>(.*?(, )?)+))?\\. \
                               (?P<description>.*) \
                               (?P<time>(Wednesday|Thursday|Friday|Saturday|Sunday), [0-9]{1,2}:[0-9]{2}(AM|PM) - [0-9]{1,2}:[0-9]{2}(AM|PM))\
                               (?P<misc>.*)\
							   $";

pub fn parse_events<'a, I>(nodes: &mut I, date_parser: DateParser) -> Vec<Event>
where
	I: Iterator<Item = Node<'a>>,
{
	lazy_static! {
		static ref CODE_RE: Regex = Regex::new(CODE_REGEX).unwrap();
	}
	let matchers = [
		matcher::Matcher::new("rpg", RPG_REGEX, date_parser),
		matcher::Matcher::new("game", GAME_REGEX, date_parser),
		matcher::Matcher::new("cancelled", CANCELLED_REGEX, date_parser),
		matcher::Matcher::new("metatopia", METATOPIA_REGEX, date_parser),
	];
	let mut events_map = IndexMap::with_capacity(1024);
	let mut next = nodes.next();
	while next.is_some() {
		let node = next.unwrap();
		match node.as_text() {
			Some(_) => {
				next = nodes.next();
				continue;
			}
			_ => (),
		}

		// Found the start of an event
		let body = if node.is(Or(Name("font"), Name("b"))) && CODE_RE.is_match(node.text().as_str())
		{
			// Event isn't wrapped in a <p> tag, usually just the first event is like this.
			// Use of tmp_next is to avoid variable shadowing shenanigans.
			let (result, tmp_next) = extract_flat_event(&node, nodes);
			next = tmp_next;
			match result {
				Some(body) => body,
				None => {
					continue;
				}
			}
		} else if node.is(Name("p")) {
			// Most events are wrapped in a <p> tag, get its contents as text.
			next = nodes.next();
			node.text().replace('\n', "").trim().to_string()
		} else {
			// Found something we don't know how to handle, ignore and keep going.
			// println!(
			//     "Don't know how to handle node {}: {}",
			//     node.index(),
			//     node.html()
			// );
			next = nodes.next();
			continue;
		};

		match matchers.iter().find_map(|m| m.parse_event(&body)) {
			None => {
				// if body.len() > 0 {
				//     println!("<{}>", body);
				// }
			}
			Some(event) => {
				events_map.insert(event.code.clone(), event);
			}
		}
	}

	// TODO: post-processing stage to link related events

	let events_vec: Vec<Event> = events_map.drain(..).map(|(_, v)| v).collect();
	println!("Successfully parsed {} events.", events_vec.len());
	if events_vec.len() > 0 {
		let mut e_iter = events_vec.iter();
		let mut prev_event: &Event = e_iter.next().unwrap();
		while let Some(curr_event) = e_iter.next() {
			let curr_num: usize = match curr_event.code[1..].parse() {
				Err(_) => {
					println!("WARNING: invalid event code: {}", curr_event.code);
					prev_event = curr_event;
					continue;
				}
				Ok(c) => c,
			};
			let prev_num: usize = match prev_event.code[1..].parse() {
				Err(_) => continue,
				Ok(c) => c,
			};
			if prev_num != curr_num - 1 {
				println!(
					"WARNING: possible missing event: {} -> {}",
					prev_event.code, curr_event.code
				);
			}
			prev_event = curr_event;
		}
	}

	events_vec
}

fn extract_flat_event<'a, I>(initial: &Node, nodes: &mut I) -> (Option<String>, Option<Node<'a>>)
where
	I: Iterator<Item = Node<'a>>,
{
	let iter_limit = 8;
	let mut event_block = String::new();
	event_block.push_str(initial.text().replace('\n', "").as_str());
	let mut next = None;
	for _ in 0..iter_limit {
		next = nodes.next();
		if next.map_or(false, |n| n.is(pred::Name("p"))) {
			return (
				if event_block.is_empty() {
					None
				} else {
					Some(event_block)
				},
				next,
			);
		}
		event_block.push_str(
			next.map(|n| n.text())
				.unwrap_or("".to_string())
				.replace('\n', "")
				.as_str(),
		);
	}
	(None, next)
}
