use chomp::*;
use chomp::combinators::bounded;
use chrono::offset::fixed::FixedOffset;
use bytes::{Bytes, ByteStr};

use util::*;
use rfc2822::*;
use rfc2822::address::*;
use rfc2822::atom::*;
use rfc2822::datetime::*;
use rfc2822::folding::*;
use rfc2822::misc::*;
use rfc2822::primitive::*;

// %d0-127 except CR and LF
// obs-char = %d0-9 / %d11 / %d12 / %d14-127
pub fn obs_char(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |i| (i <= 9) || (i == 11) || (i == 12) || (14 <= i && i <= 127))
}

#[test]
pub fn test_obs_char() {
    assert_eq!(parse_only(obs_char, "1".as_bytes()), Ok('1' as u8));
    assert_eq!(parse_only(obs_char, &[10][..]), Err(ParseError::Error(&[10][..], Error::Unexpected)));
}

// obs-text = *LF *CR *(obs-char *LF *CR)
//
// NOTE: I think this is a flaw in the spec - it leaks
// matches to *(%0-9 / %11 / %12 / %14-127).  This matcher eliminates the 
// obs-char repeat
pub fn obs_text(i: Input<u8>) -> U8Result<u8> {
    parse!{i;
        skip_many(lf);
        skip_many(cr);
        let c = obs_char();
        skip_many(lf);
        skip_many(cr);

        ret c
    }
}

// obs-qp = "\" (%d0-127)
// Consumes & returns matches
pub fn obs_qp(i: Input<u8>) -> U8Result<u8> {
    parse!{i;
        token(b'\\');
        satisfy(|i| i <= 127)
    }
}

// obs-FWS = 1*WSP *(CRLF 1*WSP)
pub fn obs_fws(i: Input<u8>) -> U8Result<Bytes> {
    matched_by(i, |i| skip_many1(i, wsp)).bind(|i, (pre, _)| {
        many(i, |i| {
            crlf(i).then(|i| {
                matched_by(i, |i| skip_many1(i, wsp)).bind(|i, (post, _)| {
                    i.ret(Bytes::from_slice(post))
                })
            })

        }).bind(|i, post: Vec<Bytes>| {
            let bs = post.into_iter().fold(Bytes::from_slice(pre), |acc, b| acc.concat(&b));

            i.ret(bs)
        })
    })
}

// obs-phrase = word *(word / "." / CFWS)
// TODO: Figure out how this gets stuck...
pub fn obs_phrase(i: Input<u8>) -> U8Result<Bytes> {
    word(i).bind(|i, w1| {
        // println!("obs_phrase.word.bind({:?}, {:?})", i, w1);

        let a = |i| {
            // println!("obs_phrase.many({:?})", i);

            let w = |i| {
                word(i).bind(|i, v| {
                    // println!("obs_phrase.word.bind({:?}, {:?})", i, v);
                    i.ret(v)
                })
            };
            let t = |i| {
                token(i, b'.').map(|_| Bytes::from_slice(&[b'.'][..])).bind(|i, v| {
                    // println!("obs_phrase.token.bind({:?}, {:?})", i, v);
                    i.ret(v)
                })
            };
            let c = |i| {
               cfws(i).map(|_| Bytes::empty()).bind(|i, v| {
                    // println!("obs_phrase.cfws.bind({:?}, {:?})", i, v);
                    i.ret(v)
               })
            };

            or(i, w, |i| or(i, t, c))
               //     or(i, word,
               // |i| or(i, |i| token(i, b'.').map(|_| vec!(b'.')), 
               //           |i| cfws(i).map(|_| vec!())))
        };

        // TODO: Fix cfws cycle, then remove the bound here...
        bounded::many(i, (0..10), a).bind(|i, ws: Vec<Bytes>| {
            // println!("obs_phrase.many(a).bind({:?}, {:?})", i, ws);

            let bs = ws.into_iter().fold(w1, |acc, wn| acc.concat(&wn));
            i.ret(bs)
        })
    })
}

#[test]
fn test_obs_phrase() {
    let i = b"Joe Q. Public";
    let msg = parse_only(obs_phrase, i);
    assert!(msg.is_ok());
}

// obs-day-of-week = [CFWS] day-name [CFWS]
pub fn obs_day_of_week(i: Input<u8>) -> U8Result<Day> {
    parse!{i;
        option(cfws, Bytes::empty());
        let d = day_name();
        option(cfws, Bytes::empty());

        ret d
    }
}

// obs-day = [CFWS] 1*2DIGIT [CFWS]
pub fn obs_day(i: Input<u8>) -> U8Result<usize> {
    parse!{i;
        option(cfws, Bytes::empty());
        let n = parse_digits((1..3));
        option(cfws, Bytes::empty());

        ret n
    }
}

// obs-month = CFWS month-name CFWS
pub fn obs_month(i: Input<u8>) -> U8Result<Month> {
    parse!{i;
        option(cfws, Bytes::empty());
        let m = month_name();
        option(cfws, Bytes::empty());

        ret m
    }
}

// obs-year = [CFWS] 2*DIGIT [CFWS]
// NOTE: obs-year is only used in year, which is only used in date,
// which is only used in date-time, which is broken by the terminal [CFWS] 
// because it prevents FWS from ever matching.  So I'm dropping it - effective:
// obs-year = [CFWS] 2*DIGIT
pub fn obs_year(i: Input<u8>) -> U8Result<usize> {
    option(i, cfws, Bytes::empty()).then(|i| {
        parse_digits(i, (2..4)).bind(|i, y: usize| {
            let actual_year = if y < 49 {
                y + 2000
            } else {
                y + 1900
            };
            i.ret(actual_year)
        })
    })
}

// obs-hour = [CFWS] 2DIGIT [CFWS]
pub fn obs_hour(i: Input<u8>) -> U8Result<usize> {
    parse!{i;
        option(cfws, Bytes::empty());
        let n = parse_digits(2);
        option(cfws, Bytes::empty());

        ret n
    }
}

// obs-minute = [CFWS] 2DIGIT [CFWS]
pub fn obs_minute(i: Input<u8>) -> U8Result<usize> {
    parse!{i;
        option(cfws, Bytes::empty());
        let n = parse_digits(2);
        option(cfws, Bytes::empty());

        ret n
    }
}

// obs-second = [CFWS] 2DIGIT [CFWS]
pub fn obs_second(i: Input<u8>) -> U8Result<usize> {
    parse!{i;
        option(cfws, Bytes::empty());
        let n = parse_digits(2);
        option(cfws, Bytes::empty());

        ret n
    }
}

// obs-zone        =       "UT" / "GMT" /          ; Universal Time
//                                                 ; North American UT
//                                                 ; offsets
//                         "EST" / "EDT" /         ; Eastern:  - 5/ - 4
//                         "CST" / "CDT" /         ; Central:  - 6/ - 5
//                         "MST" / "MDT" /         ; Mountain: - 7/ - 6
//                         "PST" / "PDT" /         ; Pacific:  - 8/ - 7
//
//                         %d65-73 /               ; Military zones - "A"
//                         %d75-90 /               ; through "I" and "K"
//                         %d97-105 /              ; through "Z", both
//                         %d107-122               ; upper and lower case
// 
// "Other multi-character (usually between 3 and 5) alphabetic time zones
// have been used in Internet messages.  Any such time zone whose
// meaning is not known SHOULD be considered equivalent to "-0000"
// unless there is out-of-band information confirming their meaning."
//
pub fn obs_zone(i: Input<u8>) -> U8Result<FixedOffset> {
    println!("obs_zone({:?})", i);

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
}

#[test]
fn test_obs_zone() {
    let i = b"-0330 (Newfoundland Time)\r\n";
    let msg = parse_only(obs_zone, i);
    assert!(msg.is_err());
}

// obs-local-part = word *("." word)
// NOTE Excluding '@' from matches
pub fn obs_local_part(i: Input<u8>) -> U8Result<Bytes> {
    let b = |i| {
        word_not(i, |c| c == b'@').then(|i| {
            skip_many(i, |i| {
                token(i, b'.').then(|i| {
                    word_not(i, |c| c == b'@')
                })
            })
        })
    };

    matched_by(i, b).bind(|i, (v, _)| {
        i.ret(Bytes::from_slice(v))
    })
}

// obs-domain = atom *("." atom)
pub fn obs_domain(i: Input<u8>) -> U8Result<Bytes> {
    matched_by(i, |i| { parse!{i;
        atom();
        skip_many(|i| { parse!{i;
            token(b'.');
            atom();

            ret ()
        }});
    }}).map(|(v, _)| Bytes::from_slice(v))
}

// obs-mbox-list = 1*([mailbox] [CFWS] "," [CFWS]) [mailbox]
pub fn obs_mbox_list(i: Input<u8>) -> U8Result<Vec<Address>> {
    let r = parse!{i;
        let an:Vec<Option<Address>> = many1(|i| { parse!{i;
            let m: Option<Address> = option(|i| mailbox(i).map(|i| Some(i)), None);
            option(cfws, Bytes::empty());
            token(b',');
            option(cfws, Bytes::empty());

            ret m
        }});

        let a: Option<Address> = option(|i| mailbox(i).map(|i| Some(i)), None);

        ret (an, a)
    };

    r.map(|(mut an, a)| {
        an.push(a);
        an.into_iter().filter(|i| *i != None).map(|i| i.unwrap()).collect()
    })
}

// obs-addr-list   =       1*([address] [CFWS] "," [CFWS]) [address]
pub fn obs_addr_list(i: Input<u8>) -> U8Result<Vec<Address>> {
    let r = parse!{i;
        let an:Vec<Option<Address>> = many1(|i| { parse!{i;
            let m: Option<Address> = option(|i| address(i).map(|i| Some(i)), None);
            option(cfws, Bytes::empty());
            token(b',');
            option(cfws, Bytes::empty());

            ret m
        }});

        let a: Option<Address> = option(|i| address(i).map(|i| Some(i)), None);

        ret (an, a)
    };

    r.map(|(mut an, a)| {
        an.push(a);
        an.into_iter().filter(|i| *i != None).map(|i| i.unwrap()).collect()
    })
}

// obs-id-left     =       local-part
pub fn obs_id_left(i: Input<u8>) -> U8Result<Bytes> {
    local_part(i)
}

// obs-id-right    =       domain
pub fn obs_id_right(i: Input<u8>) -> U8Result<Bytes> {
    domain(i)
}

// NOTE: I'm omitting all this noise
//
// obs-domain-list =       "@" domain *(*(CFWS / "," ) [CFWS] "@" domain)
// obs-route       =       [CFWS] obs-domain-list ":" [CFWS]
// obs-angle-addr  =       [CFWS] "<" [obs-route] addr-spec ">" [CFWS]
// obs-path        =       obs-angle-addr
