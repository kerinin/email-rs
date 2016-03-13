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
pub struct MessageID {
    id_left: Vec<u8>,
    id_right: Vec<u8>,
}

#[derive(Debug)]
pub struct Message {
    pub traces: Vec<Trace>,
    pub fields: Vec<Field>,
    pub body: Vec<u8>,
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
    pub data: Vec<u8>,
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
}

#[derive(Debug)]
pub struct MessageIDsField {
    message_ids: Vec<MessageID>,
}

#[derive(Debug)]
pub struct UnstructuredField {
    data: Vec<u8>,
}

#[derive(Debug)]
pub struct KeywordsField {
    keywords: Vec<Vec<u8>>,
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
//     Domain(Vec<u8>),
//     MessageID(Vec<u8>),
//     Text(Vec<u8>),
// }

