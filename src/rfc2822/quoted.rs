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
const QTEXT: [bool; 256] = [
    //  0      1      2      3      4      5      6      7      8      9     10     11     12     13     14     15     16     17     18     19
    false, true,  true,  true,  true,  true,  true,  true,  true,  false, false, true,  true,  false, true,  true,  true,  true,  true,  true,  //   0 -  19
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false, true,  false, true,  true,  true,  true,  true,  //  20 -  39
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  //  40 -  59
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  //  60 -  79
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false, true,  true,  true,  true,  true,  true,  true,  //  80 -  99
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  // 100 - 119
    true,  true,  true,  true,  true,  true,  true,  true,  false, false, false, false, false, false, false, false, false, false, false, false, // 120 - 139
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 140 - 159
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 160 - 179
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 180 - 199
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 200 - 219
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 220 - 239
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false                              // 240 - 256
];
pub fn qtext(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |c| QTEXT[c as usize])
}

// qcontent = qtext / quoted-pair
pub fn qcontent(i: Input<u8>) -> U8Result<u8> {
    parse!{i;
        qtext() <|> quoted_pair()
    }
}

#[test]
fn test_qcontent() {
    let i = b"G";
    let msg = parse_only(qcontent, i);
    assert!(msg.is_ok());
    assert_eq!(msg.unwrap(), b'G');

    let i = b"\\\"";
    let msg = parse_only(qcontent, i);
    assert!(msg.is_ok());
    assert_eq!(msg.unwrap(), b'\"');
}

// quoted-string   =       [CFWS]
//                         DQUOTE *([FWS] qcontent) [FWS] DQUOTE
//                         [CFWS]
// NOTE: in order to reduce allocations, this checks for runs of qtext 
// explicitly, so expanding things out:
// quoted-string = [CFWS] DQUOTE *([FWS] qcontent) [FWS] DQUOTE [CFWS]
//
// substitute qcontent:  
//               = [CFWS] DQUOTE *([FWS] (qtext / quoted-pair)) [FWS] DQUOTE [CFWS]
//
// associate many
//               = [CFWS] DQUOTE *([FWS] (1*qtext / quoted-pair)) [FWS] DQUOTE [CFWS]
//
pub fn quoted_string(i: Input<u8>) -> U8Result<Bytes> {
    option(i, cfws, Bytes::empty()).bind(|i, ws1| {
        dquote(i).then(|i| {

            let a = |i| {
                option(i, fws, Bytes::empty()).bind(|i, ws2| {
                    or(i,
                       |i| matched_by(i, |i| skip_many1(i, qtext)).map(|(v, _)| Bytes::from_slice(v)),
                       |i| quoted_pair(i).map(|c| Bytes::from_slice(&[c][..])),
                    ).bind(|i, cs| {
                        i.ret(ws2.concat(&cs))
                    })
                })
            };

            many(i, a).bind(|i, rs: Vec<Bytes>| {
                option(i, fws, Bytes::empty()).bind(|i, ws3| {
                    dquote(i).then(|i| {
                        option(i, cfws, Bytes::empty()).bind(|i, ws4| {
                            let bs = rs.into_iter().fold(ws1, |acc, r| acc.concat(&r));

                            i.ret(bs.concat(&ws3).concat(&ws4))
                        })
                    })
                })
            })
        })
    })
}

#[test]
fn test_quoted_string() {
    let i = b"\"Giant; \\\"Big\\\" Box\"";
    let msg = parse_only(quoted_string, i);
    assert!(msg.is_ok());
    assert_eq!(msg.unwrap(), Bytes::from_slice(b"Giant; \"Big\" Box"));
}

pub fn quoted_string_not<P>(i: Input<u8>, mut p: P) -> U8Result<Bytes> where
P: FnMut(u8) -> bool,
{
    option(i, cfws, Bytes::empty()).bind(|i, ws1| {
        dquote(i).then(|i| {
            many1(i, |i| {
                option(i, fws, Bytes::empty()).bind(|i, ws2| {
                    matched_by(i, |i| {
                        peek_next(i).bind(|i, next| {
                            if p(next) {
                                i.err(Error::Unexpected)
                            } else {
                                qcontent(i)
                            }
                        })

                    }).bind(|i, (v, _)| {
                        i.ret(ws2.concat(&Bytes::from_slice(v)))
                    })
                })
            }).bind(|i, rs: Vec<Bytes>| {
                option(i, fws, Bytes::empty()).bind(|i, ws3| {
                    dquote(i).then(|i| {
                        option(i, cfws, Bytes::empty()).bind(|i, ws4| {
                            let bs = rs.into_iter().fold(ws1, |acc, r| acc.concat(&r));

                            i.ret(bs.concat(&ws3).concat(&ws4))
                        })
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
