#![feature(try_trait)]

extern crate reqwest;
extern crate select;
extern crate serde_json;
extern crate chrono;
extern crate clap;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

use select::document::Document;
use select::node::Node;
use std::fs::File;
use select::predicate as pred;
use clap::{Arg, App};
use std::io::Read;

mod events;
mod client;
mod error;

use error::Error;

use crate::events::matchers::dateparse::DateParser;


fn main() -> Result<(), Error>{
    let matches = App::new("Dextools scraper")
        .version("1.0")
        .author("Joshua Matthews <josh@jmatth.com>")
        .about("Scrapes the schedule for conventions by Double Exposure into JSON format")
        .arg(Arg::with_name("input")
             .short("i")
             .long("input")
             .value_name("INPUT")
             .help("Where to read the input from, either file or url")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("output")
             .short("o")
             .long("output")
             .value_name("OUTPUT")
             .help("File to write resulting JSON to")
             .required(false)
             .takes_value(true))
        .arg(Arg::with_name("start_date")
             .short("d")
             .long("startDate")
             .value_name("START_DATE")
             .help("First day of the convention in the format YYYY-mm-dd")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("con_name")
             .short("n")
             .long("conName")
             .value_name("CON_NAME")
             .help("The name of the convention, e.g. Dreamation 2019")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("con_email")
             .short("e")
             .long("conEmail")
             .value_name("CON_EMAIL")
             .help("The email address to send event registrations to, if available")
             .required(false)
             .takes_value(true))
        .get_matches();
    let input = matches.value_of("input").ok_or("Missing required flag 'input'")?;
    let output = matches.value_of("output").unwrap_or("./schedule.json");
    let mut start_date_strs = matches.value_of("start_date").ok_or("Missing required flag 'start_date'")?.splitn(3, "-");
    let start_date_year = start_date_strs.next()
        .ok_or("Provided start_date is invalid: could not parse year")?
        .parse::<i32>().unwrap();
    let start_date_month = start_date_strs.next()
        .ok_or("Provided start_date is invalid: could not parse month")?
        .parse::<u32>().unwrap();
    let start_date_day = start_date_strs.next()
        .ok_or("Provided start_date is invalid: could not parse day")?
        .parse::<u32>().unwrap();
    let con_name = matches.value_of("con_name").ok_or("Missing required flag 'con_name'")?;
    let con_email = matches.value_of("con_email").unwrap_or("");
    let date_parser = DateParser::new(start_date_year, start_date_month, start_date_day, 4 * 3600);
    let client = client::CachingClient::new("./cache.json")?;
    let scrape_result = client.scrape_site(input)?;
    let mut input_reader = match scrape_result {
        Some(reader) => reader,
        None => {
            println!("Upstream not modified");
            return Ok(());
        }
    };
    // let input_file = File::open(input).unwrap();
    parse_events(&mut input_reader, &output.to_string(), &date_parser, con_name.to_string(), con_email.to_string())
}

fn parse_events(input: &mut dyn Read, output_path: &String, date_parser: &DateParser, con_name: String, con_email: String) -> Result<(), Error> {
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
    let output_object = Settings {
        con_name,
        con_email,
        schedule,
    };
    let output = File::create(output_path).unwrap();
    match serde_json::to_writer_pretty(output, &output_object) {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::JsonError(err)),
    }
}

fn filter_event_nodes<'a>(container: &'a Node) -> Vec<Node<'a>> {
    container
        .children()
        .skip_while(|n| {
            n.is(pred::Name("p"))
        })
        .collect()
}


#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    con_name: String,
    con_email: String,
    schedule: Vec<events::Event>,
}
