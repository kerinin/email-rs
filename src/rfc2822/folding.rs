use chomp::*;
use bytes::{Bytes, ByteStr};

use rfc2822::obsolete::*;
use rfc2822::primitive::*;
use rfc2822::quoted::*;

// Folding white space
// FWS = ([*WSP CRLF] 1*WSP) / obs-FWS
// NOTE: Removes CRLF, returns any other characters
pub fn fws(i: Input<u8>) -> U8Result<Bytes> {
    let a = |i| {
        option(i, |i| {
            matched_by(i, |i| skip_many(i, wsp)).bind(|i, (v, _)| {
                crlf(i).then(|i| {
                    i.ret(Bytes::from_slice(v))
                })
            })

        }, Bytes::empty()).bind(|i, pre| {
            matched_by(i, |i| skip_many1(i, wsp)).bind(|i, (v, _)| {
                let bs = pre.concat(&Bytes::from_slice(v));

                i.ret(bs)
            })
        })
    };

    or(i, a, obs_fws)
}

// Non white space controls. The rest of the US-ASCII characters not 
// including "(", ")", or "\"
// ctext = NO-WS-CTL / %d33-39 / %d42-91 / %d93-126        
// Consumes matches & returns ()
pub fn ctext(i: Input<u8>) -> U8Result<u8> {
    let a = |i| {
       satisfy(i, |i| (33 <= i && i <= 39) || (42 <= i && i <= 91) || (93 <= i && i <= 126))
    };

    or(i, no_ws_ctl, a)
}

// comment = "(" *([FWS] ccontent) [FWS] ")"
// Consumes matches & returns ()
pub fn comment(i: Input<u8>) -> U8Result<Bytes> {
    token(i, b'(').then(|i| {
        many(i, |i| {
            option(i, fws, Bytes::empty()).bind(|i, ws| {

                // NOTE: it may be worth expanding ccontent out here so we can
                // capture runs of ctext and quoted-pair without coercing each
                // character to a single-element Bytes
                matched_by(i, |i| skip_many1(i, ccontent)).bind(|i, (v, _)| {
                    let bs = ws.concat(&Bytes::from_slice(v));

                    i.ret(bs)
                })
            })

        }).bind(|i, cs: Vec<Bytes>| {
            option(i, fws, Bytes::empty()).bind(|i, ws| {
                token(i, b')').then(|i| {
                    let bs = cs.into_iter().fold(Bytes::from_slice(b"("), |acc, b| acc.concat(&b));
                    
                    i.ret(bs.concat(&ws).concat(&Bytes::from_slice(b")")))
                })
            })
        })
    })
}

// ccontent = ctext / quoted-pair / comment
// Consumes matches & returns ()
pub fn ccontent(i: Input<u8>) -> U8Result<Bytes> {
    let a = |i| {
        ctext(i).bind(|i, c| i.ret(Bytes::from_slice(&[c][..])))
    };

    let b = |i| {
        quoted_pair(i).bind(|i, c| i.ret(Bytes::from_slice(&[c][..])))
    };

    parse!{i; a() <|> b() <|> comment()}
}

fn fws_comment(i: Input<u8>) -> U8Result<Bytes> {
    // println!("fws_comment({:?})", i);
    option(i, fws, Bytes::empty()).bind(|i, ws| {
        // println!("fws_comment.option(fws).then({:?})", i);
        comment(i).bind(|i, c| {
            let bs = ws.concat(&c);

            i.ret(bs)
        })
    })
}

// CFWS = *([FWS] comment) (([FWS] comment) / FWS)
//
// This is tricky for a greedy matcher.
// What's happening here is that *([FWS] comment) is consuming all the instances
// of that pattern, and then the ([FWS] comment) fails
//
// This should be equivalent:
// CFWS = 1*([FWS] comment) [FWS] / (*([FWS] comment) FWS)
//
pub fn cfws(i: Input<u8>) -> U8Result<Bytes> {
    // println!("cfws({:?})", i);

    let repeat = |i| {
        many1(i, fws_comment).bind(|i, cs: Vec<Bytes>| {
            option(i, fws, Bytes::empty()).bind(|i, c| {
                // println!("cfws.repeat");
                let bs = cs.into_iter().fold(Bytes::empty(), |acc, b| acc.concat(&b));

                i.ret(bs.concat(&c))
            })
        })
    };

    let fws_term = |i| {
        many(i, fws_comment).bind(|i, cs: Vec<Bytes>| {
            // println!("cfws.fws_term");
            fws(i).bind(|i, c| {
                let bs = cs.into_iter().fold(Bytes::empty(), |acc, b| acc.concat(&b));

                i.ret(bs.concat(&c))
            })
        })
    };

    or(i, repeat, fws_term)
}

#[test]
fn test_cfws() {
    let i = b"(his account)";
    let msg = parse_only(cfws, i);
    assert!(msg.is_ok());

    // let i = b"\r\n ";
    // let msg = parse_only(cfws, i);
    // assert!(msg.is_ok());
    //
    // let i = b"(comment)\r\n ";
    // let msg = parse_only(cfws, i);
    // assert!(msg.is_ok());

    // let i = b"(comment)(comment)\r\n ";
    // let msg = parse_only(cfws, i);
    // assert!(msg.is_ok());
    //
    // let i = b"(comment)\r\n (comment)\r\n ";
    // let msg = parse_only(cfws, i);
    // assert!(msg.is_ok());
    //
    // let i = b"\r\n (comment)\r\n (comment)\r\n ";
    // let msg = parse_only(cfws, i);
    // assert!(msg.is_ok());
}
