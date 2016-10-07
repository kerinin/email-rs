#[macro_use]
extern crate mail;
extern crate chomp;
extern crate bytes;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::io;
use std::collections::HashSet;

use bytes::str::ByteStr;
use chomp::buffer::{Source, Stream, StreamError};

use mail::*;
use mail::rfc5322::*;

pub fn main() {
    env_logger::init().unwrap();

    let mut parsed_field_names = HashSet::new();
    // parsed_field_names.insert("received".to_string()); // NOTE: Observed values depart wildly from spec
    parsed_field_names.insert("date".to_string());
    parsed_field_names.insert("from".to_string());
    parsed_field_names.insert("sender".to_string());
    parsed_field_names.insert("reply-to".to_string());
    parsed_field_names.insert("to".to_string());
    parsed_field_names.insert("cc".to_string());
    parsed_field_names.insert("message-id".to_string());
    parsed_field_names.insert("in-reply-to".to_string());
    parsed_field_names.insert("references".to_string());
    parsed_field_names.insert("subject".to_string());
    parsed_field_names.insert("comments".to_string());

    let mut input = Source::new(io::stdin());

    loop {
        match input.parse(message) {
            Ok(m) => {
                output_message(m, &parsed_field_names);
                break
            },
            Err(StreamError::Retry) => continue,
            Err(e) => {
                println!("Error parsing from STDIN: {:?}", e);
                break
            }
        }
    }
}

fn output_message(m: Message, parsed_field_names: &HashSet<String>) {
    for field in m.fields.iter() {
        match field {
            &Field::Optional(ref n, ref f) => {
                let buf = f.data.buf();
                let s = String::from_utf8_lossy(buf.bytes());
                if parsed_field_names.contains(&n.to_lowercase()) {
                    error!("failed to parse {}: {:?}", n, s);
                } else {
                    debug!("(unstructured) {}: {:?}", n, s);
                }
            },
            _ => debug!("{:?}", field),
        }
    }
    debug!("Body bytes: {}", m.body.map_or(0, |b| b.len()));
}
