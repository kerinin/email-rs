//! RFC2822 specifies message bodies (supercedes RFC822)

mod address;
mod atom;
mod datetime;
mod folding;
mod misc;
mod obsolete;
mod primitive;
mod quoted;

pub enum Day { Mon, Tue, Wed, Thu, Fri, Sat, Sun }

pub enum Month { Jan, Feb, Mar, Apr, May, Jun, Jul, Aug, Sep, Oct, Nov, Dec }

#[derive(Clone, PartialEq)]
pub enum Address{
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
