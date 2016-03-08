use chomp::*;

use rfc2822::atom::*;
use rfc2822::folding::*;
use rfc2822::obsolete::*;
use rfc2822::primitive::*;
use rfc2822::quoted::*;

// word = atom / quoted-string
pub fn word(i: Input<u8>) -> U8Result<Vec<u8>> {
    or(i,
       |i| {
           atom(i).map(|i| {
               let mut v = Vec::with_capacity(i.len());
               v.extend(i);
               v
           })
       },
       quoted_string,
       )
}

// phrase = 1*word / obs-phrase
pub fn phrase(i: Input<u8>) -> U8Result<Vec<u8>> {
    or(i,
       |i| { parse!{i;
           let wv: Vec<Vec<u8>> = many1(word);

           ret wv.into_iter().flat_map(|i| i).collect()
       }},
       obs_phrase,
       )
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
