use chomp::*;
use bytes::{Bytes, ByteStr};

use rfc2822::folding::*;

// atext           =       ALPHA / DIGIT / ; Any character except controls,
//                         "!" / "#" /     ;  SP, and specials.
//                         "$" / "%" /     ;  Used for atoms
//                         "&" / "'" /
//                         "*" / "+" /
//                         "-" / "/" /
//                         "=" / "?" /
//                         "^" / "_" /
//                         "`" / "{" /
//                         "|" / "}" /
//                         "~"
const ATEXT: [bool; 256] = [
    //  0      1      2      3      4      5      6      7      8      9     10     11     12     13     14     15     16     17     18     19
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, //   0 -  19
    false, false, false, false, false, false, false, false, false, false, false, false, false, true,  false, true,  true,  true,  true,  true,  //  20 -  39
    false, false, true,  true,  false, true,  false, true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false, false, //  40 -  59
    false, false, false, true,  false, true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  //  60 -  79
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false, false, false, true,  true,  true,  true,  true,  true,  //  80 -  99
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  // 100 - 119
    true,  true,  true,  true,  true,  true,  true,  false, false, false, false, false, false, false, false, false, false, false, false, false, // 120 - 139
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 140 - 159
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 160 - 179
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 180 - 199
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 200 - 219
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 220 - 239
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false                              // 240 - 256
];
pub fn atext(i: Input<u8>) -> U8Result<u8> {
    // satisfy(i, |i| {
    //     (48 <= i && i <= 57) ||         // digit
    //         (65 <= i && i <= 90) ||     // uppercase
    //         (97 <= i && i <= 122) ||    // lowercase
    //         i == 33 ||                  // !
    //         (35 <= i && i <= 39) ||     // #,$,%,&,'
    //         i == 42 ||                  // *
    //         i == 43 ||                  // +
    //         i == 45 ||                  // -
    //         i == 47 ||                  // /
    //         i == 63 ||                  // ?
    //         (94 <= i && i <= 96) ||     // ^,_,`
    //         (123 <= i && i <= 126)      // {,|,},~
    //
    // })
    satisfy(i, |c| ATEXT[c as usize])
}

// atom = [CFWS] 1*atext [CFWS]
pub fn atom(i: Input<u8>) -> U8Result<Bytes> {
    option(i, cfws, Bytes::empty()).bind(|i, ws1| {
        matched_by(i, |i| skip_many1(i, atext)).bind(|i, (v, _)| {
            option(i, cfws, Bytes::empty()).bind(|i, ws2| {

                i.ret(ws1.concat(&Bytes::from_slice(v)).concat(&ws2))
            })
        })
    })
}

// dot-atom-text = 1*atext *("." 1*atext)
pub fn dot_atom_text(i: Input<u8>) -> U8Result<Bytes> {
    matched_by(i, |i| {
        skip_many1(i, atext).then(|i| {
            skip_many(i, |i| {
                token(i, b'.').then(|i| {
                    skip_many1(i, atext)
                })
            })
        })
    }).map(|(v, _)| Bytes::from_slice(v))
}

// dot-atom = [CFWS] dot-atom-text [CFWS]
pub fn dot_atom(i: Input<u8>) -> U8Result<Bytes> {
    option(i, cfws, Bytes::empty()).bind(|i, ws1| {
        matched_by(i, |i| {
            skip_many1(i, dot_atom_text)
        }).bind(|i, (v, _)| {
            option(i, cfws, Bytes::empty()).bind(|i, ws2| {

                i.ret(ws1.concat(&Bytes::from_slice(v)).concat(&ws2))
            })
        })
    })
}
