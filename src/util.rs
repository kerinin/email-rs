use std::fmt::Debug;
use std::str::FromStr;

use chomp::*;

use chomp::types::*;
use chomp::parsers::*;
use chomp::combinators::*;
use chomp::primitives::Primitives;
// use chomp::primitives::IntoInner;
// use chomp::combinators::bounded;

pub fn unchecked_string_from_bufs<I: U8Input>(bufs: Vec<I::Buffer>) -> String {
    let len = bufs.iter().fold(0, |l, buf| l + buf.len());
    let mut bytes = Vec::with_capacity(len);
    for buffer in bufs.into_iter() {
        bytes.append(&mut buffer.into_vec());
    }

    unsafe { String::from_utf8_unchecked(bytes) }
}

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

/*
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
*/

// NOTE: This is an identity matrix, except 65-90 (A-Z) are mapped to 97-122 (a-z)
const DOWNCASE_ASCII: [u8; 256] = [
    000, 001, 002, 003, 004, 005, 006, 007, 008, 009, 010, 011, 012, 013, 014, 015, 016, 017, 018, 019, 
    020, 021, 022, 023, 024, 025, 026, 027, 028, 029, 030, 031, 032, 033, 034, 035, 036, 037, 038, 039, 
    040, 041, 042, 043, 044, 045, 046, 047, 048, 049, 050, 051, 052, 053, 054, 055, 056, 057, 058, 059, 
    060, 061, 062, 063, 064, 097, 098, 099, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 
    112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 091, 092, 093, 094, 095, 096, 097, 098, 099, 
    100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 
    120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 
    140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 
    160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 
    180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 
    200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 
    220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 
    240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255
];

// The same as `string` except the comparison is case-insensitive for ascii 
// characters (A == a, for A-Z)
pub fn downcased_string<I: Input<Token=u8>>(mut i: I, s: &[u8]) -> SimpleResult<I, I::Buffer> {
    let mut n = 0;
    let len   = s.len();

    // TODO: There has to be some more efficient way here
    let b = i.consume_while(|c| {
        if n >= len || DOWNCASE_ASCII[c as usize] != DOWNCASE_ASCII[s[n] as usize] {
            false
        }
        else {
            n += 1;

            true
        }
    });

    if n >= len {
        i.ret(b)
    } else {
        i.err(Error::expected(s[n]))
    }
}

#[test]
fn test_downcased_string() {
    let i = b"Reply-to";
    let msg = parse_only(|i| downcased_string(i, b"Reply-To"), i);
    assert!(msg.is_ok());
}
