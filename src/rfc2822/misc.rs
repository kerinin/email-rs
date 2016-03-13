use chomp::*;
use bytes::{Bytes, ByteStr};

use rfc2822::atom::*;
use rfc2822::folding::*;
use rfc2822::obsolete::*;
use rfc2822::primitive::*;
use rfc2822::quoted::*;

// word = atom / quoted-string
pub fn word(i: Input<u8>) -> U8Result<Bytes> {
    // println!("word({:?})", i);
    or(i, atom, quoted_string)
}

pub fn word_not<P>(i: Input<u8>, p: P) -> U8Result<Bytes> where
P: FnMut(u8) -> bool,
{
    or(i, atom, |i| quoted_string_not(i, p))
}

// phrase = 1*word / obs-phrase
pub fn phrase(i: Input<u8>) -> U8Result<Bytes> {
    let a = |i| {
        many1(i, word).map(|ws: Vec<Bytes>| {
            println!("phrase.many1(word).map({:?})", ws);

            ws.into_iter().fold(Bytes::empty(), |acc, r| acc.concat(&r))
        })
    };

    or(i, a, obs_phrase)
}

#[test]
fn test_phrase() {
    let i = b"Joe Q. Public";
    let msg = parse_only(phrase, i);
    assert!(msg.is_ok());
}


// utext           =       NO-WS-CTL /     ; Non white space controls
//                         %d33-126 /      ; The rest of US-ASCII
//                         obs-utext
pub fn utext(i: Input<u8>) -> U8Result<u8> {
    or(i,
       no_ws_ctl,
       |i| or(i,
              |i| satisfy(i, |i| (33 <= i && i <= 126)),
              obs_text, // technically this is obs-utext, but it's an alias so whatevs
             ))
}

// unstructured = *([FWS] utext) [FWS]
// NOTE: allowing runs of utext to reduce allocations, so effectively
// unstructured = *([FWS] 1*utext) [FWS]
pub fn unstructured(i: Input<u8>) -> U8Result<Bytes> {
    let a = |i| {
        option(i, fws, ()).then(|i| {
            matched_by(i, |i| skip_many1(i, utext)).bind(|i, (v, _)| {
                i.ret(Bytes::from_slice(v))
            })
        })
    };

    many(i, a).bind(|i, rs: Vec<Bytes>| {
        option(i, fws, ()).then(|i| {
            let bs = rs.into_iter().fold(Bytes::empty(), |acc, r| acc.concat(&r));

            i.ret(bs)
        })
    })
}
