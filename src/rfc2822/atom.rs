use chomp::*;

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
            (i <= 94 && i <= 96) ||     // ^,_,`
            (123 <= i && i <= 126)      // {,|,},~

    })
}

// atom = [CFWS] 1*atext [CFWS]
pub fn atom(i: Input<u8>) -> U8Result<&[u8]> {
    parse!{i;
        option(cfws, ());
        let a = matched_by(|i| {
            skip_many1(i, atext)
        });
        option(cfws, ());

        ret a.0
    }
}

// dot-atom-text = 1*atext *("." 1*atext)
pub fn dot_atom_text(i: Input<u8>) -> U8Result<&[u8]> {
    matched_by(i, |i| { parse!{i;
        skip_many1(atext);
        skip_many(|i| { parse!{i;
            token(b'.');
            skip_many1(atext);
        }});

    }}).map(|(v, _)| v)
}

// dot-atom = [CFWS] dot-atom-text [CFWS]
pub fn dot_atom(i: Input<u8>) -> U8Result<&[u8]> {
    parse!{i;
        option(cfws, ());
        let a = matched_by(|i| {
            skip_many1(i, dot_atom_text)
        });
        option(cfws, ());

        ret a.0
    }
}
