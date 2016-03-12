use chomp::*;

use rfc2822::atom::*;
use rfc2822::folding::*;
use rfc2822::obsolete::*;
use rfc2822::primitive::*;
use rfc2822::quoted::*;

// word = atom / quoted-string
pub fn word(i: Input<u8>) -> U8Result<Vec<u8>> {
    println!("word({:?})", i);
    or(i,
       |i| {
           atom(i).map(|i| {
               println!("word.atom.map({:?})", i);
               let mut v = Vec::with_capacity(i.len());
               v.extend(i);
               v
           })
       },
       quoted_string,
       )
}

pub fn word_not<P>(i: Input<u8>, p: P) -> U8Result<Vec<u8>> where
P: FnMut(u8) -> bool,
{
    or(i,
       |i| {
           atom(i).map(|i| {
               let mut v = Vec::with_capacity(i.len());
               v.extend(i);
               v
           })
       },
       |i| quoted_string_not(i, p),
       )
}

// phrase = 1*word / obs-phrase
pub fn phrase(i: Input<u8>) -> U8Result<Vec<u8>> {
    let a = |i| {
        many1(i, word).map(|ws: Vec<Vec<u8>>| {
            println!("phrase.many1(word).map({:?})", ws);

            ws.into_iter().flat_map(|i| i).collect()
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
pub fn unstructured(i: Input<u8>) -> U8Result<Vec<u8>> {
    parse!{i;
        let t = many(|i| { parse!{i;
            option(fws, ());
            utext()
        }});
        option(fws, ());

        ret t
    }
}
