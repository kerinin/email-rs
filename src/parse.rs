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
use chomp::types::*;

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

fn output_message<I: U8Input>(m: Message<I>, parsed_field_names: &HashSet<String>) {
    for field in m.fields.iter() {
        match field {
            &Field::Date(ref v) =>  {
                if v.date_time().is_raw() {
                    error!("failed to parse Date: {}", v.date_time().raw());
                }
            },
            &Field::From(ref v) => {
                if v.addresses().is_raw() {
                    error!("failed to parse From: {}", v.addresses().raw());
                }
            },
            &Field::Sender(ref v) => {
                if v.address().is_raw() {
                    error!("failed to parse Sender: {}", v.address().raw());
                }
            },
            &Field::ReplyTo(ref v) => {
                if v.addresses().is_raw() {
                    error!("failed to parse Reply-To: {}", v.addresses().raw());
                }
            },
            &Field::To(ref v) => {
                if v.addresses().is_raw() {
                    error!("failed to parse To: {}", v.addresses().raw());
                }
            },
            &Field::Cc(ref v) => {
                if v.addresses().is_raw() {
                    error!("failed to parse Cc: {}", v.addresses().raw());
                }
            },
            &Field::MessageID(ref v) => {
                if v.message_id().is_raw() {
                    error!("failed to parse Message-ID: {}", v.message_id().raw());
                }
            },
            &Field::InReplyTo(ref v) => {
                if v.message_ids().is_raw() {
                    error!("failed to parse In-Reply-To: {}", v.message_ids().raw());
                }
            },
            &Field::References(ref v) => {
                if v.message_ids().is_raw() {
                    error!("failed to parse References: {}", v.message_ids().raw());
                }
            },
            &Field::Optional(ref n, ref f) => {
                if parsed_field_names.contains(&n.to_lowercase()) {
                    error!("failed to parse {}: {}", n, f.to_string());
                } else {
                    debug!("-> (unstructured) {}: {}", n, f.to_string());
                }
            },
            _ => debug!("-> {:?}", field),
        }
    }
    debug!("Body bytes: {}", m.body().len());
}
