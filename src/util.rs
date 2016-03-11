use std::fmt::Debug;
use std::str::FromStr;

use chomp::*;
use chomp::combinators::bounded;

fn is_digit(c: u8) -> bool {
    48 <= c && c <= 57
}

pub fn parse_digits<T, R>(i: Input<u8>, range: R) -> SimpleResult<u8, T> where
T: FromStr,
R: bounded::BoundedRange + Debug,
{
    println!("parse_digits({:?}, {:?})", i, range);
    matched_by(i, |i| { 
        println!("parse_digits.matched_by({:?})", i);
        bounded::skip_many(i, range, |i| satisfy(i, is_digit))
    }).bind(|i, (v, _)| {
        println!("parse_digits.matched_by.bind(|{:?}, ({:?}, _)|)", i, v);
        match String::from_utf8_lossy(v).parse::<T>() {
            Ok(n) => i.ret(n),
            Err(_) => i.err(Error::Unexpected),
        }
    })
}

