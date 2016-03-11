//! RFC2822 specifies message bodies (supercedes RFC822)
use chrono::datetime::DateTime;
use chrono::offset::fixed::FixedOffset;

pub mod address;
pub mod atom;
pub mod datetime;
pub mod fields;
pub mod folding;
pub mod message;
pub mod misc;
pub mod obsolete;
pub mod primitive;
pub mod quoted;

#[derive(Debug)]
pub enum Day { Mon, Tue, Wed, Thu, Fri, Sat, Sun }

#[derive(Debug)]
pub enum Month { Jan, Feb, Mar, Apr, May, Jun, Jul, Aug, Sep, Oct, Nov, Dec }

#[derive(Clone, PartialEq, Debug)]
pub enum Address {
    Mailbox {
        local_part: String,
        domain: String,
        display_name: Option<Vec<u8>>,
    },
    Group {
        display_name: String,
        mailboxes: Vec<Address>,
    },
}

#[derive(Debug)]
pub enum Field {
    Date(DateTime<FixedOffset>),
    From(Vec<Address>),
    Sender(Address),
    ReplyTo(Vec<Address>),
    To(Vec<Address>),
    Cc(Vec<Address>),
    Bcc(Vec<Address>),
    MessageID(Vec<u8>),
    InReplyTo(Vec<Vec<u8>>),
    References(Vec<Vec<u8>>),
    Subject(Vec<u8>),
    Comments(Vec<u8>),
    Keywords(Vec<Vec<u8>>),
    ReturnPath(Address),
    Optional(Vec<u8>, Vec<u8>),
}

impl Field {
    pub fn is_unstructured(&self) -> bool {
        match self {
            &Field::Optional(_, _) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum Resent {
    Date(DateTime<FixedOffset>),
    From(Vec<Address>),
    Sender(Address),
    To(Vec<Address>),
    Cc(Vec<Address>),
    Bcc(Vec<Address>),
    MessageID(Vec<u8>),
}

#[derive(Debug)]
pub enum ReceivedValue {
    Addresses(Vec<Address>),
    Address(Address),
    Domain(Vec<u8>),
    MessageID(Vec<u8>),
    Text(Vec<u8>),
}

#[derive(Debug)]
pub struct Received {
    pub date_time: DateTime<FixedOffset>,
    pub data: Vec<(Vec<u8>, ReceivedValue)>,
}

#[derive(Debug)]
pub struct Trace {
    pub return_path: Option<Address>,
    pub received: Vec<Received>,
    pub fields: Vec<Resent>,
}

#[derive(Debug)]
pub struct Message {
    pub traces: Vec<Trace>,
    pub fields: Vec<Field>,
    pub body: Vec<u8>,
}
