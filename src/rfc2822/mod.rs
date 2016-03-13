//! RFC2822 specifies message bodies (supercedes RFC822)

use bytes::Bytes;
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
        display_name: Option<Bytes>,
    },
    Group {
        display_name: Bytes,
        mailboxes: Vec<Address>,
    },
}

#[derive(Debug)]
pub struct MessageID {
    id_left: Bytes,
    id_right: Bytes,
}

#[derive(Debug)]
pub struct Message {
    pub traces: Vec<Trace>,
    pub fields: Vec<Field>,
    pub body: Bytes,
}

#[derive(Debug)]
pub struct Trace {
    pub return_path: Option<Address>,
    pub received: Vec<ReceivedField>,
    pub fields: Vec<Resent>,
}

#[derive(Debug)]
pub struct ReceivedField {
    pub date_time: DateTime<FixedOffset>,
    pub data: Bytes,
}

#[derive(Debug)]
pub enum Resent {
    Date(DateTimeField),
    From(AddressesField),
    Sender(AddressField),
    To(AddressesField),
    Cc(AddressesField),
    Bcc(AddressesField),
    MessageID(MessageIDField),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct DateTimeField {
    date_time: DateTime<FixedOffset>,
}

#[derive(Debug)]
pub struct AddressesField {
    addresses: Vec<Address>,
}

#[derive(Debug)]
pub struct AddressField {
    address: Address,
}

#[derive(Debug)]
pub struct MessageIDField {
    message_id: MessageID,
}

#[derive(Debug)]
pub struct MessageIDsField {
    message_ids: Vec<MessageID>,
}

#[derive(Debug)]
pub struct UnstructuredField {
    data: Bytes,
}

#[derive(Debug)]
pub struct KeywordsField {
    keywords: Vec<Bytes>,
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

// #[derive(Debug)]
// pub enum ReceivedValue {
//     Addresses(Vec<Address>),
//     Address(Address),
//     Domain(Bytes),
//     MessageID(Bytes),
//     Text(Bytes),
// }

