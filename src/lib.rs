//! Parser/Serializer for emails
// #![feature(test)]
#![recursion_limit="1000"]
#[macro_use]
extern crate chomp;
extern crate chrono;
extern crate bytes;

// pub mod rfc2822;
pub mod rfc5322;
mod util;

use chrono::datetime::DateTime;
use chrono::offset::fixed::FixedOffset;
use bytes::Bytes;

#[derive(Clone, Debug, PartialEq)]
pub enum Day { Mon, Tue, Wed, Thu, Fri, Sat, Sun }

#[derive(Clone, Debug, PartialEq)]
pub enum Month { Jan, Feb, Mar, Apr, May, Jun, Jul, Aug, Sep, Oct, Nov, Dec }

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct MessageID {
    pub id_left: Bytes,
    pub id_right: Bytes,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Message {
    // pub traces: Vec<Trace>,
    pub fields: Vec<Field>,
    pub body: Option<Bytes>,
}

impl Message {
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

    pub fn subject<'a>(&'a self) -> Option<&'a UnstructuredField> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::Subject(ref f) => Some(f),
                _ => None,
            }
        }).next()
    }

    pub fn comments<'a>(&'a self) -> Vec<&'a UnstructuredField> {
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

    pub fn optional<'a>(&'a self) -> Vec<(&'a str, &'a UnstructuredField)> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::Optional(ref k, ref v) => Some((k.as_str(), v)),
                _ => None,
            }
        }).collect()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Trace {
    pub return_path: Option<Address>,
    pub received: Vec<ReceivedField>,
    pub fields: Vec<Resent>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReceivedField {
    pub date_time: DateTime<FixedOffset>,
    pub data: Bytes,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Resent {
    Date(DateTimeField),
    From(AddressesField),
    Sender(AddressField),
    To(AddressesField),
    Cc(AddressesField),
    Bcc(AddressesField),
    ReplyTo(AddressesField),
    MessageID(MessageIDField),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Field {
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
    Subject(UnstructuredField),
    Comments(UnstructuredField),
    Keywords(KeywordsField),
    ReturnPath(AddressField),
    Optional(String, UnstructuredField),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DateTimeField {
    pub date_time: DateTime<FixedOffset>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AddressesField {
    pub addresses: Vec<Address>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AddressField {
    pub address: Address,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MessageIDField {
    pub message_id: MessageID,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MessageIDsField {
    pub message_ids: Vec<MessageID>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnstructuredField {
    pub data: Bytes,
}

#[derive(Clone, Debug, PartialEq)]
pub struct KeywordsField {
    pub keywords: Vec<Bytes>,
}

impl Field {
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
