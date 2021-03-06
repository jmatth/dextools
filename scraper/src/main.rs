#![feature(try_trait)]

use std::fs::File;
use std::io::Read;
use std::io::Write;

use chrono::naive::NaiveDate;
use chrono::Datelike;
use clap::{crate_authors, crate_name, crate_version, load_yaml, App};
use lazy_static::lazy_static;
use regex::Regex;
use select::document::Document;
use select::node::Node;
use select::predicate as pred;
use serde_json::map::Map;
use serde_json::{json, Value};

mod client;
mod error;
mod parser;

use error::Error;
use parser::dateparse::DateParser;

const SCHEDULE_KEY: &str = "schedule";
const CON_TIMEZONE: chrono_tz::Tz = chrono_tz::America::New_York;
const DATE_REGEX: &str = "^(?P<weekday>Monday|Tuesday|Wednesday|Thursday|Friday|Saturday|Sunday), \
						  (?P<month>January|February|March|April|May|June|July|August|September|October|November|December) \
						  (?P<day>[0-9]{1,2}) - \
						  (Monday|Tuesday|Wednesday|Thursday|Friday|Saturday|Sunday), \
						  (January|February|March|April|May|June|July|August|September|October|November|December) \
						  [0-9]{1,2}, \
						  (?P<year>[0-9]{4})";

fn main() -> Result<(), Error> {
	let app_yaml = load_yaml!("cli.yml");
	let matches = App::from_yaml(app_yaml)
		.name(crate_name!())
		.version(crate_version!())
		.author(crate_authors!())
		.get_matches();
	let input = matches
		.value_of("input")
		.ok_or("Missing required flag 'input'")?;
	let output = matches
		.value_of("output")
		.ok_or("Missing required flag 'output'")?;
	let config_template_path = matches.value_of("template_config").unwrap_or("");
	let cache = matches.value_of("cache").unwrap_or("/dev/null");
	let client = client::CachingClient::new(cache)?;
	if input.starts_with("http://") || input.starts_with("https://") {
		let scrape_result = client.scrape_site(input)?;
		match scrape_result {
			Some(mut http_reader) => {
				parse_events(&mut http_reader, &output.to_string(), config_template_path)
			}
			None => {
				println!("Upstream not modified");
				return Ok(());
			}
		}
	} else {
		let mut file_input = File::open(&input)?;
		parse_events(&mut file_input, &output.to_string(), config_template_path)
	}
}

fn parse_events<R: Read>(
	input: &mut R,
	output_path: &String,
	template_config_path: &str,
) -> Result<(), Error> {
	let doc = Document::from_read(input).unwrap();
	let mut bg3_nodes = doc.find(pred::And(pred::Name("td"), pred::Class("bg3")));
	let container = bg3_nodes.next().unwrap();
	// Bail if there's more than one possible container
	assert_eq!(None, bg3_nodes.next());

	let (start_year, start_month, start_day) = get_start_date(&container)?;
	let date_parser = DateParser::new(start_year, start_month, start_day, CON_TIMEZONE);

	let event_children = filter_event_nodes(&container);

	let mut iter = event_children.iter().map(|n| *n);
	let schedule = parser::parse_events(&mut iter, date_parser);
	let mut config_val: Value = if template_config_path.len() < 1 {
		serde_json::from_str("{}")?
	} else {
		let template_config_file = File::open(template_config_path)?;
		serde_json::from_reader(template_config_file)?
	};
	let config: &mut Map<String, Value> = config_val.as_object_mut()?;
	config.insert(SCHEDULE_KEY.to_string(), json!(schedule));
	let mut output = File::create(output_path)?;
	serde_json::to_writer_pretty(&output, &config)?;
	output.write(b"\n").map(|_| ()).map_err(|e| Error::from(e))
}

fn get_start_date<'a>(container: &'a Node) -> Result<(i32, u32, u32), Error> {
	lazy_static! {
		static ref DATE_RE: Regex = Regex::new(DATE_REGEX).unwrap();
	}

	// The text describing the date range of the con is always within a center tag, but whether
	// it's an h2 or h3 nested within that is inconsistent.
	let date_node_candidates = container.find(pred::Child(
		pred::Child(
			pred::Name("center"),
			pred::Or(pred::Name("h2"), pred::Name("h3")),
		),
		pred::Any,
	));

	for date_node in date_node_candidates {
		let date_string = date_node.text();
		// Thursday, November 1 - Sunday, November 4, 2018
		let captures = match DATE_RE.captures(&date_string.as_str()) {
			Some(captures) => captures,
			None => continue,
		};
		let weekday_str = captures.name("weekday")?.as_str();
		let month_str = captures.name("month")?.as_str();
		let day_str = captures.name("day")?.as_str();
		let year_str = captures.name("year")?.as_str();
		let intermediate_string = format!("{} {} {} {}", year_str, month_str, day_str, weekday_str);
		let start_date = NaiveDate::parse_from_str(&intermediate_string, "%Y %B %e %A")?;
		return Ok((start_date.year(), start_date.month(), start_date.day()));
	}

	Err(Error::from("No matches found to extract start date"))
}

fn filter_event_nodes<'a>(container: &'a Node) -> Vec<Node<'a>> {
	container
		.children()
		.skip_while(|n| n.is(pred::Name("p")))
		.collect()
}
