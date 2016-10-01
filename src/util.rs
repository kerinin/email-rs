use std::fmt::Debug;
use std::str::FromStr;

use chomp::types::*;
use chomp::parsers::*;
use chomp::combinators::*;
// use chomp::combinators::bounded;

fn is_digit(c: u8) -> bool {
    48 <= c && c <= 57
}

pub fn parse_digits<I: U8Input, R, T>(i: I, range: R) -> SimpleResult<I, T> where
T: FromStr,
R: bounded::BoundedRange + Debug,
{
    matched_by(i, |i| { 
        bounded::skip_many(i, range, |i| satisfy(i, is_digit))
    }).bind(|i, (buf, _)| {
        match String::from_utf8_lossy(&buf.into_vec()).parse::<T>() {
            Ok(n) => i.ret(n),
            Err(_) => i.err(Error::unexpected()),
        }
    })
}

