//! RFC2822 specifies message bodies (supercedes RFC822)

use std::marker::PhantomData;
use std::iter::FromIterator;

use chomp::*;

fn cr(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |i| i == 13)
}

fn lf(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |i| i == 10)
}

fn dquote(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |i| i == 34)
}

fn crlf(i: Input<u8>) -> U8Result<&[u8]> {
    string(i, &[13,10][..])
}

// the space (SP, ASCII value 32) and horizontal tab (HTAB, ASCII value 9) characters
// (together known as the white space characters, WSP)
fn wsp(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |i| (i == 32) || (i == 9))
}
// US-ASCII control characters that do not include the carriage return, 
// line feed, and white space characters
// NO-WS-CTL = %d1-8 / %d11 / %d12 / %d14-31 / %d127
fn no_ws_ctl(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |i| (1 <= i && i <= 8) || (i == 11) || (i == 12) || (14 <= i && i <= 31) || (i == 127))
}

// %d0-127 except CR and LF
// obs-char = %d0-9 / %d11 / %d12 / %d14-127
fn obs_char(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |i| (i <= 9) || (i == 11) || (i == 12) || (14 <= i && i <= 127))
}

#[test]
fn test_obs_char() {
    assert_eq!(parse_only(obs_char, "1".as_bytes()), Ok('1' as u8));
    assert_eq!(parse_only(obs_char, &[10][..]), Err(ParseError::Error(&[10][..], Error::Unexpected)));
}

// obs-text = *LF *CR *(obs-char *LF *CR)
//
// NOTE: I think this is a flaw in the spec - it leaks
// matches to *(%0-9 / %11 / %12 / %14-127).  This matcher eliminates the 
// obs-char repeat
fn obs_text(i: Input<u8>) -> U8Result<u8> {
    parse!{i;
        skip_many(|i| lf(i));
        skip_many(|i| cr(i));
        let c = obs_char();
        skip_many(|i| lf(i));
        skip_many(|i| cr(i));

        ret c
    }
}

// Characters excluding CR and LF
// text = %d1-9 / %d11 / %d12 / %d14-127 / obs-text 
fn text(i: Input<u8>) -> U8Result<u8> {
    parse!{i;
        or( 
            |i| satisfy(i, |c| (1 <= c && c <= 9) || (c == 11) || (c == 12) || (14 <= c && c <= 127)),
            |i| obs_text(i),
            )}
}

// obs-qp = "\" (%d0-127)
// Consumes & returns matches
fn obs_qp(i: Input<u8>) -> U8Result<u8> {
    parse!{i;
        token(b'\\');
        satisfy(|i| i <= 127)
    }
}

// quoted-pair = ("\" text) / obs-qp
// Consumes & returns matches
fn quoted_pair(i: Input<u8>) -> U8Result<u8> {
    parse!{i;
        or( 
            |i| parse!{i; token(b'\\') >> text() },
            |i| obs_qp(i),
            )}
}

#[test]
fn test_quoted_pair() {
    assert_eq!(parse_only(quoted_pair, "\\\n".as_bytes()), Ok("\n".as_bytes()));
}

// obs-FWS = 1*WSP *(CRLF 1*WSP)
// Consumes matches & returns ()
fn obs_fws(i: Input<u8>) -> U8Result<()> {
    parse!{i;
        skip_many1(|i| wsp(i));
        skip_many(|i| parse!{i;
            crlf();
            skip_many1(|i| wsp(i));
        })
    }
}

// Folding white space
// FWS = ([*WSP CRLF] 1*WSP) / obs-FWS
// Consumes matches & returns ()
fn fws(i: Input<u8>) -> U8Result<()> {
    or(i,
       |i| { parse!{i;
           option(|i| { parse!{i;
               skip_many(|i| wsp(i));
               crlf();

               ret ()
           }}, ());
           skip_many1(|i| wsp(i));
       }},
       |i| obs_fws(i),
       )
}

// Non white space controls. The rest of the US-ASCII characters not 
// including "(", ")", or "\"
// ctext = NO-WS-CTL / %d33-39 / %d42-91 / %d93-126        
// Consumes matches & returns ()
fn ctext(i: Input<u8>) -> U8Result<()> {
    or(i,
       |i| no_ws_ctl(i).then(|i| i.ret(())),
       |i| satisfy(i, |i| (33 <= i && i <= 39) || (42 <= i && i <= 91) || (93 <= i && i <= 126)).then(|i| i.ret(())),
       )
}

// comment = "(" *([FWS] ccontent) [FWS] ")"
// Consumes matches & returns ()
fn comment(i: Input<u8>) -> U8Result<()> {
    parse!{i;
        token(b'(');
        skip_many(|i| { parse!{i;
            option(|i| fws(i), ());
            ccontent()
        }} );
        option(|i| fws(i), ());
        token(b')');

        ret ()
    }
}

// ccontent = ctext / quoted-pair / comment
// Consumes matches & returns ()
fn ccontent(i: Input<u8>) -> U8Result<()> {
    or(i, 
       |i| ctext(i).then(|i| i.ret(())),
       |i| {
           or(i,
              |i| quoted_pair(i).then(|i| i.ret(())),
              |i| comment(i),
              )
       },)
}

// CFWS = *([FWS] comment) (([FWS] comment) / FWS)
fn cfws(i: Input<u8>) -> U8Result<()> {
    parse!{i;
        skip_many(|i| { parse!{i;
            option(fws, ());
            comment();
        }});

        or(
            |i| { parse!{i;
                option(fws, ());
                comment();
            }},
            |i| fws(i)
          )
    }
}

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
fn atext(i: Input<u8>) -> U8Result<u8> {
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
fn atom(i: Input<u8>) -> U8Result<&[u8]> {
    parse!{i;
        option(|i| cfws(i), ());
        let a = matched_by(|i| {
            skip_many1(i, |i| atext(i))
        });
        option(|i| cfws(i), ());

        ret a.0
    }
}

// dot-atom-text = 1*atext *("." 1*atext)
fn dot_atom_text(i: Input<u8>) -> U8Result<&[u8]> {
    matched_by(i, |i| { parse!{i;
        skip_many1(|i| atext(i));
        skip_many(|i| { parse!{i;
            token(b'.');
            skip_many1(|i| atext(i));
        }});

    }}).map(|(v, _)| v)
}

// dot-atom = [CFWS] dot-atom-text [CFWS]
fn dot_atom(i: Input<u8>) -> U8Result<&[u8]> {
    parse!{i;
        option(|i| cfws(i), ());
        let a = matched_by(|i| {
            skip_many1(i, |i| dot_atom_text(i))
        });
        option(|i| cfws(i), ());

        ret a.0
    }
}


// qtext           =       NO-WS-CTL /     ; Non white space controls
//
//                         %d33 /          ; The rest of the US-ASCII
//                         %d35-91 /       ;  characters not including "\"
//                         %d93-126        ;  or the quote character
fn qtext(i: Input<u8>) -> U8Result<u8> {
    parse!{i;
        no_ws_ctl() <|> 
            satisfy(|i| (i == 33) || (35 <= i && i <= 91) || (93 <= i && i <= 126))
    }
}

// qcontent = qtext / quoted-pair
fn qcontent(i: Input<u8>) -> U8Result<u8> {
    parse!{i;
        qtext() <|> quoted_pair()
    }
}

// quoted-string   =       [CFWS]
//                         DQUOTE *([FWS] qcontent) [FWS] DQUOTE
//                         [CFWS]
fn quoted_string(i: Input<u8>) -> U8Result<Vec<u8>> {
    parse!{i;
        option(|i| cfws(i), ());
        dquote();
        let c = many(|i| parse!{i; option(|i| fws(i), ()) >> qcontent()});
        option(|i| fws(i), ());
        dquote();

        ret c
    }
}

// word = atom / quoted-string
fn word(i: Input<u8>) -> U8Result<Vec<u8>> {
    or(i,
       |i| {
           atom(i).map(|i| {
               let mut v = Vec::with_capacity(i.len());
               v.extend(i);
               v
           })
       },
       |i| quoted_string(i),
       )
}

// obs-phrase = word *(word / "." / CFWS)
fn obs_phrase(i: Input<u8>) -> U8Result<Vec<u8>> {
    let r = parse!{i;
        let w1: Vec<u8> = word();
        let wv: Vec<Vec<u8>> = many(|i| {
            or(i,
               |i| word(i),
               |i| or(i,
                      |i| token(i, b'.').map(|_| vec!(b'.')),
                      |i| cfws(i).map(|_| vec!()),
                      )
              )
        });

        ret (w1, wv)
    };

    r.map(|(w1, wv)| {
        wv.into_iter().fold(w1, |mut acc, mut wn| {
            acc.append(&mut wn);
            acc
        })
    })
}

// phrase = 1*word / obs-phrase
fn phrase(i: Input<u8>) -> U8Result<Vec<u8>> {
    or(i,
       |i| { parse!{i;
           let wv: Vec<Vec<u8>> = many1(|i| word(i));

           ret wv.into_iter().flat_map(|i| i).collect()
       }},
       |i| obs_phrase(i),
       )
}


// utext           =       NO-WS-CTL /     ; Non white space controls
//                         %d33-126 /      ; The rest of US-ASCII
//                         obs-utext
fn utext(i: Input<u8>) -> U8Result<u8> {
    or(i,
       |i| no_ws_ctl(i),
       |i| or(i,
              |i| satisfy(i, |i| (33 <= i && i <= 126)),
              |i| obs_text(i), // technically this is obs-utext, but it's an alias so whatevs
             ))
}

// unstructured = *([FWS] utext) [FWS]
fn unstructured(i: Input<u8>) -> U8Result<Vec<u8>> {
    parse!{i;
        let t = many(|i| { parse!{i;
            option(fws, ());
            utext()
        }});
        option(fws, ());

        ret t
    }
}
