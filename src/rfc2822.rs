//! RFC2822 specifies message bodies (supercedes RFC822)

use std::marker::PhantomData;
use std::iter::FromIterator;

use chomp::*;

fn cr(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |i| i == 13)
}

fn lf(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |i| i == 10)
}

fn no_ws_ctl(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |i| (1 <= i && i <= 8) || (i == 11) || (i == 12) || (14 <= i && i <= 31) || (i == 127))
}

fn obs_char(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |i| (i <= 9) || (i == 11) || (i == 12) || (14 <= i && i <= 127))
}

#[test]
fn test_obs_char() {
    assert_eq!(parse_only(obs_char, "1".as_bytes()), Ok('1' as u8));
    assert_eq!(parse_only(obs_char, &[10][..]), Err(ParseError::Error(&[10][..], Error::Unexpected)));
}

fn obs_text(i: Input<u8>) -> U8Result<&[u8]> {
    matched_by(i, |i| {
        parse!{i;
            skip_many(|i| lf(i));
            skip_many(|i| cr(i));
            skip_many(|i| parse!{i;
                skip_many1(|i| obs_char(i));
                skip_many(|i| lf(i));
                skip_many(|i| cr(i));
            })
        }
    }).map(|(v, _)| v)
}

#[test]
fn test_obs_text() {
    assert_eq!(parse_only(obs_text, &[10,10,13,13,1,2,3,10,10,13,13,1,2,3][..]), Ok(&[10,10,13,13,1,2,3,10,10,13,13,1,2,3][..]));
}

// NOTE: I think this is a flaw in the spec - the `obs_text` alternate leaks
// matches to *(%0-9 / %11 / %12 / %14-127).  This parser should probably yield
// U8Result<u8>
fn text(i: Input<u8>) -> U8Result<&[u8]> {
    matched_by(i, |i| {
        parse!{i;
            or( 
                |i| { parse!{i;
                    satisfy(|c| (1 <= c && c <= 9) || (c == 11) || (c == 12) || (14 <= c && c <= 127));
                    take(1)
                }},
                |i| obs_text(i)
              )
        }
    }).map(|(v, _)| v)
}

fn obs_qp(i: Input<u8>) -> U8Result<u8> {
    parse!{i;
        token(b'\\');
        satisfy(|i| i <= 127)
    }
}

fn quoted_pair(i: Input<u8>) -> U8Result<&[u8]> {
    parse!{i;
        or( 
            |i| { parse!{i;
                token(b'\\');
                text()
            }},
            |i| { parse!{i;
                obs_qp();
                take(1)
            }},
            )
    }
}

#[test]
fn test_quoted_pair() {
    assert_eq!(parse_only(quoted_pair, "\\\n".as_bytes()), Ok("\n".as_bytes()));
}
