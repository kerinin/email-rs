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
pub mod mime;
mod util;

use std::fmt;
use std::collections::HashMap;

use chrono::datetime::DateTime;
use chrono::offset::fixed::FixedOffset;
use bytes::Bytes;
use bytes::ByteStr;
use chomp::*;
use chomp::types::*;
use chomp::parsers::*;
use chomp::combinators::*;

use rfc5322::*;
use mime::*;

pub enum FieldValue<T> {
    Ok(T),
    Raw(Bytes),
    Missing,
}

impl<T> FieldValue<T> {
    pub fn is_ok(&self) -> bool {
        match self {
            &FieldValue::Ok(_) => true,
            _ => false,
        }
    }
    pub fn is_raw(&self) -> bool {
        match self {
            &FieldValue::Raw(_) => true,
            _ => false,
        }
    }
    pub fn is_missing(&self) -> bool {
        match self {
            &FieldValue::Missing => true,
            _ => false,
        }
    }
    pub fn unwrap(self) -> T {
        match self {
            FieldValue::Ok(v) => v,
            FieldValue::Raw(b) => panic!("unwrap raw value {:?}", b),
            FieldValue::Missing => panic!("unwrap missing value"),
        }
    }
    // TODO: Return Vec<u8>
    pub fn raw(&self) -> String {
        match self {
            &FieldValue::Raw(ref b) => String::from_utf8(b.buf().bytes().to_vec()).unwrap(),
            &FieldValue::Ok(_) => panic!("raw called on parsed value"),
            &FieldValue::Missing => panic!("raw called on missing value"),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for FieldValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &FieldValue::Ok(ref v) => write!(f, "{:?}", v),
            &FieldValue::Raw(ref b) => write!(f, "{:?}", b),
            &FieldValue::Missing => write!(f, "<missing>"),
        }
    }
}

#[derive(PartialEq)]
pub enum Day { Mon, Tue, Wed, Thu, Fri, Sat, Sun }

#[derive(Debug, PartialEq)]
pub enum Month { Jan, Feb, Mar, Apr, May, Jun, Jul, Aug, Sep, Oct, Nov, Dec }

#[derive(Debug, PartialEq)]
pub enum Address {
    Mailbox {
        local_part: String,
        domain: String,
        display_name: Option<String>,
    },
    Group {
        display_name: String,
        mailboxes: Vec<Address>,
    },
}

#[derive(Debug, PartialEq)]
pub struct MessageID {
    pub id_left: Option<String>,
    pub id_right: String,
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

    pub fn from(&self) -> FieldValue<Vec<Address>> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::From(ref f) => Some(f.addresses()),
                _ => None,
            }
        }).next().unwrap_or(FieldValue::Missing)
    }

    pub fn date(&self) -> FieldValue<DateTime<FixedOffset>> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::Date(ref f) => Some(f.date_time()),
                _ => None,
            }
        }).next().unwrap_or(FieldValue::Missing)
    }

    pub fn sender(&self) -> FieldValue<Address> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::Sender(ref f) => Some(f.address()),
                _ => None,
            }
        }).next().unwrap_or(FieldValue::Missing)
    }

    pub fn reply_to(&self) -> FieldValue<Vec<Address>> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::ReplyTo(ref f) => Some(f.addresses()),
                _ => None,
            }
        }).next().unwrap_or(FieldValue::Missing)
    }

    pub fn to(&self) -> FieldValue<Vec<Address>> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::To(ref f) => Some(f.addresses()),
                _ => None,
            }
        }).next().unwrap_or(FieldValue::Missing)
    }

    pub fn cc(&self) -> FieldValue<Vec<Address>> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::Cc(ref f) => Some(f.addresses()),
                _ => None,
            }
        }).next().unwrap_or(FieldValue::Missing)
    }

    pub fn bcc(&self) -> FieldValue<Vec<Address>> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::Bcc(ref f) => Some(f.addresses()),
                _ => None,
            }
        }).next().unwrap_or(FieldValue::Missing)
    }

    pub fn message_id(&self) -> FieldValue<MessageID> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::MessageID(ref f) => Some(f.message_id()),
                _ => None,
            }
        }).next().unwrap_or(FieldValue::Missing)
    }

    pub fn references(&self) -> FieldValue<Vec<MessageID>> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::References(ref f) => Some(f.message_ids()),
                _ => None,
            }
        }).next().unwrap_or(FieldValue::Missing)
    }

    pub fn in_reply_to(&self) -> FieldValue<Vec<MessageID>> {
        self.fields.iter().filter_map(|i| {
            match i {
                &Field::InReplyTo(ref f) => Some(f.message_ids()),
                _ => None,
            }
        }).next().unwrap_or(FieldValue::Missing)
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
    MIMEVersion(MIMEVersionField),
    ContentType(ContentTypeField),
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
            &Field::MIMEVersion(ref v) =>       write!(f, "MIME-Version: {}.{}", v.top_version, v.sub_version),
            // TODO: write params too
            &Field::ContentType(ref v) =>       write!(f, "Content-Type: {}/{}", v.top_level.to_string(), v.sub_level.to_string()),
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
    /*
    pub fn tokens(&self) -> FieldValue<(Vec<Bytes>, DateTime<FixedOffset>)> {
        let data = self.data.to_vec();
        let parser = |i| {
            many(i, received_token).bind(|i, tokens: Vec<Vec<I::Buffer>>| {
                token(i, b';').then(|i| {
                    let token_bytes: Vec<Bytes> = tokens.into_iter().map(|t: Vec<I::Buffer>| {
                        t.into_iter().fold(Bytes::empty(), |l, r| l.concat(&Bytes::from_slice(&r.into_vec())))
                    }).collect();

                    date_time(i).map(|dt: DateTime<FixedOffset>| (token_bytes, dt))
                })
            })
        };
        match parse_only(parser, &data[..]) {
            Ok(v) => FieldValue::Ok(v),
            Err(_) => FieldValue::Raw(Bytes::from_slice(&data[..])),
        }
    }
    */

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
    pub fn date_time(&self) -> FieldValue<DateTime<FixedOffset>> {
        let data = self.data.to_vec();
        match parse_only(date_time, &data[..]) {
            Ok(v) => FieldValue::Ok(v),
            Err(_) => FieldValue::Raw(Bytes::from_slice(&data[..])),
        }
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
    pub fn addresses(&self) -> FieldValue<Vec<Address>> {
        let data = self.data.to_vec();
        match parse_only(address_list, &data[..]) {
            Ok(v) => FieldValue::Ok(v),
            Err(_) => FieldValue::Raw(Bytes::from_slice(&data[..])),
        }
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
    pub fn address(&self) -> FieldValue<Address> {
        let data = self.data.to_vec();
        match parse_only(mailbox, &data[..]) {
            Ok(v) => FieldValue::Ok(v),
            Err(_) => FieldValue::Raw(Bytes::from_slice(&data[..])),
        }
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
    pub fn message_id(&self) -> FieldValue<MessageID> {
        let data = self.data.to_vec();
        match parse_only(msg_id, &data[..]) {
            Ok(v) => FieldValue::Ok(v),
            Err(_) => FieldValue::Raw(Bytes::from_slice(&data[..])),
        }
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
    pub fn message_ids(&self) -> FieldValue<Vec<MessageID>> {
        let data = self.data.to_vec();
        let parser = |i| {
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
        };
        match parse_only(parser, &data[..]) {
            Ok(v) => FieldValue::Ok(v),
            Err(_) => FieldValue::Raw(Bytes::from_slice(&data[..])),
        }
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
pub struct MIMEVersionField {
    pub top_version: usize,
    pub sub_version: usize,
}

#[derive(PartialEq)]
pub struct ContentTypeField {
    pub top_level: TopLevel,
    pub sub_level: SubLevel,
    pub params: HashMap<String, Vec<u8>>,
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
            // Received formats vary wildly, so ignore it here
            // &Field::Received(ref v) =>          v.tokens().is_raw(),
            &Field::Date(ref v) =>              v.date_time().is_raw(),
            &Field::From(ref v) =>              v.addresses().is_raw(),
            &Field::Sender(ref v) =>            v.address().is_raw(),
            &Field::ReplyTo(ref v) =>           v.addresses().is_raw(),
            &Field::To(ref v) =>                v.addresses().is_raw(),
            &Field::Cc(ref v) =>                v.addresses().is_raw(),
            &Field::MessageID(ref v) =>         v.message_id().is_raw(),
            &Field::InReplyTo(ref v) =>         v.message_ids().is_raw(),
            &Field::References(ref v) =>        v.message_ids().is_raw(),
            &Field::Subject(_) =>               false,
            &Field::Comments(_) =>              false,
            &Field::ResentFrom(ref v) =>        v.addresses().is_raw(),
            &Field::ResentSender(ref v) =>      v.address().is_raw(),
            &Field::ResentDate(ref v) =>        v.date_time().is_raw(),
            &Field::ResentTo(ref v) =>          v.addresses().is_raw(),
            &Field::ResentCc(ref v) =>          v.addresses().is_raw(),
            &Field::ResentBcc(ref v) =>         v.addresses().is_raw(),
            &Field::ResentMessageID(ref v) =>   v.message_id().is_raw(),
            &Field::ResentReplyTo(ref v) =>     v.addresses().is_raw(),
            _ =>                                false,
        }
    }
}
