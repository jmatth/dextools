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

mod events;

use crate::events::matchers::dateparse::DateParser;


fn main() -> Result<(), String>{
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
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap_or("./schedule.json");
    let mut start_date_strs = matches.value_of("start_date").unwrap().splitn(3, "-");
    let start_date_year = start_date_strs.next().unwrap().parse::<i32>().unwrap();
    let start_date_month = start_date_strs.next().unwrap().parse::<u32>().unwrap();
    let start_date_day = start_date_strs.next().unwrap().parse::<u32>().unwrap();
    let con_name = matches.value_of("con_name").unwrap();
    let con_email = matches.value_of("con_email").unwrap_or("");
    let date_parser = DateParser::new(start_date_year, start_date_month, start_date_day, 4 * 3600);
    let input_file = File::open(input).unwrap();
    scrape_dexposure(input_file, &output.to_string(), &date_parser, con_name.to_string(), con_email.to_string())
}

fn scrape_dexposure(input: File, output_path: &String, date_parser: &DateParser, con_name: String, con_email: String) -> Result<(), String> {
    // let resp = reqwest::get(url).unwrap();
    // assert!(resp.status().is_success());

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

    let mut iter = event_children
        .iter()
        .map(|n| *n)
        .peekable();
    let schedule = events::parse_events(&mut iter, date_parser);
    let output_object = Settings {
        con_name,
        con_email,
        schedule,
    };
    let output = File::create(output_path).unwrap();
    match serde_json::to_writer_pretty(output, &output_object) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to write json output: {}", err.to_string()))
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
