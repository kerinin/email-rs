use chomp::*;
use bytes::Bytes;

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
pub fn atext(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |i| {
        (48 <= i && i <= 57) ||         // digit
            (65 <= i && i <= 90) ||     // uppercase
            (97 <= i && i <= 122) ||    // lowercase
            i == 33 ||                  // !
            (35 <= i && i <= 39) ||     // #,$,%,&,'
            i == 42 ||                  // *
            i == 43 ||                  // +
            i == 45 ||                  // -
            i == 47 ||                  // /
            i == 63 ||                  // ?
            (94 <= i && i <= 96) ||     // ^,_,`
            (123 <= i && i <= 126)      // {,|,},~

    })
}

// atom = [CFWS] 1*atext [CFWS]
pub fn atom(i: Input<u8>) -> U8Result<Bytes> {
    option(i, cfws, ()).then(|i| {
        matched_by(i, |i| skip_many1(i, atext)).bind(|i, (v, _)| {
            option(i, cfws, ()).then(|i| {
                i.ret(Bytes::from_slice(v))
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
    option(i, cfws, ()).then(|i| {
        matched_by(i, |i| {
            skip_many1(i, dot_atom_text)
        }).bind(|i, (v, _)| {
            option(i, cfws, ()).then(|i| {
                i.ret(Bytes::from_slice(v))
            })
        })
    })
}
