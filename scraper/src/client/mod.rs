use std::io::{Seek, SeekFrom};
use std::time::Duration;
use std::fs::File;
use std::fs::OpenOptions;
use std::collections::HashMap;

use reqwest;
use reqwest::header::ETAG;
use reqwest::header::LAST_MODIFIED;
use reqwest::header::IF_NONE_MATCH;
use reqwest::header::IF_MODIFIED_SINCE;
use reqwest::header::HeaderName;
use reqwest::StatusCode;
use serde_json;

use super::Error;

const USED_HEADERS: [(HeaderName, HeaderName); 2] = [
    (ETAG, IF_NONE_MATCH),
    (LAST_MODIFIED, IF_MODIFIED_SINCE),
];

pub struct CachingClient<'a> {
    cache_file_path: &'a str,
    client: reqwest::Client,
}

impl<'a> CachingClient<'a> {
    pub fn new(cache_file_path: &'a str) -> Result<Self, Error> {
        let client = reqwest::Client::builder()
            .gzip(true)
            .timeout(Duration::from_secs(10))
            .build()?;
        Ok(Self { client, cache_file_path })
    }

    pub fn scrape_site(&self, url: &str) -> Result<Option<reqwest::Response>, Error> {
        let mut cache_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(self.cache_file_path)?;
        let mut headers_map = get_headers(&cache_file);
        let our_headers = headers_map.entry(url.to_string()).or_default();
        let mut request = self.client.get(url);
        for (key, value) in our_headers.iter() {
            request = request.header(key.as_str(), value.as_bytes());
        }
        let response = request.send()?;

        if response.status() == StatusCode::NOT_MODIFIED {
            return Ok(None)
        }

        for (from, to) in &USED_HEADERS {
            match response.headers().get(from) {
                None => (),
                Some(found_header) => {
                    our_headers.insert(to.to_string(), found_header.to_str()?.to_string());
                },
            };
        }
        cache_file.seek(SeekFrom::Start(0))?;
        serde_json::to_writer(cache_file, &headers_map)?;
        Ok(Some(response))
    }
}

fn get_headers(cache_file: &File) -> HashMap<String, HashMap<String, String>> {
    serde_json::from_reader(cache_file).unwrap_or(HashMap::new())
}
