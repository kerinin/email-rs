use bytes::{Bytes, ByteStr};
use chomp::*;

use rfc2822::folding::*;
use rfc2822::obsolete::*;
use rfc2822::primitive::*;

// quoted-pair = ("\" text) / obs-qp
// Consumes & returns matches
pub fn quoted_pair(i: Input<u8>) -> U8Result<u8> {
    parse!{i;
        or( 
            |i| parse!{i; token(b'\\') >> text() },
            obs_qp,
            )}
}

/*
#[test]
pub fn test_quoted_pair() {
assert_eq!(parse_only(quoted_pair, "\\\n".as_bytes()), Ok("\n".as_bytes()));
}
*/

// qtext           =       NO-WS-CTL /     ; Non white space controls
//
//                         %d33 /          ; The rest of the US-ASCII
//                         %d35-91 /       ;  characters not including "\"
//                         %d93-126        ;  or the quote character
pub fn qtext(i: Input<u8>) -> U8Result<u8> {
    parse!{i;
        no_ws_ctl() <|> 
            satisfy(|i| (i == 33) || (35 <= i && i <= 91) || (93 <= i && i <= 126))
    }
}

// qcontent = qtext / quoted-pair
pub fn qcontent(i: Input<u8>) -> U8Result<u8> {
    parse!{i;
        qtext() <|> quoted_pair()
    }
}

// quoted-string   =       [CFWS]
//                         DQUOTE *([FWS] qcontent) [FWS] DQUOTE
//                         [CFWS]
// NOTE: in order to reduce allocations, this checks for runs of qcontent 
// explicitly, so effectively
// quoted-string = [CFWS] DQUOTE *([FWS] 1*(qcontent)) [FWS] DQUOTE [CFWS]
pub fn quoted_string(i: Input<u8>) -> U8Result<Bytes> {
    option(i, cfws, ()).then(|i| {
        dquote(i).then(|i| {
            let a = |i| {
                option(i, fws, ()).then(|i| {
                    matched_by(i, |i| skip_many1(i, qcontent)).bind(|i, (v, _)| {
                        i.ret(Bytes::from_slice(v))
                    })
                })
            };

            many(i, a).bind(|i, rs: Vec<Bytes>| {
                option(i, fws, ()).then(|i| {
                    dquote(i).then(|i| {
                        option(i, cfws, ()).then(|i| {
                            let bs = rs.into_iter().fold(Bytes::empty(), |acc, r| acc.concat(&r));

                            i.ret(bs)
                        })
                    })
                })
            })
        })
    })
}

pub fn quoted_string_not<P>(i: Input<u8>, mut p: P) -> U8Result<Bytes> where
P: FnMut(u8) -> bool,
{
    option(i, cfws, ()).then(|i| {
        dquote(i).then(|i| {
            many1(i, |i| {
                option(i, fws, ()).then(|i| {
                    peek_next(i).bind(|i, next| {
                        if p(next) {
                            i.err(Error::Unexpected)
                        } else {
                            qcontent(i).bind(|i, c| {
                                i.ret(c)
                            })
                        }
                    })
                })
            }).bind(|i, cs: Vec<u8>| {
                option(i, fws, ()).then(|i| {
                    dquote(i).then(|i| {
                        i.ret(Bytes::from_slice(&cs[..]))
                    })
                })
            })
        })
    })
}


#[test]
fn test_quoted_string_not() {
    let i = b"\"jdoe\"";
    let msg = parse_only(|i| quoted_string_not(i, |c| c == b'@'), i);
    assert_eq!(msg, Ok(Bytes::from_slice(b"jdoe")));

    let i = b"\"jdoe\"@example.com";
    let msg = parse_only(|i| quoted_string_not(i, |c| c == b'@'), i);
    assert_eq!(msg, Ok(Bytes::from_slice(b"jdoe")));
}
