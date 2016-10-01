//! RFC5322 specifies message bodies (supercedes RFC2822)

use bytes::{Bytes, ByteStr};

use chomp::types::*;
use chomp::parsers::*;
use chomp::combinators::*;

use super::*;

// ALPHA          =  %x41-5A / %x61-7A   ; A-Z / a-z
// ALPHA          =  %d65-90 / %d97-122  ; A-Z / a-z
const ALPHA: [bool; 256] = [
    //  0      1      2      3      4      5      6      7      8      9     10     11     12     13     14     15     16     17     18     19
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, //   0 -  19
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, //  20 -  39
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, //  40 -  59
    false, false, false, false, false, true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  //  60 -  79
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false, false, false, false, false, false, true,  true,  true,  //  80 -  99
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  // 100 - 119
    true,  true,  true,  false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 120 - 139
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 140 - 159
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 160 - 179
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 180 - 199
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 200 - 219
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 220 - 239
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false                              // 240 - 256
];
pub fn alpha<I: U8Input>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |c| ALPHA[c as usize])
}

// BIT            =  "0" / "1"
pub fn bit<I: U8Input>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |i| i == 0 || i == 1)
}

// CHAR           =  %x01-7F
//                        ; any 7-bit US-ASCII character,
//                        ;  excluding NUL
//
// CR             =  %x0D
// CR             =  %d13
//                        ; carriage return
pub fn cr<I: U8Input>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |i| i == 13)
}

// CRLF           =  CR LF
//                        ; Internet standard newline
pub fn crlf<I: U8Input>(i: I) -> SimpleResult<I, Bytes> {
    cr(i).bind(|i, v_cr| {
        lf(i).bind(|i, v_lf| {
            i.ret(Bytes::from_slice(&vec!(v_cr, v_lf)))
        })
    })
}

// CTL            =  %x00-1F / %x7F
//                        ; controls
//
// DIGIT          =  %x30-39
//                        ; 0-9
//
// DQUOTE         =  %x22
// DQUOTE         =  %d34
//                        ; " (Double Quote)
pub fn dquote<I: U8Input>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |i| i == 34)
}
//
// HEXDIG         =  DIGIT / "A" / "B" / "C" / "D" / "E" / "F"
//
// HTAB           =  %x09
// HTAB           =  %d09
//                        ; horizontal tab
pub fn htab<I: U8Input>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |i| i == 9)
}
//
// LF             =  %x0A
// LF             =  %d10
//                        ; linefeed
pub fn lf<I: U8Input>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |i| i == 10)
}

// LWSP           =  *(WSP / CRLF WSP)
//                        ; Use of this linear-white-space rule
//                        ;  permits lines containing only white
//                        ;  space that are no longer legal in
//                        ;  mail headers and have caused
//                        ;  interoperability problems in other
//                        ;  contexts.
//                        ; Do not use when defining mail
//                        ;  headers and use with caution in
//                        ;  other contexts.
//
// OCTET          =  %x00-FF
//                        ; 8 bits of data
//
// SP             =  %x20
// SP             =  %d32
pub fn sp<I: U8Input>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |i| i == 32)
}

// VCHAR          =  %x21-7E
// VCHAR          =  %d33-126
//                        ; visible (printing) characters
const VCHAR: [bool; 256] = [
    //  0      1      2      3      4      5      6      7      8      9     10     11     12     13     14     15     16     17     18     19
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, //   0 -  19
    false, false, false, false, false, false, false, false, false, false, false, false, false, true,  true,  true,  true,  true,  true,  true,  //  20 -  39
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  //  40 -  59
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  //  60 -  79
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  //  80 -  99
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  // 100 - 119
    true,  true,  true,  true,  true,  true,  true,  false, false, false, false, false, false, false, false, false, false, false, false, false, // 120 - 139
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 140 - 159
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 160 - 179
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 180 - 199
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 200 - 219
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 220 - 239
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false                              // 240 - 256
];
pub fn vchar<I: U8Input>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |c| VCHAR[c as usize])
}

// WSP            =  SP / HTAB
//						  ; white space
pub fn wsp<I: U8Input>(i: I) -> SimpleResult<I, u8> {
    or(i, sp, htab)
}

// quoted-pair     =   ("\" (VCHAR / WSP)) / obs-qp
pub fn quoted_pair<I: U8Input>(i: I) -> SimpleResult<I, u8> {
    or(i, 
       |i| {
           token(i, b'\\').then(|i| {
               or(i, vchar, wsp)
           })
       },
       obs_qp)
}

// FWS             =   ([*WSP CRLF] 1*WSP) /  obs-FWS
//                                        ; Folding white space
pub fn fws<I: U8Input>(i: I) -> SimpleResult<I, ()> {
    or(i, 
       |i| {
           option(i, |i| {
               skip_many(i, wsp).then(crlf).map(|_| ())
           }, ()).then(|i| {
               skip_many1(i, wsp)
           })
       },
       obs_fws)
}

// ctext           =   %d33-39 /          ; Printable US-ASCII
//                     %d42-91 /          ;  characters not including
//                     %d93-126 /         ;  "(", ")", or "\"
//                     obs-ctext
const CTEXT: [bool; 256] = [
    //  0      1      2      3      4      5      6      7      8      9     10     11     12     13     14     15     16     17     18     19
    false, true,  true,  true,  true,  true,  true,  true,  true,  false, false, true,  true,  false, true,  true,  true,  true,  true,  true,  //   0 -  19
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false, true,  true,  true,  true,  true,  true,  true,  //  20 -  39
    false, false, true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  //  40 -  59
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  //  60 -  79
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false, true,  true,  true,  true,  true,  true,  true,  //  80 -  99
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  // 100 - 119
    true,  true,  true,  true,  true,  true,  true,  true,  false, false, false, false, false, false, false, false, false, false, false, false, // 120 - 139
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 140 - 159
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 160 - 179
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 180 - 199
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 200 - 219
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 220 - 239
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false                              // 240 - 256
];
pub fn ctext<I: U8Input>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |c| CTEXT[c as usize])
}

// ccontent        =   ctext / quoted-pair / comment
// NOTE: This _seems_ like it's going to create a loop
pub fn ccontent<I: U8Input>(i: I) -> SimpleResult<I, ()> {
    or(i,
       |i| ctext(i).map(|_| ()),
       |i| or(i,
              |i| quoted_pair(i).map(|_| ()),
              comment))
}

// comment         =   "(" *([FWS] ccontent) [FWS] ")"
pub fn comment<I: U8Input>(i: I) -> SimpleResult<I, ()> {
    token(i, b'(').then(|i| {
        skip_many(i, |i| {
            option(i, fws, ()).then(ccontent)
        }).then(|i| {
            option(i, fws, ())
        })
    }).then(|i| {
        token(i, b')').map(|_| ())
    })
}

// CFWS            =   (1*([FWS] comment) [FWS]) / FWS
pub fn cfws<I: U8Input>(i: I) -> SimpleResult<I, ()> {
    or(i,
       |i| {
           skip_many1(i, |i| {
               option(i, fws, ()).then(comment)
           }).then(|i| {
               option(i, fws, ()).map(|_| ())
           })
       },
       fws)
}

// atext           =   ALPHA / DIGIT /    ; Printable US-ASCII
//                     "!" / "#" /        ;  characters not including
//                     "$" / "%" /        ;  specials.  Used for atoms.
//                     "&" / "'" /
//                     "*" / "+" /
//                     "-" / "/" /
//                     "=" / "?" /
//                     "^" / "_" /
//                     "`" / "{" /
//                     "|" / "}" /
//                     "~"
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
pub fn atext<I: U8Input>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |c| ATEXT[c as usize])
}

// atom            =   [CFWS] 1*atext [CFWS]
pub fn atom<I: U8Input>(i: I) -> SimpleResult<I, Bytes> {
    option(i, cfws, ()).then(|i| {
        matched_by(i, |i| {
            skip_many1(i, atext)
        }).map(|(buf, _)| Bytes::from_slice(&buf.into_vec()))
    }).bind(|i, buf| {
        option(i, cfws, ()).bind(|i, _| {
            i.ret(buf)
        })
    })
}

// dot-atom-text   =   1*atext *("." 1*atext)
pub fn dot_atom_text<I: U8Input>(i: I) -> SimpleResult<I, Bytes> {
    matched_by(i, |i| {
        skip_many1(i, atext).then(|i| {
            skip_many1(i, |i| {
                token(i, b'.').then(|i| {
                    skip_many1(i, atext)
                })
            })
        })
    }).map(|(buf, _)| Bytes::from_slice(&buf.into_vec()))
}

// dot-atom        =   [CFWS] dot-atom-text [CFWS]
pub fn dot_atom<I: U8Input>(i: I) -> SimpleResult<I, Bytes> {
    option(i, cfws, ()).then(|i| {
        dot_atom_text(i).bind(|i, v| {
            option(i, cfws, ()).then(|i| {
                i.ret(v)
            })
        })
    })
}

// specials        =   "(" / ")" /        ; Special characters that do
//                     "<" / ">" /        ;  not appear in atext
//                     "[" / "]" /
//                     ":" / ";" /
//                     "@" / "\" /
//                     "," / "." /
//                     DQUOTE
//
// qtext           =   %d33 /             ; Printable US-ASCII
//                     %d35-91 /          ;  characters not including
//                     %d93-126 /         ;  "\" or the quote character
//                     obs-qtext
const QTEXT: [bool; 256] = [
    //  0      1      2      3      4      5      6      7      8      9     10     11     12     13     14     15     16     17     18     19
    false, true,  true,  true,  true,  true,  true,  true,  true,  false, false, true,  true,  false, true,  true,  true,  true,  true,  true,  //   0 -  19
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false, true,  false, true,  true,  true,  true,  true,  //  20 -  39
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  //  40 -  59
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  //  60 -  79
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false, true,  true,  true,  true,  true,  true,  true,  //  80 -  99
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  // 100 - 119
    true,  true,  true,  true,  true,  true,  true,  true,  false, false, false, false, false, false, false, false, false, false, false, false, // 120 - 139
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 140 - 159
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 160 - 179
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 180 - 199
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 200 - 219
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 220 - 239
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false                              // 240 - 256
];
pub fn qtext<I: U8Input>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |c| QTEXT[c as usize])
}

// qcontent        =   qtext / quoted-pair
pub fn qcontent<I: U8Input>(i: I) -> SimpleResult<I, u8> {
    or(i, qtext, quoted_pair)
}

// quoted-string   =   [CFWS]
//                     DQUOTE *([FWS] qcontent) [FWS] DQUOTE
//                     [CFWS]
pub fn quoted_string<I: U8Input>(i: I) -> SimpleResult<I, Bytes> {
    option(i, cfws, ()).then(|i| {
        dquote(i).then(|i| {
            many(i, |i| {
                option(i, fws, ()).then(|i| {
                    // NOTE: Take advantage of the buffer
                    matched_by(i, |i| {
                        skip_many1(i, qcontent)
                    }).map(|(buf, _)| Bytes::from_slice(&buf.into_vec()))
                })
            }).map(|bufs: Vec<Bytes>| {
                bufs.into_iter().fold(Bytes::empty(), |l, r| l.concat(&r))
            }).bind(|i, buf| {
                option(i, fws, ()).then(|i| {
                    dquote(i).then(|i| {
                        i.ret(buf)
                    })
                })
            })
        })
    })
}

// word            =   atom / quoted-string
pub fn word<I: U8Input>(i: I) -> SimpleResult<I, Bytes> {
    or(i, atom, quoted_string)
}
//
// phrase          =   1*word / obs-phrase
pub fn phrase<I: U8Input>(i: I) -> SimpleResult<I, Bytes> {
    or(i,
       |i| {
           many1(i, word).map(|bufs: Vec<Bytes>| {
               bufs.into_iter().fold(Bytes::empty(), |l, r| l.concat(&r))
           })
       },
       obs_phrase)
}

// unstructured    =   (*([FWS] VCHAR) *WSP) / obs-unstruct
// TODO: parse new version
pub fn unstructured<I: U8Input>(i: I) -> SimpleResult<I, Bytes> {
    obs_unstruct(i)
}

// date-time       =   [ day-of-week "," ] date time [CFWS]
//
// day-of-week     =   ([FWS] day-name) / obs-day-of-week
//
// day-name        =   "Mon" / "Tue" / "Wed" / "Thu" /
//                     "Fri" / "Sat" / "Sun"
//
// date            =   day month year
//
// day             =   ([FWS] 1*2DIGIT FWS) / obs-day
//
// month           =   "Jan" / "Feb" / "Mar" / "Apr" /
//                     "May" / "Jun" / "Jul" / "Aug" /
//                     "Sep" / "Oct" / "Nov" / "Dec"
//
// year            =   (FWS 4*DIGIT FWS) / obs-year
//
// time            =   time-of-day zone
//
// time-of-day     =   hour ":" minute [ ":" second ]
//
// hour            =   2DIGIT / obs-hour
//
// minute          =   2DIGIT / obs-minute
//
// second          =   2DIGIT / obs-second
//
// zone            =   (FWS ( "+" / "-" ) 4DIGIT) / obs-zone
//
// address         =   mailbox / group
//
// mailbox         =   name-addr / addr-spec
pub fn mailbox<I: U8Input>(i: I) -> SimpleResult<I, Address> {
    or(i,
       |i| name_addr(i).map(|(local_part, domain, maybe_display_name)| {
           Address::Mailbox{
               local_part: unsafe { String::from_utf8_unchecked(local_part.buf().bytes().to_vec()) },
               domain: unsafe { String::from_utf8_unchecked(domain.buf().bytes().to_vec()) },
               display_name: maybe_display_name,
           }
       }),
       |i| addr_spec(i).map(|(local_part, domain)| {
           Address::Mailbox{
               local_part: unsafe { String::from_utf8_unchecked(local_part.buf().bytes().to_vec()) },
               domain: unsafe { String::from_utf8_unchecked(domain.buf().bytes().to_vec()) },
               display_name: None,
           }
       }))
}

// name-addr       =   [display-name] angle-addr
pub fn name_addr<I: U8Input>(i: I) -> SimpleResult<I, (Bytes, Bytes, Option<Bytes>)> {
    option(i, |i| {
        display_name(i).map(|n| Some(n))
    }, None).bind(|i, n| {
        angle_addr(i).bind(|i, (l, d)| {
            i.ret((l, d, n))
        })
    })
}

// angle-addr      =   [CFWS] "<" addr-spec ">" [CFWS] /
//                     obs-angle-addr
// NOTE: Not implementing obs-angle-addr because "routing" is bs
pub fn angle_addr<I: U8Input>(i: I) -> SimpleResult<I, (Bytes, Bytes)> {
    option(i, cfws, ()).then(|i| {
        token(i, b'<').then(|i| {
            addr_spec(i).bind(|i, (l, d)| {
                token(i, b'>').then(|i| {
                    option(i, cfws, ()).then(|i| {
                        i.ret((l, d))
                    })
                })
            })
        })
    })
}

// group           =   display-name ":" [group-list] ";" [CFWS]
//
// display-name    =   phrase
pub fn display_name<I: U8Input>(i: I) -> SimpleResult<I, Bytes> {
    phrase(i)
}

// mailbox-list    =   (mailbox *("," mailbox)) / obs-mbox-list
//
// address-list    =   (address *("," address)) / obs-addr-list
//
// group-list      =   mailbox-list / CFWS / obs-group-list
//
// addr-spec       =   local-part "@" domain
pub fn addr_spec<I: U8Input>(i: I) -> SimpleResult<I, (Bytes, Bytes)> {
    local_part(i).bind(|i, l| {
        token(i, b'@').then(|i| {
            domain(i).bind(|i, d| {
                i.ret((l, d))
            })
        })
    })
}

// local-part      =   dot-atom / quoted-string / obs-local-part
pub fn local_part<I: U8Input>(i: I) -> SimpleResult<I, Bytes> {
    or(i,
       dot_atom,
       |i| or(i,
              quoted_string,
              obs_local_part))
}

// domain          =   dot-atom / domain-literal / obs-domain
// TODO: Support new fields
pub fn domain<I: U8Input>(i: I) -> SimpleResult<I, Bytes> {
    obs_domain(i)
}

// domain-literal  =   [CFWS] "[" *([FWS] dtext) [FWS] "]" [CFWS]
//
// dtext           =   %d33-90 /          ; Printable US-ASCII
//                     %d94-126 /         ;  characters not including
//                     obs-dtext          ;  "[", "]", or "\"
//
// message         =   (fields / obs-fields)
//                     [CRLF body]
// TODO: Support new fields
pub fn message<I: U8Input>(i: I) -> SimpleResult<I, Message> {
    obs_fields(i).bind(|i, f| {
        option(i, |i| {
            crlf(i).then(|i| {
                body(i)
            })
        }, Bytes::empty()).bind(|i, b| {
            let message = Message {
                fields: f,
                body: b,
            };
            i.ret(message)
        })
    })
}

// body            =   (*(*998text CRLF) *998text) / obs-body
// TODO: support new fields
pub fn body<I: U8Input>(i: I) -> SimpleResult<I, Bytes> {
    obs_body(i)
}

// text            =   %d1-9 /            ; Characters excluding CR
//                     %d11 /             ;  and LF
//                     %d12 /
//                     %d14-127
const TEXT: [bool; 256] = [
    //  0      1      2      3      4      5      6      7      8      9     10     11     12     13     14     15     16     17     18     19
    false, true,  true,  true,  true,  true,  true,  true,  true,  true,  false, true,  true,  false, true,  true,  true,  true,  true,  true,  //   0 -  19
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  //  20 -  39
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  //  40 -  59
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  //  60 -  79
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  //  80 -  99
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  // 100 - 119
    true,  true,  true,  true,  true,  true,  true,  true,  false, false, false, false, false, false, false, false, false, false, false, false, // 120 - 139
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 140 - 159
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 160 - 179
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 180 - 199
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 200 - 219
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 220 - 239
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false                              // 240 - 256
];
pub fn text<I: U8Input>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |c| TEXT[c as usize])
}

// fields          =   *(trace
//                       *optional-field /
//                       *(resent-date /
//                        resent-from /
//                        resent-sender /
//                        resent-to /
//                        resent-cc /
//                        resent-bcc /
//                        resent-msg-id))
//                     *(orig-date /
//                     from /
//                     sender /
//                     reply-to /
//                     to /
//                     cc /
//                     bcc /
//                     message-id /
//                     in-reply-to /
//                     references /
//                     subject /
//                     comments /
//                     keywords /
//                     optional-field)
//
// +----------------+--------+------------+----------------------------+
// | Field          | Min    | Max number | Notes                      |
// |                | number |            |                            |
// +----------------+--------+------------+----------------------------+
// | trace          | 0      | unlimited  | Block prepended - see      |
// |                |        |            | 3.6.7                      |
// | resent-date    | 0*     | unlimited* | One per block, required if |
// |                |        |            | other resent fields are    |
// |                |        |            | present - see 3.6.6        |
// | resent-from    | 0      | unlimited* | One per block - see 3.6.6  |
// | resent-sender  | 0*     | unlimited* | One per block, MUST occur  |
// |                |        |            | with multi-address         |
// |                |        |            | resent-from - see 3.6.6    |
// | resent-to      | 0      | unlimited* | One per block - see 3.6.6  |
// | resent-cc      | 0      | unlimited* | One per block - see 3.6.6  |
// | resent-bcc     | 0      | unlimited* | One per block - see 3.6.6  |
// | resent-msg-id  | 0      | unlimited* | One per block - see 3.6.6  |
// | orig-date      | 1      | 1          |                            |
// | from           | 1      | 1          | See sender and 3.6.2       |
// | sender         | 0*     | 1          | MUST occur with            |
// |                |        |            | multi-address from - see   |
// |                |        |            | 3.6.2                      |
// | reply-to       | 0      | 1          |                            |
// | to             | 0      | 1          |                            |
// | cc             | 0      | 1          |                            |
// | bcc            | 0      | 1          |                            |
// | message-id     | 0*     | 1          | SHOULD be present - see    |
// |                |        |            | 3.6.4                      |
// | in-reply-to    | 0*     | 1          | SHOULD occur in some       |
// |                |        |            | replies - see 3.6.4        |
// | references     | 0*     | 1          | SHOULD occur in some       |
// |                |        |            | replies - see 3.6.4        |
// | subject        | 0      | 1          |                            |
// | comments       | 0      | unlimited  |                            |
// | keywords       | 0      | unlimited  |                            |
// | optional-field | 0      | unlimited  |                            |
// +----------------+--------+------------+----------------------------+
//
// orig-date       =   "Date:" date-time CRLF
//
// from            =   "From:" mailbox-list CRLF
//
// sender          =   "Sender:" mailbox CRLF
//
// reply-to        =   "Reply-To:" address-list CRLF
//
// to              =   "To:" address-list CRLF
//
// cc              =   "Cc:" address-list CRLF
//
// bcc             =   "Bcc:" [address-list / CFWS] CRLF
//
// message-id      =   "Message-ID:" msg-id CRLF
//
// in-reply-to     =   "In-Reply-To:" 1*msg-id CRLF
//
// references      =   "References:" 1*msg-id CRLF
//
// msg-id          =   [CFWS] "<" id-left "@" id-right ">" [CFWS]
//
// id-left         =   dot-atom-text / obs-id-left
//
// id-right        =   dot-atom-text / no-fold-literal / obs-id-right
//
// no-fold-literal =   "[" *dtext "]"
//
// subject         =   "Subject:" unstructured CRLF
//
// comments        =   "Comments:" unstructured CRLF
//
// keywords        =   "Keywords:" phrase *("," phrase) CRLF
//
// resent-date     =   "Resent-Date:" date-time CRLF
//
// resent-from     =   "Resent-From:" mailbox-list CRLF
//
// resent-sender   =   "Resent-Sender:" mailbox CRLF
//
// resent-to       =   "Resent-To:" address-list CRLF
//
// resent-cc       =   "Resent-Cc:" address-list CRLF
//
// resent-bcc      =   "Resent-Bcc:" [address-list / CFWS] CRLF
//
// resent-msg-id   =   "Resent-Message-ID:" msg-id CRLF
//
// trace           =   [return]
//                     1*received
//
// return          =   "Return-Path:" path CRLF
//
// path            =   angle-addr / ([CFWS] "<" [CFWS] ">" [CFWS])
//
// received        =   "Received:" *received-token ";" date-time CRLF
//
// received-token  =   word / angle-addr / addr-spec / domain
//
// optional-field  =   field-name ":" unstructured CRLF
//
// field-name      =   1*ftext
pub fn field_name<I: U8Input>(i: I) -> SimpleResult<I, I::Buffer> {
	matched_by(i, |i| {
        skip_many1(i, ftext)
	}).map(|(buf, ())| {
        buf
    })
}

// ftext           =   %d33-57 /          ; Printable US-ASCII
//                     %d59-126           ;  characters not including
//                                        ;  ":".
const FTEXT: [bool; 256] = [
    //  0      1      2      3      4      5      6      7      8      9     10     11     12     13     14     15     16     17     18     19
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, //   0 -  19
    false, false, false, false, false, false, false, false, false, false, false, false, false, true,  true,  true,  true,  true,  true,  true,  //  20 -  39
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false, true,  //  40 -  59
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  //  60 -  79
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  //  80 -  99
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  // 100 - 119
    true,  true,  true,  true,  true,  true,  true,  false, false, false, false, false, false, false, false, false, false, false, false, false, // 120 - 139
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 140 - 159
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 160 - 179
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 180 - 199
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 200 - 219
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 220 - 239
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false                              // 240 - 256
];
pub fn ftext<I: U8Input>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |c| FTEXT[c as usize])
}

// obs-NO-WS-CTL   =   %d1-8 /            ; US-ASCII control
//                     %d11 /             ;  characters that do not
//                     %d12 /             ;  include the carriage
//                     %d14-31 /          ;  return, line feed, and
//                     %d127              ;  white space characters
const OBS_NO_WS_CTL: [bool; 256] = [
    //  0      1      2      3      4      5      6      7      8      9     10     11     12     13     14     15     16     17     18     19
    false, true,  true,  true,  true,  true,  true,  true,  true,  false, false, true,  true,  false, true,  true,  true,  true,  true,  true,  //   0 -  19
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false, false, false, false, false, false, false, false, //  20 -  39
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, //  40 -  59
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, //  60 -  79
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, //  80 -  99
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 100 - 119
    false, false, false, false, false, false, false, true,  false, false, false, false, false, false, false, false, false, false, false, false, // 120 - 139
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 140 - 159
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 160 - 179
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 180 - 199
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 200 - 219
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 220 - 239
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false                              // 240 - 256
];
pub fn obs_no_ws_ctl<I: U8Input>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |c| OBS_NO_WS_CTL[c as usize])
}

// obs-ctext       =   obs-NO-WS-CTL
pub fn obs_ctext<I: U8Input>(i: I) -> SimpleResult<I, u8> {
	obs_no_ws_ctl(i)
}

// obs-qtext       =   obs-NO-WS-CTL
pub fn obs_qtext<I: U8Input>(i: I) -> SimpleResult<I, u8> {
	obs_no_ws_ctl(i)
}

// obs-utext       =   %d0 / obs-NO-WS-CTL / VCHAR
pub fn obs_utext<I: U8Input>(i: I) -> SimpleResult<I, u8> {
	or(i, 
       |i| satisfy(i, |i| i == 0),
       |i| or(i, obs_no_ws_ctl, vchar))
}

// obs-qp          =   "\" (%d0 / obs-NO-WS-CTL / LF / CR)
// Where any quoted-pair appears, it is to be interpreted as the
// character alone.  That is to say, the "\" character that appears as
// part of a quoted-pair is semantically "invisible".
pub fn obs_qp<I: U8Input>(i: I) -> SimpleResult<I, u8> {
    token(i, b'\\').then(|i| {
        or(i,
           |i| satisfy(i, |i| i == 0),
           |i| or(i,
                  obs_no_ws_ctl,
                  |i| or(i,
                         lf,
                         cr)))
    })
}

// obs-body        =   *((*LF *CR *((%d0 / text) *LF *CR)) / CRLF)
// NOTE: Since all of these variants are optional/repeated fields, the only
// real constraint is that we need to drop CFWS
pub fn obs_body<I: U8Input>(i: I) -> SimpleResult<I, Bytes> {
    many(i, |i| {
        let a = |i| {
            matched_by(i, |i| {
                skip_many(i, lf).then(|i| {
                    skip_many(i, cr).then(|i| {
                        skip_many(i, |i| token(i, b'0')).then(|i| {
                            skip_many(i, text)
                        })
                    })
                })
            }).map(|(buf, ()): (I::Buffer, ())| {
                Bytes::from_slice(&buf.into_vec())
            })
        };

        or(i, a, crlf)

    }).map(|bufs: Vec<Bytes>| {
        bufs.into_iter().fold(Bytes::empty(), |l, r| l.concat(&r))
    })
}

// obs-unstruct    =   *((*LF *CR *(obs-utext *LF *CR)) / FWS)
// NOTE: Since all of these variants are optional/repeated fields, the only
// real constraint is that we need to drop FWS
pub fn obs_unstruct<I: U8Input>(i: I) -> SimpleResult<I, Bytes> {
    many(i, |i| {
        let a = |i| {
            matched_by(i, |i| {
                skip_many(i, lf).then(|i| {
                    skip_many(i, cr).then(|i| {
                        skip_many(i, obs_utext)
                    })
                })
            }).map(|(buf, ()): (I::Buffer, ())| {
                let slice = buf.into_vec();
                Bytes::from_slice(&slice)
            })
        };

        let b = |i| {
            fws(i).map(|_| Bytes::empty())
        };

        or(i, a, b)

    }).map(|bufs: Vec<Bytes>| {
        bufs.into_iter().fold(Bytes::empty(), |l, r| l.concat(&r))
    })
}

// obs-phrase      =   word *(word / "." / CFWS)
pub fn obs_phrase<I: U8Input>(i: I) -> SimpleResult<I, Bytes> {
    word(i).bind(|i, w1| {
        many(i, |i| {
            or(i,
               word,
               |i| or(i,
                      |i| token(i, b'.').map(|_| Bytes::from_slice(&[b'.'])),
                      |i| cfws(i).map(|_| Bytes::empty())))
        }).map(|bufs: Vec<Bytes>| {
            bufs.into_iter().fold(w1, |l, r| l.concat(&r))
        })
    })
}


// obs-phrase-list =   [phrase / CFWS] *("," [phrase / CFWS])
pub fn obs_phrase_list<I: U8Input>(i: I) -> SimpleResult<I, Vec<Bytes>> {
    option(i, |i| {
        or(i,
           |i| phrase(i).map(|buf| Some(buf)),
           |i| cfws(i).map(|_| None))
    }, None).bind(|i, option_phrase1| {
        many(i, |i| {
            token(i, b',').then(|i| {
                option(i, |i| {
                    or(i,
                       |i| phrase(i).map(|buf| Some(buf)),
                       |i| cfws(i).map(|_| None))
                }, None)
            })
        }).map(|bufs: Vec<Option<Bytes>>| {
            // NOTE: Assume worst-case scenario (no cfws parsed)
            let mut init = Vec::with_capacity(bufs.len()+1);
            if option_phrase1.is_some() {
                init.push(option_phrase1.unwrap())
            }

            bufs.into_iter().fold(init, |mut l, r| {
                match r {
                    Some(buf) => {l.push(buf)},
                    None => {},
                }
                l
            })
        })
    })
}

// obs-FWS         =   1*WSP *(CRLF 1*WSP)
pub fn obs_fws<I: U8Input>(i: I) -> SimpleResult<I, ()> {
    skip_many1(i, wsp).then(|i| {
        skip_many(i, |i| {
            crlf(i).then(|i| {
                skip_many1(i, wsp)
            })
        })
    })
}

// obs-day-of-week =   [CFWS] day-name [CFWS]
//
// obs-day         =   [CFWS] 1*2DIGIT [CFWS]
//
// obs-year        =   [CFWS] 2*DIGIT [CFWS]
//
// obs-hour        =   [CFWS] 2DIGIT [CFWS]
//
// obs-minute      =   [CFWS] 2DIGIT [CFWS]
//
// obs-second      =   [CFWS] 2DIGIT [CFWS]
//
// obs-zone        =   "UT" / "GMT" /     ; Universal Time
//                                        ; North American UT
//                                        ; offsets
//                     "EST" / "EDT" /    ; Eastern:  - 5/ - 4
//                     "CST" / "CDT" /    ; Central:  - 6/ - 5
//                     "MST" / "MDT" /    ; Mountain: - 7/ - 6
//                     "PST" / "PDT" /    ; Pacific:  - 8/ - 7
//                                        ;
//                     %d65-73 /          ; Military zones - "A"
//                     %d75-90 /          ; through "I" and "K"
//                     %d97-105 /         ; through "Z", both
//                     %d107-122          ; upper and lower case
//
//    EDT is semantically equivalent to -0400
//    EST is semantically equivalent to -0500
//    CDT is semantically equivalent to -0500
//    CST is semantically equivalent to -0600
//    MDT is semantically equivalent to -0600
//    MST is semantically equivalent to -0700
//    PDT is semantically equivalent to -0700
//    PST is semantically equivalent to -0800
//
// obs-angle-addr  =   [CFWS] "<" obs-route addr-spec ">" [CFWS]
// NOTE: Not supporting because obs-route is stupid

// obs-route       =   obs-domain-list ":"
// NOTE: Not supporting because why, even?

// obs-domain-list =   *(CFWS / ",") "@" domain
//                     *("," [CFWS] ["@" domain])
pub fn obs_domain_list<I: U8Input>(i: I) -> SimpleResult<I, Vec<Bytes>> {
    skip_many(i, |i| {
        or(i, cfws, |i| token(i, b',').map(|_| ()))
    }).then(|i| {
        token(i, b'@').then(|i| {
            domain(i).bind(|i, domain1| {
                many(i, |i| {
                    token(i, b',').then(|i| {
                        option(i, cfws, ()).then(|i| {
                            option(i, |i| {
                                token(i, b'@').then(|i| {
                                    domain(i).map(|d| Some(d))
                                })
                            }, None)
                        })
                    })
                }).map(|bufs: Vec<Option<Bytes>>| {
                    let mut domains = Vec::with_capacity(bufs.len()+1);
                    domains.push(domain1);

                    bufs.into_iter().fold(domains, |mut l, r| {
                        if r.is_some() {
                            l.push(r.unwrap())
                        }
                        l
                    })
                })
            })
        })
    })
}

// obs-mbox-list   =   *([CFWS] ",") mailbox *("," [mailbox / CFWS])
//
// obs-addr-list   =   *([CFWS] ",") address *("," [address / CFWS])
//
// obs-group-list  =   1*([CFWS] ",") [CFWS]
//
// obs-local-part  =   word *("." word)
pub fn obs_local_part<I: U8Input>(i: I) -> SimpleResult<I, Bytes> {
    word(i).bind(|i, w1| {
        many(i, |i| {
            token(i, b'.').bind(|i, tok| {
                word(i).map(|buf| Bytes::from_slice(&[tok]).concat(&buf))
            })
        }).map(|bufs: Vec<Bytes>| {
            bufs.into_iter().fold(w1, |l, r| l.concat(&r))
        })
    })
}

// obs-domain      =   atom *("." atom)
pub fn obs_domain<I: U8Input>(i: I) -> SimpleResult<I, Bytes> {
    atom(i).bind(|i, a1| {
        many(i, |i| {
            token(i, b'.').bind(|i, d_n| {
                atom(i).bind(|i, a_n| {
                    i.ret(Bytes::from_slice(&[d_n]).concat(&a_n))
                })
            })
        }).map(|bufs: Vec<Bytes>| {
            bufs.into_iter().fold(a1, |l, r| l.concat(&r))
        })
    })
}

// obs-dtext       =   obs-NO-WS-CTL / quoted-pair
//
// obs-fields      =   *(obs-return /
//                     obs-received /
//                     obs-orig-date /
//                     obs-from /
//                     obs-sender /
//                     obs-reply-to /
//                     obs-to /
//                     obs-cc /
//                     obs-bcc /
//                     obs-message-id /
//                     obs-in-reply-to /
//                     obs-references /
//                     obs-subject /
//                     obs-comments /
//                     obs-keywords /
//                     obs-resent-date /
//                     obs-resent-from /
//                     obs-resent-send /
//                     obs-resent-rply /
//                     obs-resent-to /
//                     obs-resent-cc /
//                     obs-resent-bcc /
//                     obs-resent-mid /
//                     obs-optional)
// TODO: Parse actual fields
pub fn obs_fields<I: U8Input>(i: I) -> SimpleResult<I, Vec<Field>> {
    // NOTE: REALLY wish the parser macro worked right about here
    many(i, |i| {
        or(i, 
           obs_subject,
           |i| or(i,
              obs_comments,
              obs_optional))
    })
}

// obs-orig-date   =   "Date" *WSP ":" date-time CRLF
//
// obs-from        =   "From" *WSP ":" mailbox-list CRLF
//
// obs-sender      =   "Sender" *WSP ":" mailbox CRLF
//
// obs-reply-to    =   "Reply-To" *WSP ":" address-list CRLF
//
// obs-to          =   "To" *WSP ":" address-list CRLF
//
// obs-cc          =   "Cc" *WSP ":" address-list CRLF
//
// obs-bcc         =   "Bcc" *WSP ":"
//                     (address-list / (*([CFWS] ",") [CFWS])) CRLF
//
// obs-message-id  =   "Message-ID" *WSP ":" msg-id CRLF
//
// obs-in-reply-to =   "In-Reply-To" *WSP ":" *(phrase / msg-id) CRLF
//
// obs-references  =   "References" *WSP ":" *(phrase / msg-id) CRLF
//
// obs-id-left     =   local-part
//
// obs-id-right    =   domain
//
// obs-subject     =   "Subject" *WSP ":" unstructured CRLF
pub fn obs_subject<I: U8Input>(i: I) -> SimpleResult<I, Field> {
    string(i, b"Subject").then(|i| {
        option(i, wsp, 0).then(|i| {
            token(i, b':').then(|i| {
                unstructured(i).bind(|i, v| {
                    crlf(i).then(|i| {
                        let value = UnstructuredField {data: v};

                        i.ret(Field::Subject(value))
                    })
                })
            })
        })
    })
}

// obs-comments    =   "Comments" *WSP ":" unstructured CRLF
pub fn obs_comments<I: U8Input>(i: I) -> SimpleResult<I, Field> {
    string(i, b"Comments").then(|i| {
        option(i, wsp, 0).then(|i| {
            token(i, b':').then(|i| {
                unstructured(i).bind(|i, v| {
                    crlf(i).then(|i| {
                        let value = UnstructuredField {data: v};

                        i.ret(Field::Comments(value))
                    })
                })
            })
        })
    })
}

// obs-keywords    =   "Keywords" *WSP ":" obs-phrase-list CRLF
//
// obs-resent-from =   "Resent-From" *WSP ":" mailbox-list CRLF
//
// obs-resent-send =   "Resent-Sender" *WSP ":" mailbox CRLF
//
// obs-resent-date =   "Resent-Date" *WSP ":" date-time CRLF
//
// obs-resent-to   =   "Resent-To" *WSP ":" address-list CRLF
//
// obs-resent-cc   =   "Resent-Cc" *WSP ":" address-list CRLF
//
// obs-resent-bcc  =   "Resent-Bcc" *WSP ":"
//                     (address-list / (*([CFWS] ",") [CFWS])) CRLF
//
// obs-resent-mid  =   "Resent-Message-ID" *WSP ":" msg-id CRLF
//
// obs-resent-rply =   "Resent-Reply-To" *WSP ":" address-list CRLF
//
// obs-return      =   "Return-Path" *WSP ":" path CRLF
//
// obs-received    =   "Received" *WSP ":" *received-token CRLF
//
// obs-optional    =   field-name *WSP ":" unstructured CRLF
pub fn obs_optional<I: U8Input>(i: I) -> SimpleResult<I, Field> {
    field_name(i).bind(|i, n| {
        option(i, wsp, 0).then(|i| {
            token(i, b':').then(|i| {
                unstructured(i).bind(|i, v| {
                    crlf(i).then(|i| {
                        // NOTE: `field-name` is "printable US-ASCII characters not including ':'"
                        let name = unsafe { String::from_utf8_unchecked(n.into_vec()) };
                        let value = UnstructuredField {data: v};

                        i.ret(Field::Optional(name, value))
                    })
                })
            })
        })
    })
}
