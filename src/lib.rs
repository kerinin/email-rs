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
use std::borrow;

use chrono::datetime::DateTime;
use chrono::offset::fixed::FixedOffset;
use bytes::Bytes;
use chomp::types::*;

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
    pub body: Option<Bytes>,
}

impl<I: U8Input> Message<I> {
    pub fn from<'a>(&'a self) -> Option<&'a AddressesField> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::From(ref f) => Some(f),
                _ => None,
            }
        }).next()
    }

    pub fn date<'a>(&'a self) -> Option<&'a DateTimeField> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::Date(ref f) => Some(f),
                _ => None,
            }
        }).next()
    }

    pub fn sender<'a>(&'a self) -> Option<&'a AddressField> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::Sender(ref f) => Some(f),
                _ => None,
            }
        }).next()
    }

    pub fn reply_to<'a>(&'a self) -> Option<&'a AddressesField> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::ReplyTo(ref f) => Some(f),
                _ => None,
            }
        }).next()
    }

    pub fn to<'a>(&'a self) -> Option<&'a AddressesField> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::To(ref f) => Some(f),
                _ => None,
            }
        }).next()
    }

    pub fn cc<'a>(&'a self) -> Option<&'a AddressesField> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::Cc(ref f) => Some(f),
                _ => None,
            }
        }).next()
    }

    pub fn bcc<'a>(&'a self) -> Option<&'a AddressesField> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::Bcc(ref f) => Some(f),
                _ => None,
            }
        }).next()
    }

    pub fn message_id<'a>(&'a self) -> Option<&'a MessageIDField> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::MessageID(ref f) => Some(f),
                _ => None,
            }
        }).next()
    }

    pub fn references<'a>(&'a self) -> Option<&'a MessageIDsField> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::References(ref f) => Some(f),
                _ => None,
            }
        }).next()
    }

    pub fn in_reply_to<'a>(&'a self) -> Option<&'a MessageIDsField> {
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

    pub fn keywords<'a>(&'a self) -> Vec<&'a KeywordsField> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::Keywords(ref f) => Some(f),
                _ => None,
            }
        }).collect()
    }

    pub fn optional<'a>(&'a self) -> Vec<(&'a str, &'a UnstructuredField<I>)> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::Optional(ref k, ref v) => Some((k.as_str(), v)),
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

#[derive(Debug, PartialEq)]
pub enum Field<I: U8Input> {
    Date(DateTimeField),
    From(AddressesField),
    Sender(AddressField),
    ReplyTo(AddressesField),
    To(AddressesField),
    Cc(AddressesField),
    Bcc(AddressesField),
    MessageID(MessageIDField),
    InReplyTo(MessageIDsField),
    References(MessageIDsField),
    Subject(UnstructuredField<I>),
    Comments(UnstructuredField<I>),
    Keywords(KeywordsField),
    ReturnPath(AddressField),
    Received(ReceivedField),
    ResentDate(DateTimeField),
    ResentFrom(AddressesField),
    ResentSender(AddressField),
    ResentTo(AddressesField),
    ResentCc(AddressesField),
    ResentBcc(AddressesField),
    ResentReplyTo(AddressesField),
    ResentMessageID(MessageIDField),
    Optional(String, UnstructuredField<I>),
}

#[derive(Debug, PartialEq)]
pub struct ReceivedField {
    pub date_time: DateTime<FixedOffset>,
    pub tokens: Vec<Bytes>,
}

#[derive(Debug, PartialEq)]
pub struct DateTimeField {
    pub date_time: DateTime<FixedOffset>,
}

#[derive(Debug, PartialEq)]
pub struct AddressesField {
    pub addresses: Vec<Address>,
}

#[derive(Debug, PartialEq)]
pub struct AddressField {
    pub address: Address,
}

#[derive(Debug, PartialEq)]
pub struct MessageIDField {
    pub message_id: MessageID,
}

#[derive(Debug, PartialEq)]
pub struct MessageIDsField {
    pub message_ids: Vec<MessageID>,
}

#[derive(PartialEq)]
pub struct UnstructuredField<I: U8Input> {
    data: I::Buffer,
}

impl<I: U8Input> UnstructuredField<I> {
    pub fn data<'a>(&self) -> String {
        let s = &self.data.to_vec()[..];
        let cow = String::from_utf8_lossy(s);
        cow.into_owned().to_string()
    }
}

impl<I: U8Input> fmt::Debug for UnstructuredField<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.data())
    }
}

#[derive(Debug, PartialEq)]
pub struct KeywordsField {
    pub keywords: Vec<Bytes>,
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
