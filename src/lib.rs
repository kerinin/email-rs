//! Parser/Serializer for emails
// #![feature(test)]
#![recursion_limit="1000"]
#[macro_use]
extern crate chomp;
extern crate chrono;
extern crate bytes;
#[macro_use]
extern crate log;

// pub mod rfc2822;
pub mod rfc5322;
mod util;

use std::fmt;

use chrono::datetime::DateTime;
use chrono::offset::fixed::FixedOffset;
use bytes::Bytes;
use chomp::*;
use chomp::types::*;
use chomp::parsers::*;
use chomp::combinators::*;

use rfc5322::*;

#[derive(PartialEq)]
pub enum Day { Mon, Tue, Wed, Thu, Fri, Sat, Sun }

#[derive(Debug, PartialEq)]
pub enum Month { Jan, Feb, Mar, Apr, May, Jun, Jul, Aug, Sep, Oct, Nov, Dec }

#[derive(Debug, PartialEq)]
pub enum Address {
    Mailbox {
        local_part: String,
        domain: String,
        display_name: Option<Bytes>,
    },
    Group {
        display_name: Bytes,
        mailboxes: Vec<Address>,
    },
}

#[derive(Debug, PartialEq)]
pub struct MessageID {
    pub id_left: Bytes,
    pub id_right: Bytes,
}

#[derive(Debug, PartialEq)]
pub struct Message<I: U8Input> {
    // pub traces: Vec<Trace>,
    pub fields: Vec<Field<I>>,
    body: Option<I::Buffer>,
}

impl<I: U8Input> Message<I> {
    pub fn body(&self) -> Bytes {
        match self.body {
            Some(ref buf) => Bytes::from_slice(&buf.to_vec()),
            None => Bytes::empty(),
        }
    }

    pub fn from<'a>(&'a self) -> Option<&'a AddressesField<I>> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::From(ref f) => Some(f),
                _ => None,
            }
        }).next()
    }

    pub fn date<'a>(&'a self) -> Option<&'a DateTimeField<I>> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::Date(ref f) => Some(f),
                _ => None,
            }
        }).next()
    }

    pub fn sender<'a>(&'a self) -> Option<&'a AddressField<I>> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::Sender(ref f) => Some(f),
                _ => None,
            }
        }).next()
    }

    pub fn reply_to<'a>(&'a self) -> Option<&'a AddressesField<I>> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::ReplyTo(ref f) => Some(f),
                _ => None,
            }
        }).next()
    }

    pub fn to<'a>(&'a self) -> Option<&'a AddressesField<I>> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::To(ref f) => Some(f),
                _ => None,
            }
        }).next()
    }

    pub fn cc<'a>(&'a self) -> Option<&'a AddressesField<I>> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::Cc(ref f) => Some(f),
                _ => None,
            }
        }).next()
    }

    pub fn bcc<'a>(&'a self) -> Option<&'a AddressesField<I>> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::Bcc(ref f) => Some(f),
                _ => None,
            }
        }).next()
    }

    pub fn message_id<'a>(&'a self) -> Option<&'a MessageIDField<I>> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::MessageID(ref f) => Some(f),
                _ => None,
            }
        }).next()
    }

    pub fn references<'a>(&'a self) -> Option<&'a MessageIDsField<I>> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::References(ref f) => Some(f),
                _ => None,
            }
        }).next()
    }

    pub fn in_reply_to<'a>(&'a self) -> Option<&'a MessageIDsField<I>> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::InReplyTo(ref f) => Some(f),
                _ => None,
            }
        }).next()
    }

    pub fn subject<'a>(&'a self) -> Option<&'a UnstructuredField<I>> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::Subject(ref f) => Some(f),
                _ => None,
            }
        }).next()
    }

    pub fn comments<'a>(&'a self) -> Vec<&'a UnstructuredField<I>> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::Comments(ref f) => Some(f),
                _ => None,
            }
        }).collect()
    }

    pub fn keywords<'a>(&'a self) -> Vec<&'a KeywordsField<I>> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::Keywords(ref f) => Some(f),
                _ => None,
            }
        }).collect()
    }

    pub fn optional<'a>(&'a self) -> Vec<(&'a String, &'a UnstructuredField<I>)> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::Optional(ref k, ref v) => Some((k, v)),
                _ => None,
            }
        }).collect()
    }
}

#[derive(Debug, PartialEq)]
pub struct Trace<I: U8Input> {
    pub return_path: Option<Address>,
    pub fields: Vec<Field<I>>,
}

#[derive(PartialEq)]
pub enum Field<I: U8Input> {
    Date(DateTimeField<I>),
    From(AddressesField<I>),
    Sender(AddressField<I>),
    ReplyTo(AddressesField<I>),
    To(AddressesField<I>),
    Cc(AddressesField<I>),
    Bcc(AddressesField<I>),
    MessageID(MessageIDField<I>),
    InReplyTo(MessageIDsField<I>),
    References(MessageIDsField<I>),
    Subject(UnstructuredField<I>),
    Comments(UnstructuredField<I>),
    Keywords(KeywordsField<I>),
    ReturnPath(AddressField<I>),
    Received(ReceivedField<I>),
    ResentDate(DateTimeField<I>),
    ResentFrom(AddressesField<I>),
    ResentSender(AddressField<I>),
    ResentTo(AddressesField<I>),
    ResentCc(AddressesField<I>),
    ResentBcc(AddressesField<I>),
    ResentReplyTo(AddressesField<I>),
    ResentMessageID(MessageIDField<I>),
    Optional(String, UnstructuredField<I>),
}

impl<I: U8Input> fmt::Debug for Field<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Field::Date(ref v) =>              write!(f, "Date: {}", v.to_string()),
            &Field::From(ref v) =>              write!(f, "From: {}", v.to_string()),
            &Field::Sender(ref v) =>            write!(f, "Sender: {}", v.to_string()),
            &Field::ReplyTo(ref v) =>           write!(f, "Reply-To: {}", v.to_string()),
            &Field::To(ref v) =>                write!(f, "To: {}", v.to_string()),
            &Field::Cc(ref v) =>                write!(f, "Cc: {}", v.to_string()),
            &Field::Bcc(ref v) =>               write!(f, "Bcc: {}", v.to_string()),
            &Field::MessageID(ref v) =>         write!(f, "Message-ID: {}", v.to_string()),
            &Field::InReplyTo(ref v) =>         write!(f, "In-Reply-To: {}", v.to_string()),
            &Field::References(ref v) =>        write!(f, "References: {}", v.to_string()),
            &Field::Subject(ref v) =>           write!(f, "Subject: {}", v.to_string()),
            &Field::Comments(ref v) =>          write!(f, "Comments: {}", v.to_string()),
            &Field::Keywords(ref v) =>          write!(f, "Keywords: {}", v.to_string()),
            &Field::ReturnPath(ref v) =>        write!(f, "Return-Path: {}", v.to_string()),
            &Field::Received(ref v) =>          write!(f, "Received: {}", v.to_string()),
            &Field::ResentDate(ref v) =>        write!(f, "Resent-Date: {}", v.to_string()),
            &Field::ResentFrom(ref v) =>        write!(f, "Resent-From: {}", v.to_string()),
            &Field::ResentSender(ref v) =>      write!(f, "Resent-Sender: {}", v.to_string()),
            &Field::ResentTo(ref v) =>          write!(f, "Resent-To: {}", v.to_string()),
            &Field::ResentCc(ref v) =>          write!(f, "Resent-Cc: {}", v.to_string()),
            &Field::ResentBcc(ref v) =>         write!(f, "Resent-Bcc: {}", v.to_string()),
            &Field::ResentReplyTo(ref v) =>     write!(f, "Resent-Reply-To: {}", v.to_string()),
            &Field::ResentMessageID(ref v) =>   write!(f, "Resent-Message-ID: {}", v.to_string()),
            &Field::Optional(ref n, ref v) =>   write!(f, "{}: {}", n, v.to_string()),
        }
    }
}

#[derive(PartialEq)]
pub struct ReceivedField<I: U8Input> {
    data: I::Buffer,
}

impl<I: U8Input> ReceivedField<I> {
    // *received-token ";" date-time
    pub fn tokens(&self) -> Option<(Vec<Bytes>, DateTime<FixedOffset>)> {
        let parser = |i| {
            many(i, received_token).bind(|i, tokens: Vec<Bytes>| {
                token(i, b';').then(|i| {
                    date_time(i).map(|dt: DateTime<FixedOffset>| (tokens, dt))
                })
            })
        };
        parse_only(parser, &self.data.to_vec()[..]).ok()
    }

    pub fn to_string(&self) -> String {
        let s = &self.data.to_vec()[..self.data.len()-2];
        let cow = String::from_utf8_lossy(s);
        cow.into_owned().to_string()
    }
}

impl<I: U8Input> fmt::Debug for ReceivedField<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}

#[derive(PartialEq)]
pub struct DateTimeField<I: U8Input> {
    data: I::Buffer,
}

impl<I: U8Input> DateTimeField<I> {
    // date-time
    pub fn date_time(&self) -> Option<DateTime<FixedOffset>> {
        parse_only(date_time, &self.data.to_vec()[..]).ok()
    }

    pub fn to_string(&self) -> String {
        let s = &self.data.to_vec()[..self.data.len()-2];
        let cow = String::from_utf8_lossy(s);
        cow.into_owned().to_string()
    }
}

impl<I: U8Input> fmt::Debug for DateTimeField<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}

#[derive(PartialEq)]
pub struct AddressesField<I: U8Input> {
    data: I::Buffer,
}

impl<I: U8Input> AddressesField<I> {
    // address-list
    pub fn addresses(&self) -> Vec<Address> {
        parse_only(address_list, &self.data.to_vec()[..]).unwrap_or(vec!())
    }

    pub fn to_string(&self) -> String {
        let s = &self.data.to_vec()[..self.data.len()-2];
        let cow = String::from_utf8_lossy(s);
        cow.into_owned().to_string()
    }
}

impl<I: U8Input> fmt::Debug for AddressesField<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}

#[derive(PartialEq)]
pub struct AddressField<I: U8Input> {
    data: I::Buffer,
}

impl<I: U8Input> AddressField<I> {

    // mailbox
    pub fn address(&self) -> Option<Address> {
        parse_only(mailbox, &self.data.to_vec()[..]).ok()
    }

    pub fn to_string(&self) -> String {
        let s = &self.data.to_vec()[..self.data.len()-2];
        let cow = String::from_utf8_lossy(s);
        cow.into_owned().to_string()
    }
}

impl<I: U8Input> fmt::Debug for AddressField<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}

#[derive(PartialEq)]
pub struct MessageIDField<I: U8Input> {
    data: I::Buffer,
}

impl<I: U8Input> MessageIDField<I> {
    pub fn message_id(&self) -> Option<MessageID> {
        parse_only(msg_id, &self.data.to_vec()[..]).ok()
    }

    pub fn to_string(&self) -> String {
        let s = &self.data.to_vec()[..self.data.len()-2];
        let cow = String::from_utf8_lossy(s);
        cow.into_owned().to_string()
    }
}

impl<I: U8Input> fmt::Debug for MessageIDField<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}

#[derive(PartialEq)]
pub struct MessageIDsField<I: U8Input> {
    data: I::Buffer,
}

impl<I: U8Input> MessageIDsField<I> {
    //  *(phrase / msg-id)
    //  For purposes of interpretation, the phrases in the "In-Reply-To:" and
    //  "References:" fields are ignored.
    pub fn message_ids(&self) -> Vec<MessageID> {
        parse_only(|i| {
            many(i, |i| {
                or(i, 
                   |i| phrase(i).map(|_| None),
                   |i| msg_id(i).map(|v| Some(v)))
            }).map(|vs: Vec<Option<MessageID>>| {
                vs.into_iter()
                    .filter(|v| v.is_some())
                    .map(|v| v.unwrap())
                    .collect::<Vec<MessageID>>()
            })
        }, &self.data.to_vec()[..]).unwrap_or(vec!())
    }

    pub fn to_string(&self) -> String {
        let s = &self.data.to_vec()[..self.data.len()-2];
        let cow = String::from_utf8_lossy(s);
        cow.into_owned().to_string()
    }
}

impl<I: U8Input> fmt::Debug for MessageIDsField<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}

#[derive(PartialEq)]
pub struct UnstructuredField<I: U8Input> {
    data: I::Buffer,
}

impl<I: U8Input> UnstructuredField<I> {
    pub fn to_string(&self) -> String {
        let s = &self.data.to_vec()[..self.data.len()-2];
        let cow = String::from_utf8_lossy(s);
        cow.into_owned().to_string()
    }
}

impl<I: U8Input> fmt::Debug for UnstructuredField<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}

#[derive(PartialEq)]
pub struct KeywordsField<I: U8Input> {
    data: I::Buffer,
}

impl<I: U8Input> KeywordsField<I> {
    pub fn keywords(&self) -> Vec<Bytes> {
        vec!()
    }

    pub fn to_string(&self) -> String {
        let s = &self.data.to_vec()[..self.data.len()-2];
        let cow = String::from_utf8_lossy(s);
        cow.into_owned().to_string()
    }
}

impl<I: U8Input> fmt::Debug for KeywordsField<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}

impl<I: U8Input> Field<I> {
    /// Returns true if this is an "unstructured" field
    pub fn is_unstructured(&self) -> bool {
        match self {
            &Field::Optional(_, _) => true,
            _ => false,
        }
    }

    /// Returns true if "structured field" parsing failed
    pub fn is_malformed(&self) -> bool {
        match self {
            &Field::Optional(ref name, _) => {
                match name.to_lowercase().as_str() {
                    "date" => true,
                    "from" => true,
                    "sender" => true,
                    "reply-to" => true,
                    "to" => true,
                    "cc" => true,
                    "bcc" => true,
                    "message-id" => true,
                    "in-reply-to" => true,
                    "references" => true,
                    "subject" => true,
                    "comments" => true,
                    "keywords" => true,
                    "resent-date" => true,
                    "resent-from" => true,
                    "resent-sender" => true,
                    "resent-to" => true,
                    "resent-cc" => true,
                    "resent-bcc" => true,
                    "resent-message-id" => true,
                    "return-path" => true,
                    "received" => true,
                    _ => false,
                }
            },
            _ => false,
        }
    }
}
