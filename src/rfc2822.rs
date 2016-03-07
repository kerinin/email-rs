//! RFC2822 specifies message bodies (supercedes RFC822)

use std::marker::PhantomData;
use std::iter::FromIterator;

use chomp::*;

fn is_cr(c: u8) -> bool {
    c == 13
}

fn is_lf(c: u8) -> bool {
    c == 10
}

fn no_ws_ctl(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |c| (1 <= c && c <= 8) || (c == 11) || (c == 12) || (14 <= c && c <= 31) || (c == 127))
}

fn is_obs_char(c: u8) -> bool {
    (c <= 9) || (c == 11) || (c == 12) || (14 <= c && c <= 127)
}

/*
#[test]
fn test_obs_char() {
assert_eq!(parse_only(obs_char, "1".as_bytes()), Ok('1' as u8));
assert_eq!(parse_only(obs_char, &[10][..]), Err(ParseError::Error(&[10][..], Error::Unexpected)));
}
*/

fn obs_text(i: Input<u8>) -> U8Result<&[u8]> {
    matched_by(i, |i| {
        parse!{i;
            skip_many(|i| satisfy(i, is_lf));
            skip_many(|i| satisfy(i, is_cr));
            skip_many(|i| parse!{i;
                skip_many1(|i| satisfy(i, is_obs_char));
                skip_many(|i| satisfy(i, is_lf));
                skip_many(|i| satisfy(i, is_cr));
            })
        }
    }).map(|(v, _)| v)
}

#[test]
fn test_obs_text() {
    assert_eq!(parse_only(obs_text, &[10,10,13,13,1,2,3,10][..]), Ok(&[10,10,13,13,1,2,3,10][..]));
    // assert_eq!(parse_only(obs_char, &[10][..]), Err(ParseError::Error(&[10][..], Error::Unexpected)));
}

/*

   fn text(i: Input<u8>) -> SimpleResult<I, I> {
   parse!{ i;
   or(
   satisfy(|c| (1 <= c && c <= 9) || (c == 11) || (c == 12) || (14 <= c && c <= 127)),
   obs_text,
   )
   }
   }
   */
