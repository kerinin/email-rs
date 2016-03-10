//! RFC2822 specifies message bodies (supercedes RFC822)
use chrono::datetime::DateTime;
use chrono::offset::fixed::FixedOffset;

mod address;
mod atom;
mod datetime;
mod fields;
mod folding;
mod misc;
mod obsolete;
mod primitive;
mod quoted;

pub enum Day { Mon, Tue, Wed, Thu, Fri, Sat, Sun }

pub enum Month { Jan, Feb, Mar, Apr, May, Jun, Jul, Aug, Sep, Oct, Nov, Dec }

#[derive(Clone, PartialEq)]
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

pub enum ReceivedValue {
    Addresses(Vec<Address>),
    Address(Address),
    Domain(Vec<u8>),
    MessageID(Vec<u8>),
    Text(Vec<u8>),
}

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
    ResentDate(DateTime<FixedOffset>),
    ResentFrom(Vec<Address>),
    ResentSender(Address),
    ResentTo(Vec<Address>),
    ResentCc(Vec<Address>),
    ResentBcc(Vec<Address>),
    ResentMessageID(Vec<u8>),
    ReturnPath(Address),
    Received(Vec<(Vec<u8>, ReceivedValue)>, DateTime<FixedOffset>),
}
