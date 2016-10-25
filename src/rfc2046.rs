//! RFC2046 provides initial media type definitions
//! https://tools.ietf.org/html/rfc2046

use chomp::types::*;
use chomp::parsers::*;
use chomp::combinators::*;
use chomp::primitives::Primitives;
// use chomp::combinators::bounded;

use super::rfc5322::*;

// boundary := 0*69<bchars> bcharsnospace
// bchars := bcharsnospace / " "
pub fn boundary<I: U8Input>(mut i: I, token: String) -> SimpleResult<I, ()> {
    let mut mark = None;
    let mut consumed: u8 = 0;

    loop {
        if consumed == 60 {
            break
        }

        let maybe_t = i.pop();

        if maybe_t.is_none() {
            break
        }

        let t = maybe_t.unwrap();

        if BCHARSNOSPACE[t as usize] {
            mark = Some(i.mark());
        } else if t != b' ' {
            break
        }

        consumed += 1;
    }

    match mark {
        Some(m) => i.restore(m).ret(()),
        None => i.err(Error::unexpected()),
    }
}

// bcharsnospace := DIGIT / ALPHA / "'" / "(" / ")" /
//                  "+" / "_" / "," / "-" / "." /
//                  "/" / ":" / "=" / "?"
// ALPHA          =  %d65-90 / %d97-122  ; A-Z / a-z
// DIGIT          =  %d48-57
//                =    %d34 \       ; "
//                     %d39 \       ; '
//                     %d40 \       ; (
//                     %d41 \       ; )
//                     %d43 \       ; +
//                     %d44 \       ; ,
//                     %d45 \       ; -
//                     %d46 \       ; .
//                     %d47 \       ; /
//                     %d58 \       ; :
//                     %d61 \       ; =
//                     %d63 \       ; ?
//                     %d95 \       ; _
const BCHARSNOSPACE: [bool; 256] = [
    //  0      1      2      3      4      5      6      7      8      9     10     11     12     13     14     15     16     17     18     19
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, //   0 -  19
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,  false, false, false, false, true,  //  20 -  39
    true,  true,  false, true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false, //  40 -  59
    false, true,  false, true,  false, true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  //  60 -  79
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false, false, false, false, true,  false, true,  true,  true,  //  80 -  99
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  // 100 - 119
    true,  true,  true,  false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 120 - 139
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 140 - 159
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 160 - 179
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 180 - 199
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 200 - 219
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 220 - 239
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false                              // 240 - 256
];

// body-part := <"message" as defined in RFC 822, with all
//               header fields optional, not starting with the
//               specified dash-boundary, and with the
//               delimiter not occurring anywhere in the
//               body part.  Note that the semantics of a
//               part differ from the semantics of a message,
//               as described in the text.>
//
// close-delimiter := delimiter "--"
pub fn close_delimiter<I: U8Input>(i: I, token: String) -> SimpleResult<I, ()> {
    delimiter(i, token).then(|i| {
        string(i, b"--").map(|v| ())
    })
}

// dash-boundary := "--" boundary
//                  ; boundary taken from the value of
//                  ; boundary parameter of the
//                  ; Content-Type field.
pub fn dash_boundary<I: U8Input>(i: I, token: String) -> SimpleResult<I, ()> {
    string(i, b"--").then(|i| {
        boundary(i, token).map(|v| ())
    })
}

// delimiter := CRLF dash-boundary
pub fn delimiter<I: U8Input>(i: I, token: String) -> SimpleResult<I, ()> {
    crlf(i).then(|i| {
        dash_boundary(i, token).map(|_| ())
    })
}

// discard-text := *(*text CRLF)
//                 ; May be ignored or discarded.
//
// encapsulation := delimiter transport-padding
//                  CRLF body-part
//
// epilogue := discard-text
//
// multipart-body := [preamble CRLF]
//                   dash-boundary transport-padding CRLF
//                   body-part *encapsulation
//                   close-delimiter transport-padding
//                   [CRLF epilogue]
//
// preamble := discard-text
//
// transport-padding := *LWSP-char
//                      ; Composers MUST NOT generate
//                      ; non-zero length transport
//                      ; padding, but receivers MUST
//                      ; be able to handle padding
//                      ; added by message transports.
