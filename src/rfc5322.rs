//! RFC5322 specifies message bodies (supercedes RFC2822)

use std::fmt::Debug;

use chrono::datetime::DateTime;
use chrono::offset::LocalResult;
use chrono::offset::TimeZone;
use chrono::offset::fixed::FixedOffset;
use chrono::naive::datetime::NaiveDateTime;
use chrono::naive::time::NaiveTime;
use chrono::naive::date::NaiveDate;
use bytes::{Bytes, ByteStr};

use chomp::*;
use chomp::types::*;
use chomp::parsers::*;
use chomp::combinators::*;

use super::*;
use super::util::*;

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
pub fn alpha<I: U8Input + Debug>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |c| ALPHA[c as usize])
}

// BIT            =  "0" / "1"
pub fn bit<I: U8Input + Debug>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |i| i == 0 || i == 1)
}

// CHAR           =  %x01-7F
//                        ; any 7-bit US-ASCII character,
//                        ;  excluding NUL
//
// CR             =  %x0D
// CR             =  %d13
//                        ; carriage return
pub fn cr<I: U8Input + Debug>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |i| i == 13)
}

pub fn many_cr<I: U8Input + Debug>(i: I) -> SimpleResult<I, I::Buffer> {
    matched_by(i, |i| {
        skip_many(i, cr)
    }).map(|(buf, _)| buf)
}

// CRLF           =  CR LF
//                        ; Internet standard newline
pub fn crlf<I: U8Input + Debug>(i: I) -> SimpleResult<I, I::Buffer> {
    matched_by(i, |i| {
        cr(i).then(lf)
    }).map(|(buf, _)| buf)
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
pub fn dquote<I: U8Input + Debug>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |i| i == 34)
}
//
// HEXDIG         =  DIGIT / "A" / "B" / "C" / "D" / "E" / "F"
//
// HTAB           =  %x09
// HTAB           =  %d09
//                        ; horizontal tab
pub fn htab<I: U8Input + Debug>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |i| i == 9)
}
//
// LF             =  %x0A
// LF             =  %d10
//                        ; linefeed
pub fn lf<I: U8Input + Debug>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |i| i == 10)
}

pub fn many_lf<I: U8Input + Debug>(i: I) -> SimpleResult<I, I::Buffer> {
    matched_by(i, |i| {
        skip_many(i, lf)
    }).map(|(buf, _)| buf)
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
pub fn sp<I: U8Input + Debug>(i: I) -> SimpleResult<I, u8> {
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
pub fn vchar<I: U8Input + Debug>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |c| VCHAR[c as usize])
}

// WSP            =  SP / HTAB
//						  ; white space
pub fn wsp<I: U8Input + Debug>(i: I) -> SimpleResult<I, u8> {
    or(i, sp, htab)
}

// quoted-pair     =   ("\" (VCHAR / WSP)) / obs-qp
pub fn quoted_pair<I: U8Input + Debug>(i: I) -> SimpleResult<I, u8> {
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
// TODO: Drop ONLY CRLF
pub fn fws<I: U8Input + Debug>(i: I) -> SimpleResult<I, Bytes> {
    or(i, 
       |i| {
           option(i, |i| {
               matched_by(i, |i| {
                   skip_many(i, wsp)
               }).bind(|i, (buf1, _)| {
                   crlf(i).then(|i| {
                       i.ret(Bytes::from_slice(&buf1.into_vec()))
                   })
               })
           }, Bytes::empty()).bind(|i, buf1| {
               matched_by(i, |i| {
                   skip_many1(i, wsp)
               }).map(|(buf2, _)| {
                   let bytes2 = Bytes::from_slice(&buf2.into_vec());
                   buf1.concat(&bytes2)
               })
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
pub fn ctext<I: U8Input + Debug>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |c| CTEXT[c as usize])
}

// ccontent        =   ctext / quoted-pair / comment
// NOTE: This _seems_ like it's going to create a loop
pub fn ccontent<I: U8Input + Debug>(i: I) -> SimpleResult<I, ()> {
    or(i,
       |i| ctext(i).map(|_| ()),
       |i| or(i,
              |i| quoted_pair(i).map(|_| ()),
              comment))
}

// comment         =   "(" *([FWS] ccontent) [FWS] ")"
pub fn comment<I: U8Input + Debug>(i: I) -> SimpleResult<I, ()> {
    token(i, b'(').then(|i| {
        skip_many(i, |i| {
            option(i, fws, Bytes::empty()).then(ccontent)
        }).then(|i| {
            option(i, fws, Bytes::empty())
        })
    }).then(|i| {
        token(i, b')').map(|_| ())
    })
}

// CFWS            =   (1*([FWS] comment) [FWS]) / FWS
//                 =   ([FWS] 1*(comment [FWS])) / FWS
// TODO: Don't drop all this text
pub fn cfws<I: U8Input + Debug>(i: I) -> SimpleResult<I, Bytes> {
    or(i,
       |i| {
           option(i, fws, Bytes::empty()).bind(|i, buf1| {
               many1(i, |i| {
                   comment(i).then(|i| {
                       option(i, |i| {
                           fws(i).map(|v| Some(v))
                       }, None)
                   })
               }).map(|vs: Vec<Option<Bytes>>| {
                   vs.into_iter().filter(|v| v.is_some()).fold(buf1, |l, r| l.concat(&r.unwrap()))
               })
           })
       },
       fws)
}

pub fn drop_cfws<I: U8Input + Debug>(i: I) -> SimpleResult<I, ()> {
    cfws(i).map(|_| ())
}

#[test]
fn test_cfws() {
    let i = b"(his account)";
    let msg = parse_only(cfws, i);
    assert!(msg.is_ok());

    // let i = b"\r\n ";
    // let msg = parse_only(cfws, i);
    // assert!(msg.is_ok());
    //
    // let i = b"(comment)\r\n ";
    // let msg = parse_only(cfws, i);
    // assert!(msg.is_ok());

    // let i = b"(comment)(comment)\r\n ";
    // let msg = parse_only(cfws, i);
    // assert!(msg.is_ok());
    //
    // let i = b"(comment)\r\n (comment)\r\n ";
    // let msg = parse_only(cfws, i);
    // assert!(msg.is_ok());
    //
    // let i = b"\r\n (comment)\r\n (comment)\r\n ";
    // let msg = parse_only(cfws, i);
    // assert!(msg.is_ok());
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
pub fn atext<I: U8Input + Debug>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |c| ATEXT[c as usize])
}

// atom            =   [CFWS] 1*atext [CFWS]
pub fn atom<I: U8Input + Debug>(i: I) -> SimpleResult<I, Bytes> {
    option(i, cfws, Bytes::empty()).bind(|i, cfws1| {
        matched_by(i, |i| {
            skip_many1(i, atext)
        }).bind(|i, (buf, _)| {
            option(i, cfws, Bytes::empty()).bind(|i, cfws2| {
                let b = Bytes::from_slice(&buf.into_vec());

                i.ret(cfws1.concat(&b).concat(&cfws2))
            })
        })
    })
}

// dot-atom-text   =   1*atext *("." 1*atext)
pub fn dot_atom_text<I: U8Input + Debug>(i: I) -> SimpleResult<I, I::Buffer> {
    matched_by(i, |i| {
        skip_many1(i, atext).then(|i| {
            skip_many1(i, |i| {
                token(i, b'.').then(|i| {
                    skip_many1(i, atext)
                })
            })
        })
    }).map(|(buf, _)| buf)
}

// dot-atom        =   [CFWS] dot-atom-text [CFWS]
pub fn dot_atom<I: U8Input + Debug>(i: I) -> SimpleResult<I, Bytes> {
    option(i, cfws, Bytes::empty()).bind(|i, buf1| {
        dot_atom_text(i).bind(|i, buf| {
            option(i, cfws, Bytes::empty()).bind(|i, buf2| {
                let t = Bytes::from_slice(&buf.into_vec());
                i.ret(buf1.concat(&t).concat(&buf2))
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
pub fn qtext<I: U8Input + Debug>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |c| QTEXT[c as usize])
}

// qcontent        =   qtext / quoted-pair
pub fn qcontent<I: U8Input + Debug>(i: I) -> SimpleResult<I, u8> {
    or(i, qtext, quoted_pair)
}

#[test]
fn test_qcontent() {
    let i = b"G";
    let msg = parse_only(qcontent, i);
    assert!(msg.is_ok());
    assert_eq!(msg.unwrap(), b'G');

    let i = b"\\\"";
    let msg = parse_only(qcontent, i);
    assert!(msg.is_ok());
    assert_eq!(msg.unwrap(), b'\"');
}

// quoted-string   =   [CFWS]
//                     DQUOTE *([FWS] qcontent) [FWS] DQUOTE
//                     [CFWS]
// Semantically, neither the optional CFWS outside of the quote
// characters nor the quote characters themselves are part of the
// quoted-string; the quoted-string is what is contained between the two
// quote characters.
// TODO: ONLY drop as described ^^^
pub fn quoted_string<I: U8Input + Debug>(i: I) -> SimpleResult<I, Bytes> {
    println!("quoted-string: {:?}", i);
    option(i, drop_cfws, ()).then(|i| {
        println!("quoted-string.cfws: {:?}", i);
        dquote(i).then(|i| {
            println!("quoted-string.dquote1: {:?}", i);
            many(i, |i| {
                option(i, fws, Bytes::empty()).then(|i| {
                    println!("quoted-string.fws1: {:?}", i);
                    // NOTE: Take advantage of the buffer
                    matched_by(i, |i| {
                        println!("quoted-string.qcontent: {:?}", i);
                        skip_many1(i, qcontent)
                    }).map(|(buf, _)| Bytes::from_slice(&buf.into_vec()))
                })
            }).map(|bufs: Vec<Bytes>| {
                bufs.into_iter().fold(Bytes::empty(), |l, r| l.concat(&r))
            }).bind(|i, buf| {
                option(i, fws, Bytes::empty()).then(|i| {
                    println!("quoted-string.fws2: {:?}", i);
                    dquote(i).then(|i| {
                        println!("quoted-string.dquote2: {:?}", i);
                        option(i, drop_cfws, ()).then(|i| {
                            println!("quoted-string.dquote2: {:?}", i);
                            i.ret(buf)
                        })
                    })
                })
            })
        })
    })
}

#[test]
fn test_quoted_string() {
    let i = b"\"Giant; \\\"Big\\\" Box\"";
    let msg = parse_only(quoted_string, i);
    assert!(msg.is_ok());
    assert_eq!(msg.unwrap(), Bytes::from_slice(b"Giant; \"Big\" Box"));
}

/*
#[test]
fn test_quoted_string_not() {
    let i = b"\"jdoe\"";
    let msg = parse_only(|i| quoted_string_not(i, |c| c == b'@'), i);
    assert_eq!(msg, Ok(Bytes::from_slice(b"jdoe")));

    let i = b"\"jdoe\"@example.com";
    let msg = parse_only(|i| quoted_string_not(i, |c| c == b'@'), i);
    assert_eq!(msg, Ok(Bytes::from_slice(b"jdoe")));
}
*/

// word            =   atom / quoted-string
pub fn word<I: U8Input + Debug>(i: I) -> SimpleResult<I, Bytes> {
    or(i, atom, quoted_string)
}

#[test]
fn test_word() {
    let i = b"Joe ";
    let msg = parse_only(word, i);
    assert!(msg.is_ok());
    assert_eq!(msg.unwrap(), Bytes::from_slice(b"Joe "));
}

// phrase          =   1*word / obs-phrase
// NOTE: Matching obs-phrase first to avoid early termination on '.'
pub fn phrase<I: U8Input + Debug>(i: I) -> SimpleResult<I, Bytes> {
    or(i,
       obs_phrase,
       |i| {
           many1(i, word).map(|bufs: Vec<Bytes>| {
               bufs.into_iter().fold(Bytes::empty(), |l, r| l.concat(&r))
           })
       })
}

#[test]
fn test_phrase() {
    let i = b"Joe Q. Public";
    let msg = parse_only(phrase, i);
    assert!(msg.is_ok());
    let v = msg.unwrap();
    assert_eq!(v, Bytes::from_slice(b"Joe Q. Public"));
}

// unstructured    =   (*([FWS] VCHAR) *WSP) / obs-unstruct
// TODO: parse new version
pub fn unstructured_crlf<I: U8Input + Debug>(i: I) -> SimpleResult<I, Bytes> {
    println!("unstructured_crlf {:?}", i);
    obs_unstruct_crlf(i)
}

// date-time       =   [ day-of-week "," ] date time [CFWS]
pub fn date_time<I: U8Input + Debug>(i: I) -> SimpleResult<I, DateTime<FixedOffset>> {
    println!("date-time: {:?}", i);
    option(i, |i| {
        day_of_week(i).then(|i| {
            token(i, b',').map(|_| ())
        })
    }, ()).then(|i| {
        println!("date-time.day-of-week: {:?}", i);
        date(i).bind(|i, d| {
            println!("date-time.date: {:?}", i);
            time(i).bind(|i, (t, o)| {
                println!("date-time.time: {:?}", i);
                option(i, drop_cfws, ()).then(|i| {
                    println!("date-time.cfws: {:?}", i);
                    let ndt = NaiveDateTime::new(d, t);

                    match o.from_local_datetime(&ndt) {
                        LocalResult::Single(dt) => i.ret(dt),
                        _ => i.err(Error::unexpected()),
                    }
                })
            })
        })
    })
}

#[test]
fn test_date_time() {
    // let i = b"Fri, 21 Nov 1997 09:55:06 -0600";
    // let msg = parse_only(date_time, i);
    // assert!(msg.is_ok());
    // assert_eq!(msg.unwrap(), FixedOffset::east(-6*3600).ymd(1997, 11, 21).and_hms(9,55,6));

    let i = b"21 Nov 97 09:55:06 GMT";
    let msg = parse_only(date_time, i);
    assert!(msg.is_ok());
    assert_eq!(msg.unwrap(), FixedOffset::west(0).ymd(1997, 11, 21).and_hms(9,55,6));

    let i = b"Thu,\r\n      13\r\n        Feb\r\n          1969\r\n 23:32\r\n               -0330 (Newfoundland Time)\r\n";
    let msg = parse_only(date_time, i);
    assert!(msg.is_ok());
}

// day-of-week     =   ([FWS] day-name) / obs-day-of-week
pub fn day_of_week<I: U8Input + Debug>(i: I) -> SimpleResult<I, Day> {
    or(i, 
       |i| option(i, fws, Bytes::empty()).then(day_name),
       obs_day_of_week)
}

// day-name        =   "Mon" / "Tue" / "Wed" / "Thu" /
//                     "Fri" / "Sat" / "Sun"
pub fn day_name<I: U8Input + Debug>(i: I) -> SimpleResult<I, Day> {
    or(i, |i| string(i, b"Mon").then(|i| i.ret(Day::Mon)),
    |i| or(i, |i| string(i, b"Tue").then(|i| i.ret(Day::Tue)),
    |i| or(i, |i| string(i, b"Wed").then(|i| i.ret(Day::Wed)),
    |i| or(i, |i| string(i, b"Thu").then(|i| i.ret(Day::Thu)),
    |i| or(i, |i| string(i, b"Fri").then(|i| i.ret(Day::Fri)),
    |i| or(i, |i| string(i, b"Sat").then(|i| i.ret(Day::Sat)),
    |i| string(i, b"Sun").then(|i| i.ret(Day::Sun))))))))
}

// date            =   day month year
pub fn date<I: U8Input + Debug>(i: I) -> SimpleResult<I, NaiveDate> {
    day(i).bind(|i, d| {
        month(i).bind(|i, m| {
            year(i).bind(|i, mut y| {
                if y < 100 {
                    y += 1900
                }
                match NaiveDate::from_ymd_opt(y as i32, 1 + (m as u32), d as u32) {
                    Some(nd) => i.ret(nd),
                    None => i.err(Error::unexpected()),
                }
            })
        })
    })
}

#[test]
fn test_date() {
    let i = b"21 Nov 97";
    let msg = parse_only(date, i);
    assert!(msg.is_ok());
    assert_eq!(msg.unwrap(), NaiveDate::from_ymd(1997, 11, 21));
}
 
// day             =   ([FWS] 1*2DIGIT FWS) / obs-day
pub fn day<I: U8Input + Debug>(i: I) -> SimpleResult<I, usize> {
    or(i,
       |i| {
           option(i, fws, Bytes::empty()).then(|i| {
               parse_digits(i, (1..3)).bind(|i, d| {
                   fws(i).then(|i| {
                       i.ret(d)
                   })
               })
           })
       },
       obs_day)
}

#[test]
fn test_day() {
    let i = b" 21 ";
    let msg = parse_only(day, i);
    assert!(msg.is_ok());
}

// month           =   "Jan" / "Feb" / "Mar" / "Apr" /
//                     "May" / "Jun" / "Jul" / "Aug" /
//                     "Sep" / "Oct" / "Nov" / "Dec"
pub fn month<I: U8Input + Debug>(i: I) -> SimpleResult<I, Month> {
    or(i, |i| string(i, b"Jan").then(|i| i.ret(Month::Jan)),
    |i| or(i, |i| string(i, b"Feb").then(|i| i.ret(Month::Feb)),
    |i| or(i, |i| string(i, b"Mar").then(|i| i.ret(Month::Mar)),
    |i| or(i, |i| string(i, b"Apr").then(|i| i.ret(Month::Apr)),
    |i| or(i, |i| string(i, b"May").then(|i| i.ret(Month::May)),
    |i| or(i, |i| string(i, b"Jun").then(|i| i.ret(Month::Jun)),
    |i| or(i, |i| string(i, b"Jul").then(|i| i.ret(Month::Jul)),
    |i| or(i, |i| string(i, b"Aug").then(|i| i.ret(Month::Aug)),
    |i| or(i, |i| string(i, b"Sep").then(|i| i.ret(Month::Sep)),
    |i| or(i, |i| string(i, b"Oct").then(|i| i.ret(Month::Oct)),
    |i| or(i, |i| string(i, b"Nov").then(|i| i.ret(Month::Nov)),
    |i| string(i, b"Dec").then(|i| i.ret(Month::Dec)))))))))))))
}

#[test]
fn test_month() {
    let i = b"Nov";
    let msg = parse_only(month, i);
    assert!(msg.is_ok());
}
 
// year            =   (FWS 4*DIGIT FWS) / obs-year
pub fn year<I: U8Input + Debug>(i: I) -> SimpleResult<I, usize> {
    or(i,
       |i| fws(i).then(|i| parse_digits(i, (4..))),
       obs_year)
}

#[test]
fn test_year() {
    let i = b" 1997 ";
    let msg = parse_only(year, i);
    assert!(msg.is_ok());

    let i = b" 97 ";
    let msg = parse_only(year, i);
    assert!(msg.is_ok());
}

// time            =   time-of-day zone
pub fn time<I: U8Input + Debug>(i: I) -> SimpleResult<I, (NaiveTime, FixedOffset)> {
    println!("time: {:?}", i);
    time_of_day(i).bind(|i, t| {
        println!("time.time-of-day: {:?}", i);
        zone(i).bind(|i, z| {
            println!("time.zone: {:?}", i);
            i.ret((t, z))
        })
    })
}

#[test]
fn test_time() {
    let i = b"09:55:06 -0600";
    let msg = parse_only(time, i);
    assert!(msg.is_ok());
    assert_eq!(msg.unwrap(), (NaiveTime::from_hms(9,55,6), FixedOffset::west(6*3600)));

    let i = b"09:55:06 GMT";
    let msg = parse_only(time, i);
    assert!(msg.is_ok());

    let i = b"23:32\r\n               -0330";
    let msg = parse_only(time, i);
    assert!(msg.is_ok());
}

// time-of-day     =   hour ":" minute [ ":" second ]
pub fn time_of_day<I: U8Input + Debug>(i: I) -> SimpleResult<I, NaiveTime> {
    hour(i).bind(|i, h| {
        token(i, b':').then(|i| {
            minute(i).bind(|i, m| {
                option(i, |i| {
                    token(i, b':').then(|i| second(i))
                }, 0).bind(|i, s| {
                    i.ret(NaiveTime::from_hms(h as u32, m as u32, s as u32))
                })
            })
        })
    })
}

// hour            =   2DIGIT / obs-hour
pub fn hour<I: U8Input + Debug>(i: I) -> SimpleResult<I, usize> {
    or(i,
       |i| parse_digits(i, 2),
       obs_hour)
}

// minute          =   2DIGIT / obs-minute
pub fn minute<I: U8Input + Debug>(i: I) -> SimpleResult<I, usize> {
    or(i,
       |i| parse_digits(i, 2),
       obs_minute)
}

// second          =   2DIGIT / obs-second
pub fn second<I: U8Input + Debug>(i: I) -> SimpleResult<I, usize> {
    or(i,
       |i| parse_digits(i, 2),
       obs_second)
}

// zone            =   (FWS ( "+" / "-" ) 4DIGIT) / obs-zone
pub fn zone<I: U8Input + Debug>(i: I) -> SimpleResult<I, FixedOffset> {
    println!("zone: {:?}", i);
    or(i,
       |i| {
           fws(i).then(|i| {
               println!("zone.fws: {:?}", i);
               or(i, |i| token(i, b'+'), |i| token(i, b'-')).bind(|i, s| {
                   println!("zone.token: {:?}", i);
                   parse_digits(i, 2).bind(|i, offset_h: i32| {
                       println!("zone.parse-digits: {:?}", i);
                       parse_digits(i, 2).bind(|i, offset_m: i32| {
                           println!("zone.parse-digits: {:?}", i);
                           let offset = (offset_h * 3600) + (offset_m * 60);
                           let zone = match s {
                               b'+' => FixedOffset::east(offset),
                               _ => FixedOffset::west(offset),
                           };
                           i.ret(zone)
                       })
                   })
               })
           })
       },
       obs_zone)
}

#[test]
fn test_zone() {
    let i = b" -0330";
    let msg = parse_only(zone, i);
    assert!(msg.is_ok());
}

// address         =   mailbox / group
pub fn address<I: U8Input + Debug>(i: I) -> SimpleResult<I, Address> {
    or(i, mailbox, group)
}

// mailbox         =   name-addr / addr-spec
pub fn mailbox<I: U8Input + Debug>(i: I) -> SimpleResult<I, Address> {
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

#[test]
fn test_mailbox() {
    // let i = b"Mary Smith <@machine.tld:mary@example.net>";
    // let msg = parse_only(mailbox, i);
    // assert!(msg.is_ok());

    let i = b"jdoe@test   . example";
    let msg = parse_only(mailbox, i);
    assert!(msg.is_ok());

    let i = b"Joe Q. Public <john.q.public@example.com>";
    let msg = parse_only(mailbox, i);
    assert!(msg.is_ok());
}

// name-addr       =   [display-name] angle-addr
pub fn name_addr<I: U8Input + Debug>(i: I) -> SimpleResult<I, (Bytes, Bytes, Option<Bytes>)> {
    option(i, |i| {
        display_name(i).map(|n| Some(n))
    }, None).bind(|i, n| {
        angle_addr(i).bind(|i, (l, d)| {
            i.ret((l, d, n))
        })
    })
}

#[test]
fn test_name_addr() {
    /*
    let i = b"Who? <one@y.test>";
    let msg = parse_only(name_addr, i);
    assert!(msg.is_ok());
    let a = Address::Mailbox{
        local_part: "one".to_string(),
        domain: "y.test".to_string(),
        display_name: Some(Bytes::from_slice(b"Who? ")),
    };
    assert_eq!(msg.unwrap(), a);

    let i = b"\"Giant; \\\"Big\\\" Box\" <sysservices@example.net>";
    let msg = parse_only(name_addr, i);
    assert!(msg.is_ok());
    let a = Address::Mailbox{
        local_part: "sysservices".to_string(),
        domain: "example.net".to_string(),
        display_name: Some(Bytes::from_slice(b"Giant; \"Big\" Box ")),
    };
    assert_eq!(msg.unwrap(), a);
    */

    let i = b"Joe Q. Public <john.q.public@example.com>";
    let msg = parse_only(name_addr, i);
    assert!(msg.is_ok());

    let i = b"John Doe <jdoe@machine.example>";
    let msg = parse_only(name_addr, i);
    assert!(msg.is_ok());

    let i = b"<boss@nil.test>";
    let msg = parse_only(name_addr, i);
    assert!(msg.is_ok());

    let i = b"Pete(A wonderful \\) chap) <pete(his account)@silly.test(his host)>";
    let msg = parse_only(name_addr, i);
    assert!(msg.is_ok());
}

// angle-addr      =   [CFWS] "<" addr-spec ">" [CFWS] /
//                     obs-angle-addr
// NOTE: Not implementing obs-angle-addr because "routing" is bs
pub fn angle_addr<I: U8Input + Debug>(i: I) -> SimpleResult<I, (Bytes, Bytes)> {
    option(i, drop_cfws, ()).then(|i| {
        token(i, b'<').then(|i| {
            addr_spec(i).bind(|i, (l, d)| {
                token(i, b'>').then(|i| {
                    option(i, drop_cfws, ()).then(|i| {
                        i.ret((l, d))
                    })
                })
            })
        })
    })
}

#[test]
fn test_angle_addr() {
    let i = b"<jdoe@machine(comment).  example>";
    let msg = parse_only(angle_addr, i);
    assert!(msg.is_ok());

    let i = b"<pete(his account)@silly.test(his host)>";
    let msg = parse_only(angle_addr, i);
    assert!(msg.is_ok());

    let i = b"<jdoe@machine.example>";
    let msg = parse_only(angle_addr, i);
    assert!(msg.is_ok());
}

// group           =   display-name ":" [group-list] ";" [CFWS]
pub fn group<I: U8Input + Debug>(i: I) -> SimpleResult<I, Address> {
    display_name(i).bind(|i, n| {
        token(i, b':').then(|i| {
            option(i, group_list, None).bind(|i, l| {
                token(i, b';').then(|i| {
                    option(i, drop_cfws, ()).then(|i| {
                        let g = if l.is_some() {
                            Address::Group{
                                display_name: n,
                                mailboxes: l.unwrap(),
                            }
                        } else {
                            Address::Group{
                                display_name: n,
                                mailboxes: vec!(),
                            }
                        };
                        i.ret(g)
                    })
                })
            })
        })
    })
}

#[test]
fn test_group() {
    let i = b"(Empty list)(start)Undisclosed recipients  :(nobody(that I know))  ;";
    let msg = parse_only(group, i);
    assert!(msg.is_ok());

    let i = b"A Group(Some people)\r\n     :Chris Jones <c@(Chris's host.)public.example>,\r\n         joe@example.org,\r\n  John <jdoe@one.test> (my dear friend); (the end of the group)";
    let msg = parse_only(group, i);
    assert!(msg.is_ok());
}

//
// display-name    =   phrase
pub fn display_name<I: U8Input + Debug>(i: I) -> SimpleResult<I, Bytes> {
    phrase(i)
}

#[test]
fn test_display_name() {
    let i = b"A Group(Some people)\r\n     ";
    let msg = parse_only(display_name, i);
    assert!(msg.is_ok());

    let i = b"Joe Q. Public";
    let msg = parse_only(display_name, i);
    assert!(msg.is_ok());

    let i = b"Pete(A wonderful \\) chap)";
    let msg = parse_only(display_name, i);
    assert!(msg.is_ok());

    let i = b"John Doe";
    let msg = parse_only(display_name, i);
    assert!(msg.is_ok());
}


// mailbox-list    =   (mailbox *("," mailbox)) / obs-mbox-list
pub fn mailbox_list<I: U8Input + Debug>(i: I) -> SimpleResult<I, Vec<Address>> {
    or(i,
       |i| {
           mailbox(i).bind(|i, mb1| {
               many(i, |i| {
                   token(i, b',').then(mailbox)
               }).map(|mut mbs: Vec<Address>| {
                   mbs.insert(0, mb1);
                   mbs
               })
           })
       },
       obs_mbox_list)
}

#[test]
fn test_mailbox_list() {
    let i = b"Joe Q. Public <john.q.public@example.com>";
    let msg = parse_only(mailbox_list, i);
    assert!(msg.is_ok());

    let i = b"John Doe <jdoe@machine.example>";
    let msg = parse_only(mailbox_list, i);
    assert!(msg.is_ok());

    let i = b"Pete(A wonderful \\) chap) <pete(his account)@silly.test(his host)>";
    let msg = parse_only(mailbox_list, i);
    assert!(msg.is_ok());
}

// address-list    =   (address *("," address)) / obs-addr-list
pub fn address_list<I: U8Input + Debug>(i: I) -> SimpleResult<I, Vec<Address>> {
    or(i,
       |i| {
           address(i).bind(|i, ad1| {
               many(i, |i| {
                   token(i, b',').then(address)
               }).map(|mut ads: Vec<Address>| {
                   ads.insert(0, ad1);
                   ads
               })
           })
       },
       obs_addr_list)
}

#[test]
fn test_address_list() {
    let i = b"John Doe <jdoe@machine(comment).  example>";
    let msg = parse_only(address_list, i);
    assert!(msg.is_ok());

    let i = b"Mary Smith <mary@example.net>, , jdoe@test   . example";
    let msg = parse_only(address_list, i);
    assert!(msg.is_ok());

    // let i = b"Mary Smith <@machine.tld:mary@example.net>, , jdoe@test   . example";
    // let msg = parse_only(address_list, i);
    // assert!(msg.is_ok());

    let i = b"(Empty list)(start)Undisclosed recipients  :(nobody(that I know))  ;";
    let msg = parse_only(address_list, i);
    assert!(msg.is_ok());

    let i = b"A Group(Some people)\r\n     :Chris Jones <c@(Chris's host.)public.example>,\r\n         joe@example.org,\r\n  John <jdoe@one.test> (my dear friend); (the end of the group)";
    let msg = parse_only(address_list, i);
    assert!(msg.is_ok());

    let i = b"<boss@nil.test>, \"Giant; \\\"Big\\\" Box\" <sysservices@example.net>";
    let msg = parse_only(address_list, i);
    assert!(msg.is_ok());
}

// group-list      =   mailbox-list / CFWS / obs-group-list
// NOTE: Ignoring obs-group-list, as it appears to be wrong
pub fn group_list<I: U8Input + Debug>(i: I) -> SimpleResult<I, Option<Vec<Address>>> {
    or(i,
       |i| mailbox_list(i).map(|v| Some(v)),
       |i| cfws(i).map(|_| None))
}

//
// addr-spec       =   local-part "@" domain
pub fn addr_spec<I: U8Input + Debug>(i: I) -> SimpleResult<I, (Bytes, Bytes)> {
    local_part(i).bind(|i, l| {
        token(i, b'@').then(|i| {
            domain(i).bind(|i, d| {
                i.ret((l, d))
            })
        })
    })
}

#[test]
fn test_addr_spec() {
    // let i = b"@machine.tld:mary@example.net";
    // let msg = parse_only(addr_spec, i);
    // assert!(msg.is_ok());

    let i = b"jdoe@machine(comment).  example";
    let msg = parse_only(addr_spec, i);
    assert!(msg.is_ok());

    let i = b"pete(his account)@silly.test(his host)";
    let msg = parse_only(addr_spec, i);
    assert!(msg.is_ok());

    let i = b"jdoe@machine.example";
    let msg = parse_only(addr_spec, i);
    assert!(msg.is_ok());
}


// local-part      =   dot-atom / quoted-string / obs-local-part
pub fn local_part<I: U8Input + Debug>(i: I) -> SimpleResult<I, Bytes> {
    or(i,
       dot_atom,
       |i| or(i,
              quoted_string,
              obs_local_part))
}

#[test]
fn test_local_part() {
    // let i = b"@machine.tld:mary@example.net";
    // let msg = parse_only(local_part, i);
    // assert!(msg.is_ok());

    let i = b"pete(his account)";
    let msg = parse_only(local_part, i);
    assert!(msg.is_ok());
}


// domain          =   dot-atom / domain-literal / obs-domain
// TODO: Support new fields
pub fn domain<I: U8Input + Debug>(i: I) -> SimpleResult<I, Bytes> {
    obs_domain(i).map(|buf| Bytes::from_slice(&buf.into_vec()))
}

#[test]
fn test_domain() {
    let i = b"test   . example";
    let msg = parse_only(domain, i);
    assert!(msg.is_ok());

    let i = b"silly.test(his host)";
    let msg = parse_only(domain, i);
    assert!(msg.is_ok());

    let i = b"machine.example";
    let msg = parse_only(domain, i);
    assert!(msg.is_ok());
}

// domain-literal  =   [CFWS] "[" *([FWS] dtext) [FWS] "]" [CFWS]
//
// dtext           =   %d33-90 /          ; Printable US-ASCII
//                     %d94-126 /         ;  characters not including
//                     obs-dtext          ;  "[", "]", or "\"
//                     
//                 =   %d33-90 /          ; Substitute obs-dtext
//                     %d94-126 /
//                     obs-NO-WS-CTL /
//                     quoted-pair
//
//                 =   %d1-8 /            ; Substitute obs-NO-WS-CTL, reorganize
//                     %d11 /
//                     %d12 /
//                     %d14-31 /
//                     %d33-90 /
//                     %d94-126 /
//                     %d127 /
//                     quoted-pair
// 
const DTEXT: [bool; 256] = [
    //  0      1      2      3      4      5      6      7      8      9     10     11     12     13     14     15     16     17     18     19
    false, true,  true,  true,  true,  true,  true,  true,  true,  false, false, true,  true,  false, true,  true,  true,  true,  true,  true,  //   0 -  19
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false, true,  true,  true,  true,  true,  true,  true,  //  20 -  39
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  //  40 -  59
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  //  60 -  79
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false, false, false, true,  true,  true,  true,  true,  true,  //  80 -  99
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  // 100 - 119
    true,  true,  true,  true,  true,  true,  true,  true,  false, false, false, false, false, false, false, false, false, false, false, false, // 120 - 139
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 140 - 159
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 160 - 179
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 180 - 199
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 200 - 219
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 220 - 239
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false                              // 240 - 256
];
pub fn dtext<I: U8Input + Debug>(i: I) -> SimpleResult<I, u8> {
    or(i,
       |i| satisfy(i, |c| DTEXT[c as usize]),
       quoted_pair)
}

// message         =   (fields / obs-fields)
//                     [CRLF body]
// TODO: Support new fields
pub fn message<I: U8Input + Debug>(i: I) -> SimpleResult<I, Message> {
    println!("message: {:?}", i);
    obs_fields(i).bind(|i, f| {
        option(i, |i| {
            crlf(i).then(|i| {
                body(i).map(|b| Some(b))
            })
        }, None).bind(|i, b| {
            let message = Message {
                fields: f,
                body: b.map(|buf| Bytes::from_slice(&buf.into_vec())),
            };
            i.ret(message)
        })
    })
}

// body            =   (*(*998text CRLF) *998text) / obs-body
// TODO: support new fields
pub fn body<I: U8Input + Debug>(i: I) -> SimpleResult<I, I::Buffer> {
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
pub fn text<I: U8Input + Debug>(i: I) -> SimpleResult<I, u8> {
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
pub fn msg_id<I: U8Input + Debug>(i: I) -> SimpleResult<I, MessageID> {
    option(i, drop_cfws, ()).then(|i| {
        token(i, b'<').then(|i| {
            id_left(i).bind(|i, l| {
                token(i, b'@').then(|i| {
                    id_right(i).bind(|i, r| {
                        token(i, b'>').then(|i| {
                            option(i, drop_cfws, ()).then(|i| {
                                let message_id = MessageID{
                                    id_left: l,
                                    id_right: r,
                                };
                                i.ret(message_id)
                            })
                        })
                    })
                })
            })
        })
    })
}

// id-left         =   dot-atom-text / obs-id-left
pub fn id_left<I: U8Input + Debug>(i: I) -> SimpleResult<I, Bytes> {
    or(i, 
       |i| dot_atom_text(i).map(|buf| Bytes::from_slice(&buf.into_vec())), 
       obs_id_left)
}

// id-right        =   dot-atom-text / no-fold-literal / obs-id-right
pub fn id_right<I: U8Input + Debug>(i: I) -> SimpleResult<I, Bytes> {
    or(i, 
       |i| dot_atom_text(i).map(|buf| Bytes::from_slice(&buf.into_vec())), 
       |i| or(i, 
              |i| no_fold_literal(i).map(|buf| Bytes::from_slice(&buf.into_vec())),
              obs_id_right))
}

// no-fold-literal =   "[" *dtext "]"
pub fn no_fold_literal<I: U8Input + Debug>(i: I) -> SimpleResult<I, I::Buffer> {
    token(i, b'[').then(|i| {
        matched_by(i, |i| {
            skip_many(i, dtext)
        }).bind(|i, (buf, _)| {
            token(i, b']').then(|i| {
                i.ret(buf)
            })
        })
    })
}

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
pub fn field_name<I: U8Input + Debug>(i: I) -> SimpleResult<I, I::Buffer> {
    println!("field-name {:?}", i);
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
pub fn ftext<I: U8Input + Debug>(i: I) -> SimpleResult<I, u8> {
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
pub fn obs_no_ws_ctl<I: U8Input + Debug>(i: I) -> SimpleResult<I, u8> {
    satisfy(i, |c| OBS_NO_WS_CTL[c as usize])
}

// obs-ctext       =   obs-NO-WS-CTL
pub fn obs_ctext<I: U8Input + Debug>(i: I) -> SimpleResult<I, u8> {
	obs_no_ws_ctl(i)
}

// obs-qtext       =   obs-NO-WS-CTL
pub fn obs_qtext<I: U8Input + Debug>(i: I) -> SimpleResult<I, u8> {
	obs_no_ws_ctl(i)
}

// obs-utext       =   %d0 / obs-NO-WS-CTL / VCHAR
//                 =   %d0 /
//                     %d1-8 /
//                     %d11 /
//                     %d12 /
//                     %d14-31 /
//                     %d33-126 /
//                     %d127
pub fn obs_utext<I: U8Input + Debug>(i: I) -> SimpleResult<I, u8> {
	or(i, 
       |i| satisfy(i, |i| i == 0),
       |i| or(i, obs_no_ws_ctl, vchar))
}

pub fn many1_obs_utext<I: U8Input + Debug>(i: I) -> SimpleResult<I, I::Buffer> {
    matched_by(i, |i| {
        skip_many1(i, obs_utext)
    }).map(|(buf, _)| buf)
}

// obs-qp          =   "\" (%d0 / obs-NO-WS-CTL / LF / CR)
// Where any quoted-pair appears, it is to be interpreted as the
// character alone.  That is to say, the "\" character that appears as
// part of a quoted-pair is semantically "invisible".
pub fn obs_qp<I: U8Input + Debug>(i: I) -> SimpleResult<I, u8> {
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
// NOTE: We're relying on this parser only ever being evaluated after the
// header delimiter
pub fn obs_body<I: U8Input + Debug>(i: I) -> SimpleResult<I, I::Buffer> {
    take_remainder(i)
}

// obs-unstruct    =   *((*LF *CR *(obs-utext *LF *CR)) / FWS)
//
// obs-unstruct cfws = *((*LF *CR *(obs-utext *LF *CR)) / FWS) CRLF
//
// I _think_ this is equivalent
// obs-unstruct cfws = *(1*LF / 1*(CR >>!LF) / 1*obs-utext / FWS) CRLF
//                   = *(1*(LF obs-utext) / FWS / 1*(CR >>!LF)) CRLF
// obs-utext       =   %d0 / obs-NO-WS-CTL / VCHAR
//                 =   %d0 /
//                     %d1-8 /
//                     %d11 /
//                     %d12 /
//                     %d14-31 /
//                     %d33-126 /
//                     %d127
// LF              =   %d10
const LF_OBS_UTEXT: [bool; 256] = [
    //  0      1      2      3      4      5      6      7      8      9     10     11     12     13     14     15     16     17     18     19
    true,  true,  true,  true,  true,  true,  true,  true,  true,  false, true,  true,  true,  false, true,  true,  true,  true,  true,  true,  //   0 -  19
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false, true,  true,  true,  true,  true,  true,  true,  //  20 -  39
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
pub fn obs_unstruct_crlf<I: U8Input + Debug>(i: I) -> SimpleResult<I, Bytes> {
    println!("obs_unstruct_crlf {:?}", i);
    many(i, |i| {
        or(i,
           |i| take_while1(i, |t| LF_OBS_UTEXT[t as usize]).map(|buf| Bytes::from_slice(&buf.into_vec())),
           |i| or(i, fws, many1_cr_not_lf))
    }).bind(|i, segments: Vec<Bytes>| {
        crlf(i).then(|i| {
            i.ret(segments.into_iter().fold(Bytes::empty(), |l, r| l.concat(&r)))
        })
    })
}

pub fn many1_cr_not_lf<I: U8Input + Debug>(i: I) -> SimpleResult<I, Bytes> {
    matched_by(i, |i| {
        skip_many1(i, |i| {
            token(i, 13).then(|i| {
                peek(i).bind(|i, p| {
                    if p == Some(10) {
                        i.err(Error::unexpected())
                    } else {
                        i.ret(())
                    }
                })
            })
        })
    }).map(|(buf, _)| Bytes::from_slice(&buf.into_vec()))
}

#[test]
fn test_many1_cr_not_lf() {
    let good = b"\x0d\x0d\x0dhello";
    let msg = parse_only(many1_cr_not_lf, good);
    assert!(msg.is_ok());
    assert_eq!(msg.unwrap(), Bytes::from_slice(b"\x0d\x0d\x0d"));

    let bad = b"\x0d\x0d\x0ahello";
    let msg = parse_only(many1_cr_not_lf, bad);
    assert!(msg.is_ok());
    assert_eq!(msg.unwrap(), Bytes::from_slice(b"\x0d"));
}

// obs-phrase      =   word *(word / "." / CFWS)
pub fn obs_phrase<I: U8Input + Debug>(i: I) -> SimpleResult<I, Bytes> {
    println!("obs-phrase: {:?}", i);
    word(i).bind(|i, w1| {
        println!("obs-phrase.word1: {:?}/{:?}", w1.clone(), i);
        many(i, |i| {
            println!("obs-phrase.many: {:?}", i);
            or(i,
               word,
               |i| or(i,
                      |i| token(i, b'.').bind(|i, _| {
                          println!("obs-phrase.token: {:?}", i);
                          i.ret(Bytes::from_slice(&[b'.']))
                      }),
                      |i| cfws(i).bind(|i, _| {
                          println!("obs-phrase.cfws: {:?}", i);
                          i.ret(Bytes::empty())
                      })))
        }).map(|bufs: Vec<Bytes>| {
            bufs.into_iter().fold(w1, |l, r| l.concat(&r))
        })
    })
}

#[test]
fn test_obs_phrase() {
    let i = b"Joe Q. Public";
    let msg = parse_only(obs_phrase, i);
    assert!(msg.is_ok());
}

// obs-phrase-list =   [phrase / CFWS] *("," [phrase / CFWS])
pub fn obs_phrase_list<I: U8Input + Debug>(i: I) -> SimpleResult<I, Vec<Bytes>> {
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
// TODO: Drop ONLY CRLF
pub fn obs_fws<I: U8Input + Debug>(i: I) -> SimpleResult<I, Bytes> {
    matched_by(i, |i| {
        skip_many1(i, wsp)
    }).bind(|i, (buf1, _)| {
        many(i, |i| {
            crlf(i).then(|i| {
                matched_by(i, |i| {
                    skip_many1(i, wsp)
                }).map(|(buf, _)| Bytes::from_slice(&buf.into_vec()))
            })
        }).map(|bufs: Vec<Bytes>| {
            bufs.into_iter().fold(Bytes::from_slice(&buf1.into_vec()), |l, r| l.concat(&r))
        })
    })
}

// obs-day-of-week =   [CFWS] day-name [CFWS]
pub fn obs_day_of_week<I: U8Input + Debug>(i: I) -> SimpleResult<I, Day> {
    option(i, drop_cfws, ()).then(|i| {
        day_name(i).bind(|i, d| {
            option(i, drop_cfws, ()).then(|i| {
                i.ret(d)
            })
        })
    })
}

// obs-day         =   [CFWS] 1*2DIGIT [CFWS]
pub fn obs_day<I: U8Input + Debug>(i: I) -> SimpleResult<I, usize> {
    option(i, drop_cfws, ()).then(|i| {
        parse_digits(i, (1..3)).bind(|i, n| {
            option(i, drop_cfws, ()).then(|i| {
                i.ret(n)
            })
        })
    })
}

// obs-year        =   [CFWS] 2*DIGIT [CFWS]
pub fn obs_year<I: U8Input + Debug>(i: I) -> SimpleResult<I, usize> {
    option(i, drop_cfws, ()).then(|i| {
        parse_digits(i, (2..4)).bind(|i, n| {
            option(i, drop_cfws, ()).then(|i| {
                i.ret(n)
            })
        })
    })
}

// obs-hour        =   [CFWS] 2DIGIT [CFWS]
pub fn obs_hour<I: U8Input + Debug>(i: I) -> SimpleResult<I, usize> {
    option(i, drop_cfws, ()).then(|i| {
        parse_digits(i, 2).bind(|i, n| {
            option(i, drop_cfws, ()).then(|i| {
                i.ret(n)
            })
        })
    })
}

// obs-minute      =   [CFWS] 2DIGIT [CFWS]
pub fn obs_minute<I: U8Input + Debug>(i: I) -> SimpleResult<I, usize> {
    option(i, drop_cfws, ()).then(|i| {
        parse_digits(i, 2).bind(|i, n| {
            option(i, drop_cfws, ()).then(|i| {
                i.ret(n)
            })
        })
    })
}

// obs-second      =   [CFWS] 2DIGIT [CFWS]
pub fn obs_second<I: U8Input + Debug>(i: I) -> SimpleResult<I, usize> {
    option(i, drop_cfws, ()).then(|i| {
        parse_digits(i, 2).bind(|i, n| {
            option(i, drop_cfws, ()).then(|i| {
                i.ret(n)
            })
        })
    })
}

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
// NOTE: Modifying to allow preceeding FWS
pub fn obs_zone<I: U8Input + Debug>(i: I) -> SimpleResult<I, FixedOffset> {
    fws(i).then(|i| {
        or(i, |i| string(i, b"UT").then(|i| i.ret(0)),
        |i| or(i, |i| string(i, b"GMT").then(|i| i.ret(0)),
        |i| or(i, |i| string(i, b"EST").then(|i| i.ret(-5)),
        |i| or(i, |i| string(i, b"EDT").then(|i| i.ret(-4)),
        |i| or(i, |i| string(i, b"CST").then(|i| i.ret(-6)),
        |i| or(i, |i| string(i, b"CDT").then(|i| i.ret(-5)),
        |i| or(i, |i| string(i, b"MST").then(|i| i.ret(-7)),
        |i| or(i, |i| string(i, b"MDT").then(|i| i.ret(-6)),
        |i| or(i, |i| string(i, b"PST").then(|i| i.ret(-8)),
        |i| or(i, |i| string(i, b"PDT").then(|i| i.ret(-7)),
        |i| or(i, |i| satisfy(i, |i| 65 <= i && i <= 73).then(|i| i.ret(0)),
        |i| or(i, |i| satisfy(i, |i| 75 <= i && i <= 90).then(|i| i.ret(0)),
        |i| or(i, |i| satisfy(i, |i| 97 <= i && i <= 105).then(|i| i.ret(0)),
        |i| or(i, |i| satisfy(i, |i| 107 <= i && i <= 122).then(|i| i.ret(0)),
        |i| skip_many1(i, alpha).then(|i| i.ret(0)),
        )))))))))))))).map(|o| FixedOffset::west(o))
    })
}

#[test]
fn test_obs_zone() {
    let i = b"-0330 (Newfoundland Time)\r\n";
    let msg = parse_only(obs_zone, i);
    assert!(msg.is_err());
}

// obs-angle-addr  =   [CFWS] "<" obs-route addr-spec ">" [CFWS]
// NOTE: Not supporting because obs-route is stupid

// obs-route       =   obs-domain-list ":"
// NOTE: Not supporting because why, even?

// obs-domain-list =   *(CFWS / ",") "@" domain
//                     *("," [CFWS] ["@" domain])
pub fn obs_domain_list<I: U8Input + Debug>(i: I) -> SimpleResult<I, Vec<Bytes>> {
    skip_many(i, |i| {
        or(i, drop_cfws, |i| token(i, b',').map(|_| ()))
    }).then(|i| {
        token(i, b'@').then(|i| {
            domain(i).bind(|i, domain1| {
                many(i, |i| {
                    token(i, b',').then(|i| {
                        option(i, drop_cfws, ()).then(|i| {
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
pub fn obs_mbox_list<I: U8Input + Debug>(i: I) -> SimpleResult<I, Vec<Address>> {
    skip_many(i, |i| {
        option(i, drop_cfws, ()).then(|i| {
            token(i, b',')
        })
    }).then(|i| {
        mailbox(i).bind(|i, mb1| {
            many(i, |i| {
                token(i, b',').then(|i| {
                    or(i,
                       |i| mailbox(i).map(|mb| Some(mb)),
                       |i| option(i, drop_cfws, ()).map(|_| None))
                })
            }).map(|maybe_mbs: Vec<Option<Address>>| {
                let mut mbs = Vec::with_capacity(maybe_mbs.len());
                mbs.push(mb1);
                maybe_mbs.into_iter().fold(mbs, |mut l, r| {
                    if r.is_some() {
                        l.push(r.unwrap())
                    }
                    l
                })
            })
        })
    })
}

// obs-addr-list   =   *([CFWS] ",") address *("," [address / CFWS])
pub fn obs_addr_list<I: U8Input + Debug>(i: I) -> SimpleResult<I, Vec<Address>> {
    skip_many(i, |i| {
        option(i, drop_cfws, ()).then(|i| {
            token(i, b',')
        })
    }).then(|i| {
        address(i).bind(|i, ad1| {
            many(i, |i| {
                token(i, b',').then(|i| {
                    or(i,
                       |i| address(i).map(|v| Some(v)),
                       |i| cfws(i).map(|_| None))
                })
            }).map(|maybe_ads: Vec<Option<Address>>| {
                let mut ads = Vec::with_capacity(maybe_ads.len());
                ads.push(ad1);
                maybe_ads.into_iter().fold(ads, |mut l, r| {
                    if r.is_some() {
                        l.push(r.unwrap())
                    }
                    l
                })
            })
        })
    })
}

// obs-group-list  =   1*([CFWS] ",") [CFWS]
// NOTE: Pretty sure this is wrong

// obs-local-part  =   word *("." word)
pub fn obs_local_part<I: U8Input + Debug>(i: I) -> SimpleResult<I, Bytes> {
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
pub fn obs_domain<I: U8Input + Debug>(i: I) -> SimpleResult<I, I::Buffer> {
    matched_by(i, |i| {
        atom(i).then(|i| {
            skip_many(i, |i| {
                token(i, b'.').then(|i| {
                    atom(i)
                })
            })
        })
    }).map(|(buf, _)| buf)
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
pub fn obs_fields<I: U8Input + Debug>(i: I) -> SimpleResult<I, Vec<Field>> {
    // NOTE: REALLY wish the parser macro worked right about here
    many(i, |i| {
        or(i,       obs_orig_date,
        |i| or(i,   obs_from,
        |i| or(i,   obs_sender,
        |i| or(i,   obs_reply_to,
        |i| or(i,   obs_to,
        |i| or(i,   obs_cc,
        |i| or(i,   obs_message_id,
        |i| or(i,   obs_in_reply_to,
        |i| or(i,   obs_references,
        |i| or(i,   obs_subject,
        |i| or(i,   obs_comments,
        // TODO: Should these be Field variants?
        // |i| or(i,   obs_resent_from,
        // |i| or(i,   obs_resent_send,
        // |i| or(i,   obs_resent_date,
        // |i| or(i,   obs_resent_to,
        // |i| or(i,   obs_resent_cc,
        // |i| or(i,   obs_resent_mid,
        // |i| or(i,   obs_resent_rply,
                    obs_optional,
                    )))))))))))
    })
}

// obs-orig-date   =   "Date" *WSP ":" date-time CRLF
pub fn obs_orig_date<I: U8Input + Debug>(i: I) -> SimpleResult<I, Field> {
    println!("obs-orig-date {:?}", i);
    string(i, b"Date").then(|i| {
        println!("obs-orig-date.string(Date) {:?}", i);
        option(i, wsp, 0).then(|i| {
            println!("obs-orig-date.wsp {:?}", i);
            token(i, b':').then(|i| {
                println!("obs-orig-date.token(:) {:?}", i);
                date_time(i).bind(|i, dt| {
                    println!("obs-orig-date.date-time {:?}", i);
                    crlf(i).then(|i| {
                        println!("obs-orig-date.crlf {:?}", i);
                        let value = DateTimeField {date_time: dt};

                        i.ret(Field::Date(value))
                    })
                })
            })
        })
    })
}

#[test]
fn test_obs_orig_date() {
    let i = b"Date: Fri, 21 Nov 1997 09:55:06 -0600\x0d\x0a";
    let msg = parse_only(obs_orig_date, i);
    assert!(msg.is_ok());
    println!("parsed date: {:?}", msg.unwrap());
}

// obs-from        =   "From" *WSP ":" mailbox-list CRLF
pub fn obs_from<I: U8Input + Debug>(i: I) -> SimpleResult<I, Field> {
    println!("from {:?}", i);
    string(i, b"From").then(|i| {
        println!("from.string {:?}", i);
        option(i, wsp, 0).then(|i| {
            println!("from.wsp {:?}", i);
            token(i, b':').then(|i| {
                println!("from.token {:?}", i);
                mailbox_list(i).bind(|i, mbs| {
                    println!("from.mailbox_list {:?}", i);
                    crlf(i).then(|i| {
                        println!("from.crlf {:?}", i);
                        let value = AddressesField {addresses: mbs};

                        i.ret(Field::From(value))
                    })
                })
            })
        })
    })
}

#[test]
fn test_obs_from() {
    let i = b"From: John Doe <jdoe@machine.example>\x0d\x0a";
    let msg = parse_only(obs_from, i);
    assert!(msg.is_ok());
    println!("parsed from: {:?}", msg.unwrap());
}

// obs-sender      =   "Sender" *WSP ":" mailbox CRLF
pub fn obs_sender<I: U8Input + Debug>(i: I) -> SimpleResult<I, Field> {
    string(i, b"Sender").then(|i| {
        option(i, wsp, 0).then(|i| {
            token(i, b':').then(|i| {
                mailbox(i).bind(|i, mb| {
                    crlf(i).then(|i| {
                        let value = AddressField {address: mb};

                        i.ret(Field::Sender(value))
                    })
                })
            })
        })
    })
}

// obs-reply-to    =   "Reply-To" *WSP ":" address-list CRLF
pub fn obs_reply_to<I: U8Input + Debug>(i: I) -> SimpleResult<I, Field> {
    string(i, b"Reply-To").then(|i| {
        option(i, wsp, 0).then(|i| {
            token(i, b':').then(|i| {
                address_list(i).bind(|i, mbs| {
                    crlf(i).then(|i| {
                        let value = AddressesField {addresses: mbs};

                        i.ret(Field::ReplyTo(value))
                    })
                })
            })
        })
    })
}

// obs-to          =   "To" *WSP ":" address-list CRLF
pub fn obs_to<I: U8Input + Debug>(i: I) -> SimpleResult<I, Field> {
    string(i, b"To").then(|i| {
        option(i, wsp, 0).then(|i| {
            token(i, b':').then(|i| {
                address_list(i).bind(|i, mbs| {
                    crlf(i).then(|i| {
                        let value = AddressesField {addresses: mbs};

                        i.ret(Field::To(value))
                    })
                })
            })
        })
    })
}

#[test]
fn test_obs_to() {
    let i = b"To: Mary Smith <mary@example.net>\x0d\x0a";
    let msg = parse_only(obs_to, i);
    assert!(msg.is_ok());
    println!("parsed to: {:?}", msg.unwrap());
}

// obs-cc          =   "Cc" *WSP ":" address-list CRLF
pub fn obs_cc<I: U8Input + Debug>(i: I) -> SimpleResult<I, Field> {
    string(i, b"Cc").then(|i| {
        option(i, wsp, 0).then(|i| {
            token(i, b':').then(|i| {
                address_list(i).bind(|i, mbs| {
                    crlf(i).then(|i| {
                        let value = AddressesField {addresses: mbs};

                        i.ret(Field::Cc(value))
                    })
                })
            })
        })
    })
}

// obs-bcc         =   "Bcc" *WSP ":"
//                     (address-list / (*([CFWS] ",") [CFWS])) CRLF
//
// obs-message-id  =   "Message-ID" *WSP ":" msg-id CRLF
pub fn obs_message_id<I: U8Input + Debug>(i: I) -> SimpleResult<I, Field> {
    string(i, b"Message-ID").then(|i| {
        option(i, wsp, 0).then(|i| {
            token(i, b':').then(|i| {
                msg_id(i).bind(|i, m| {
                    crlf(i).then(|i| {
                        let value = MessageIDField {message_id: m};

                        i.ret(Field::MessageID(value))
                    })
                })
            })
        })
    })
}

#[test]
fn test_obs_message_id() {
    let i = b"Message-ID: <1234@local.machine.example>\x0d\x0a";
    let msg = parse_only(obs_message_id, i);
    assert!(msg.is_ok());
    println!("parsed message-id: {:?}", msg.unwrap());
}

// obs-in-reply-to =   "In-Reply-To" *WSP ":" *(phrase / msg-id) CRLF
//    For purposes of interpretation, the phrases in the "In-Reply-To:" and
//    "References:" fields are ignored.
pub fn obs_in_reply_to<I: U8Input + Debug>(i: I) -> SimpleResult<I, Field> {
    string(i, b"In-Reply-To").then(|i| {
        option(i, wsp, 0).then(|i| {
            token(i, b':').then(|i| {
                many(i, |i| {
                    or(i, 
                       |i| phrase(i).map(|_| None),
                       |i| msg_id(i).map(|v| Some(v)))
                }).bind(|i, vs: Vec<Option<MessageID>>| {
                    crlf(i).then(|i| {
                        let message_ids = vs.into_iter()
                            .filter(|v| v.is_some())
                            .map(|v| v.unwrap())
                            .collect::<Vec<MessageID>>();
                        let value = MessageIDsField {message_ids: message_ids};

                        i.ret(Field::InReplyTo(value))
                    })
                })
            })
        })
    })
}

// obs-references  =   "References" *WSP ":" *(phrase / msg-id) CRLF
//    For purposes of interpretation, the phrases in the "In-Reply-To:" and
//    "References:" fields are ignored.
pub fn obs_references<I: U8Input + Debug>(i: I) -> SimpleResult<I, Field> {
    string(i, b"In-References").then(|i| {
        option(i, wsp, 0).then(|i| {
            token(i, b':').then(|i| {
                many(i, |i| {
                    or(i, 
                       |i| phrase(i).map(|_| None),
                       |i| msg_id(i).map(|v| Some(v)))
                }).bind(|i, vs: Vec<Option<MessageID>>| {
                    crlf(i).then(|i| {
                        let message_ids = vs.into_iter()
                            .filter(|v| v.is_some())
                            .map(|v| v.unwrap())
                            .collect::<Vec<MessageID>>();
                        let value = MessageIDsField {message_ids: message_ids};

                        i.ret(Field::References(value))
                    })
                })
            })
        })
    })
}

// obs-id-left     =   local-part
pub fn obs_id_left<I: U8Input + Debug>(i: I) -> SimpleResult<I, Bytes> {
    local_part(i)
}

// obs-id-right    =   domain
pub fn obs_id_right<I: U8Input + Debug>(i: I) -> SimpleResult<I, Bytes> {
    domain(i)
}

// obs-subject     =   "Subject" *WSP ":" unstructured CRLF
pub fn obs_subject<I: U8Input + Debug>(i: I) -> SimpleResult<I, Field> {
    println!("subject {:?}", i);
    string(i, b"Subject").then(|i| {
        println!("subject.string {:?}", i);
        option(i, wsp, 0).then(|i| {
            println!("subject.wsp {:?}", i);
            token(i, b':').then(|i| {
                println!("subject.token {:?}", i);
                unstructured_crlf(i).bind(|i, v| {
                    println!("subject.crlf {:?}", i);
                    let value = UnstructuredField {data: v};

                    i.ret(Field::Subject(value))
                })
            })
        })
    })
}

#[test]
fn test_obs_subject() {
    let i = b"Subject: Saying Hello\x0d\x0a";
    let msg = parse_only(obs_subject, i);
    assert!(msg.is_ok());
    println!("parsed subject: {:?}", msg.unwrap());
}


// obs-comments    =   "Comments" *WSP ":" unstructured CRLF
pub fn obs_comments<I: U8Input + Debug>(i: I) -> SimpleResult<I, Field> {
    string(i, b"Comments").then(|i| {
        option(i, wsp, 0).then(|i| {
            token(i, b':').then(|i| {
                unstructured_crlf(i).bind(|i, v| {
                    let value = UnstructuredField {data: v};

                    i.ret(Field::Comments(value))
                })
            })
        })
    })
}

// obs-keywords    =   "Keywords" *WSP ":" obs-phrase-list CRLF
//
// obs-resent-from =   "Resent-From" *WSP ":" mailbox-list CRLF
pub fn obs_resent_from<I: U8Input + Debug>(i: I) -> SimpleResult<I, Resent> {
    string(i, b"Resent-Cc").then(|i| {
        option(i, wsp, 0).then(|i| {
            token(i, b':').then(|i| {
                mailbox_list(i).bind(|i, mbs| {
                    crlf(i).then(|i| {
                        let value = AddressesField {addresses: mbs};

                        i.ret(Resent::Cc(value))
                    })
                })
            })
        })
    })
}

// obs-resent-send =   "Resent-Sender" *WSP ":" mailbox CRLF
pub fn obs_resent_send<I: U8Input + Debug>(i: I) -> SimpleResult<I, Resent> {
    string(i, b"Resent-Sender").then(|i| {
        option(i, wsp, 0).then(|i| {
            token(i, b':').then(|i| {
                mailbox(i).bind(|i, mb| {
                    crlf(i).then(|i| {
                        let value = AddressField {address: mb};

                        i.ret(Resent::Sender(value))
                    })
                })
            })
        })
    })
}

// obs-resent-date =   "Resent-Date" *WSP ":" date-time CRLF
pub fn obs_resent_date<I: U8Input + Debug>(i: I) -> SimpleResult<I, Resent> {
    string(i, b"Resent-Date").then(|i| {
        option(i, wsp, 0).then(|i| {
            token(i, b':').then(|i| {
                date_time(i).bind(|i, dt| {
                    crlf(i).then(|i| {
                        let value = DateTimeField {date_time: dt};

                        i.ret(Resent::Date(value))
                    })
                })
            })
        })
    })
}
//
// obs-resent-to   =   "Resent-To" *WSP ":" address-list CRLF
pub fn obs_resent_to<I: U8Input + Debug>(i: I) -> SimpleResult<I, Resent> {
    string(i, b"Resent-To").then(|i| {
        option(i, wsp, 0).then(|i| {
            token(i, b':').then(|i| {
                address_list(i).bind(|i, mbs| {
                    crlf(i).then(|i| {
                        let value = AddressesField {addresses: mbs};

                        i.ret(Resent::To(value))
                    })
                })
            })
        })
    })
}

// obs-resent-cc   =   "Resent-Cc" *WSP ":" address-list CRLF
pub fn obs_resent_cc<I: U8Input + Debug>(i: I) -> SimpleResult<I, Resent> {
    string(i, b"Resent-Cc").then(|i| {
        option(i, wsp, 0).then(|i| {
            token(i, b':').then(|i| {
                address_list(i).bind(|i, mbs| {
                    crlf(i).then(|i| {
                        let value = AddressesField {addresses: mbs};

                        i.ret(Resent::Cc(value))
                    })
                })
            })
        })
    })
}

// obs-resent-bcc  =   "Resent-Bcc" *WSP ":"
//                     (address-list / (*([CFWS] ",") [CFWS])) CRLF
//
// obs-resent-mid  =   "Resent-Message-ID" *WSP ":" msg-id CRLF
pub fn obs_resent_mid<I: U8Input + Debug>(i: I) -> SimpleResult<I, Resent> {
    string(i, b"Resent-Message-ID").then(|i| {
        option(i, wsp, 0).then(|i| {
            token(i, b':').then(|i| {
                msg_id(i).bind(|i, v| {
                    crlf(i).then(|i| {
                        let value = MessageIDField {message_id: v};

                        i.ret(Resent::MessageID(value))
                    })
                })
            })
        })
    })
}

// obs-resent-rply =   "Resent-Reply-To" *WSP ":" address-list CRLF
pub fn obs_resent_rply<I: U8Input + Debug>(i: I) -> SimpleResult<I, Resent> {
    string(i, b"Resent-Reply-To").then(|i| {
        option(i, wsp, 0).then(|i| {
            token(i, b':').then(|i| {
                address_list(i).bind(|i, mbs| {
                    crlf(i).then(|i| {
                        let value = AddressesField {addresses: mbs};

                        i.ret(Resent::ReplyTo(value))
                    })
                })
            })
        })
    })
}

// obs-return      =   "Return-Path" *WSP ":" path CRLF
//
// obs-received    =   "Received" *WSP ":" *received-token CRLF
//
// obs-optional    =   field-name *WSP ":" unstructured CRLF
pub fn obs_optional<I: U8Input + Debug>(i: I) -> SimpleResult<I, Field> {
    println!("obs_optional {:?}", i);
    field_name(i).bind(|i, n| {
        println!("obs_optional.field-name {:?}", i);
        option(i, wsp, 0).then(|i| {
            println!("obs_optional.wsp {:?}", i);
            token(i, b':').then(|i| {
                println!("obs_optional.token {:?}", i);
                unstructured_crlf(i).bind(|i, v| {
                    // NOTE: `field-name` is "printable US-ASCII characters not including ':'"
                    let name = unsafe { String::from_utf8_unchecked(n.into_vec()) };
                    let value = UnstructuredField {data: v};

                    i.ret(Field::Optional(name, value))
                })
            })
        })
    })
}
