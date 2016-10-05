use std::fmt::Debug;
use std::str::FromStr;

use chomp::*;
use bytes::{Bytes, ByteStr};

use chomp::types::*;
use chomp::parsers::*;
use chomp::combinators::*;
use chomp::primitives::Primitives;
use chomp::primitives::IntoInner;
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

// Matches all items while ``f`` returns true until ``next`` matches.
// Considered incomplete if ``f`` returns false without ``next`` matching.  
// Returns the buffer of matched items and the result of ``next``
pub fn lazy_take_while<I, F, N, T>(mut i: I, mut f: F, mut next: N) -> SimpleResult<I, (I::Buffer, T)> where
I: U8Input + Debug,
F: FnMut(I::Token) -> bool,
N: FnMut(I) -> SimpleResult<I, T>,
{
    let buf_start = i.mark();
    let mut buf_end = i.mark();

    loop {
        let t = i.pop();
        match t {
            Some(token) => {
                if f(token) {
                    buf_end = i.mark();
                    match next(i).into_inner() {
                        (mut inner, Ok(t)) => {
                            let final_mark = inner.mark();
                            inner = inner.restore(buf_end);
                            let buf = inner.consume_from(buf_start);
                            inner = inner.restore(final_mark);

                            return inner.ret((buf, t))
                        },
                        (inner, Err(_)) => {
                            i = inner;
                            continue
                        }
                    }
                } else {
                    return i.err(Error::unexpected())
                }
            },
            None => {
                return i.err(Error::unexpected())
            },
        }

    }
}

#[test]
fn test_lazy_take_while() {
    let i = b"abc";
    let parser = |i| {
        lazy_take_while(i, |_| true, |i| string(i, b"bc")).then(|i| eof(i))
    };
    let msg = parse_only(parser, i);
    println!("parsed: {:?}", msg);
    assert!(msg.is_ok());

    let i = b"abcde";
    let parser = |i| {
        lazy_take_while(i, |_| true, |i| string(i, b"bc"))
    };
    let msg = parse_only(parser, i);
    println!("parsed: {:?}", msg);
    assert!(msg.is_ok());
    assert_eq!(msg.unwrap(), (&b"a"[..], &b"bc"[..]));
    
    let i = b"aaa";
    let parser = |i| {
        lazy_take_while(i, |_| true, |i| string(i, b"bc")).then(|i| eof(i))
    };
    let msg = parse_only(parser, i);
    println!("parsed: {:?}", msg);
    assert!(!msg.is_ok());

    let i = b"";
    let parser = |i| {
        lazy_take_while(i, |_| true, |i| string(i, b"bc")).then(|i| eof(i))
    };
    let msg = parse_only(parser, i);
    println!("parsed: {:?}", msg);
    assert!(!msg.is_ok());
}
