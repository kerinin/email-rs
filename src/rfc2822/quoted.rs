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
pub fn quoted_string(i: Input<u8>) -> U8Result<Vec<u8>> {
    parse!{i;
        option(cfws, ());
        dquote();
        let c = many(|i| parse!{i; option(fws, ()) >> qcontent()});
        option(fws, ());
        dquote();

        ret c
    }
}

pub fn quoted_string_not<P>(i: Input<u8>, p: P) -> U8Result<Vec<u8>> where
P: Fn(u8) -> bool,
{
    parse!{i;
        option(cfws, ());
        dquote();
        let c = many(|i| {
            option(i, fws, ()).then(|i| {
                peek_next(i).bind(|i, next| {
                    if !p(next) {
                        qcontent(i)
                    } else {
                        i.err(Error::Unexpected)
                    }
                })
            })
        });
        option(fws, ());
        dquote();

        ret c
    }
}

#[test]
fn test_quoted_string_not() {
    let i = b"jdoe";
    let msg = parse_only(|i| quoted_string_not(i, |c| c == b'@'), i);
    assert!(msg.is_ok());

    let i = b"jdoe@example.com";
    let msg = parse_only(|i| quoted_string_not(i, |c| c == b'@'), i);
    assert!(msg.is_err());
}
