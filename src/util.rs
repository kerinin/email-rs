use std::str::FromStr;

use chomp::*;
use chomp::combinators::bounded;

fn is_digit(c: u8) -> bool {
    30 <= c && c <= 39
}

pub fn parse_digits<T, R>(i: Input<u8>, range: R) -> SimpleResult<u8, T> where
T: FromStr,
R: bounded::BoundedRange,
{
    matched_by(i, |i| { 
        bounded::skip_many(i, range, |i| satisfy(i, is_digit))
    }).bind(|i, (v, _)| {
        match String::from_utf8_lossy(v).parse::<T>() {
            Ok(n) => i.ret(n),
            Err(_) => i.err(Error::Unexpected),
        }
    })
}

