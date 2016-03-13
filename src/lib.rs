//! Parser/Serializer for emails
#![recursion_limit="1000"]
#[macro_use]
extern crate chomp;
extern crate chrono;
extern crate bytes;

pub mod rfc2822;
mod util;

