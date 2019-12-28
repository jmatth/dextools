#![feature(try_trait)]

extern crate chrono;
extern crate clap;
extern crate reqwest;
extern crate select;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

use clap::{App, Arg};
use select::document::Document;
use select::node::Node;
use select::predicate as pred;
use serde_json::map::Map;
use serde_json::{json, Value};
use std::fs::File;
use std::io::Read;

mod client;
mod error;
mod events;

use error::Error;

use crate::events::matchers::dateparse::DateParser;

const SCHEDULE_KEY: &str = "schedule";
const CON_TIMEZONE: chrono_tz::Tz = chrono_tz::America::New_York;

fn main() -> Result<(), Error> {
	let matches = App::new("Dextools scraper")
		.version("1.0")
		.author("Joshua Matthews <josh@jmatth.com>")
		.about("Scrapes the schedule for conventions by Double Exposure into JSON format")
		.arg(
			Arg::with_name("input")
				.short("i")
				.long("input")
				.value_name("INPUT")
				.help("Where to read the input from, either file or url")
				.required(true)
				.takes_value(true),
		)
		.arg(
			Arg::with_name("output")
				.short("o")
				.long("output")
				.value_name("OUTPUT")
				.help("File to write resulting JSON to")
				.required(false)
				.takes_value(true),
		)
		.arg(
			Arg::with_name("cache")
				.short("c")
				.long("cache")
				.value_name("CACHE")
				.help("File to store HTTP cache headers")
				.required(false)
				.takes_value(true),
		)
		.arg(
			Arg::with_name("start_date")
				.short("d")
				.long("startDate")
				.value_name("START_DATE")
				.help("First day of the convention in the format YYYY-mm-dd")
				.required(true)
				.takes_value(true),
		)
		.arg(
			Arg::with_name("template_config")
				.short("t")
				.long("templateConfig")
				.value_name("TEMPLATE_CONFIG_FILE")
				.help("The base json config to insert the events array into")
				.required(false)
				.takes_value(true),
		)
		.get_matches();
	let input = matches
		.value_of("input")
		.ok_or("Missing required flag 'input'")?;
	let output = matches.value_of("output").unwrap_or("./schedule.json");
	let config_template_path = matches.value_of("template_config").unwrap_or("");
	let cache = matches.value_of("cache").unwrap_or("./cache.json");
	let mut start_date_strs = matches
		.value_of("start_date")
		.ok_or("Missing required flag 'start_date'")?
		.splitn(3, "-");
	let start_date_year = start_date_strs
		.next()
		.ok_or("Provided start_date is invalid: could not parse year")?
		.parse::<i32>()
		.unwrap();
	let start_date_month = start_date_strs
		.next()
		.ok_or("Provided start_date is invalid: could not parse month")?
		.parse::<u32>()
		.unwrap();
	let start_date_day = start_date_strs
		.next()
		.ok_or("Provided start_date is invalid: could not parse day")?
		.parse::<u32>()
		.unwrap();
	let date_parser = DateParser::new(
		start_date_year,
		start_date_month,
		start_date_day,
		CON_TIMEZONE,
	);
	let client = client::CachingClient::new(cache)?;
	if input.starts_with("http://") || input.starts_with("https://") {
		let scrape_result = client.scrape_site(input)?;
		match scrape_result {
			Some(mut http_reader) => parse_events(
				&mut http_reader,
				&output.to_string(),
				&date_parser,
				config_template_path,
			),
			None => {
				println!("Upstream not modified");
				return Ok(());
			}
		}
	} else if input.starts_with("file") {
		// This substring assumes the argument started with "file://"
		let path = &input[7..];
		let mut file_input = File::open(path)?;
		parse_events(
			&mut file_input,
			&output.to_string(),
			&date_parser,
			config_template_path,
		)
	} else {
		Err(Error::StringError(
			"Invalid input, must start with https://, http://, or file://".to_string(),
		))
	}
}

fn parse_events<R: Read>(
	input: &mut R,
	output_path: &String,
	date_parser: &DateParser,
	template_config_path: &str,
) -> Result<(), Error> {
	// let resp = reqwest::get(url).unwrap();
	// assert!(resp.status().is_success());
	let mut cache_dir = std::env::current_dir()?;
	cache_dir.push(".client_cache_headers");
	let cache_dir = cache_dir.to_str()?;
	let _client = client::CachingClient::new(cache_dir);
	let doc = Document::from_read(input).unwrap();
	let mut bg3_nodes = doc.find(pred::And(pred::Name("td"), pred::Class("bg3")));
	let container = bg3_nodes.next().unwrap();
	// Bail if there's more than one possible container
	assert_eq!(None, bg3_nodes.next());

	// assert_eq!(
	//     NUM_RULES,
	//     container
	//         .children()
	//         .filter(|n| n.is(pred::Name("hr")))
	//         .count()
	// );
	let event_children = filter_event_nodes(&container);

	let mut iter = event_children.iter().map(|n| *n);
	let schedule = events::parse_events(&mut iter, date_parser);
	let mut config_val: Value = if template_config_path.len() < 1 {
		serde_json::from_str("{}")?
	} else {
		let template_config_file = File::open(template_config_path)?;
		serde_json::from_reader(template_config_file)?
	};
	let config: &mut Map<String, Value> = config_val.as_object_mut()?;
	config.insert(SCHEDULE_KEY.to_string(), json!(schedule));
	let output = File::create(output_path)?;
	match serde_json::to_writer_pretty(output, &config) {
		Ok(_) => Ok(()),
		Err(err) => Err(Error::JsonError(err)),
	}
}

fn filter_event_nodes<'a>(container: &'a Node) -> Vec<Node<'a>> {
	container
		.children()
		.skip_while(|n| n.is(pred::Name("p")))
		.collect()
}
