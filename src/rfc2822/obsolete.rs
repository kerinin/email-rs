use chomp::*;
use chrono::offset::fixed::FixedOffset;

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
// Consumes matches & returns ()
pub fn obs_fws(i: Input<u8>) -> U8Result<()> {
    parse!{i;
        skip_many1(wsp);
        skip_many(|i| parse!{i;
            crlf();
            skip_many1(wsp);
        })
    }
}

// obs-phrase = word *(word / "." / CFWS)
pub fn obs_phrase(i: Input<u8>) -> U8Result<Vec<u8>> {
    let r = parse!{i;
        let w1: Vec<u8> = word();
        let wv: Vec<Vec<u8>> = many(|i| {
            or(i,
               word,
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

// obs-day-of-week = [CFWS] day-name [CFWS]
pub fn obs_day_of_week(i: Input<u8>) -> U8Result<Day> {
    parse!{i;
        option(cfws, ());
        let d = day_name();
        option(cfws, ());

        ret d
    }
}

// obs-day = [CFWS] 1*2DIGIT [CFWS]
pub fn obs_day(i: Input<u8>) -> U8Result<usize> {
    parse!{i;
        option(cfws, ());
        let n = parse_digits((1..3));
        option(cfws, ());

        ret n
    }
}

// obs-month = CFWS month-name CFWS
pub fn obs_month(i: Input<u8>) -> U8Result<Month> {
    parse!{i;
        option(cfws, ());
        let m = month_name();
        option(cfws, ());

        ret m
    }
}

// obs-year = [CFWS] 2*DIGIT [CFWS]
pub fn obs_year(i: Input<u8>) -> U8Result<usize> {
    parse!{i;
        option(cfws, ());
        let y = or(
            |i| parse_digits(i, (2..)),
            obs_year,
            );
        option(cfws, ());

        ret y
    }
}

// obs-hour = [CFWS] 2DIGIT [CFWS]
pub fn obs_hour(i: Input<u8>) -> U8Result<usize> {
    parse!{i;
        option(cfws, ());
        let n = parse_digits(2);
        option(cfws, ());

        ret n
    }
}

// obs-minute = [CFWS] 2DIGIT [CFWS]
pub fn obs_minute(i: Input<u8>) -> U8Result<usize> {
    parse!{i;
        option(cfws, ());
        let n = parse_digits(2);
        option(cfws, ());

        ret n
    }
}

// obs-second = [CFWS] 2DIGIT [CFWS]
pub fn obs_second(i: Input<u8>) -> U8Result<usize> {
    parse!{i;
        option(cfws, ());
        let n = parse_digits(2);
        option(cfws, ());

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

// obs-local-part = word *("." word)
pub fn obs_local_part(i: Input<u8>) -> U8Result<&[u8]> {
    matched_by(i, |i| { parse!{i;
        word();
        skip_many(|i| parse!{i;
            token(b'.');
            word();

            ret ()
        });

        ret ()
    }}).bind(|i, v| i.ret(v.0))
}

// obs-domain = atom *("." atom)
pub fn obs_domain(i: Input<u8>) -> U8Result<&[u8]> {
    matched_by(i, |i| { parse!{i;
        atom();
        skip_many(|i| { parse!{i;
            token(b'.');
            atom();

            ret ()
        }});
    }}).map(|(v, _)| v)
}

// obs-mbox-list = 1*([mailbox] [CFWS] "," [CFWS]) [mailbox]
pub fn obs_mbox_list(i: Input<u8>) -> U8Result<Vec<Address>> {
    let r = parse!{i;
        let an:Vec<Option<Address>> = many1(|i| { parse!{i;
            let m: Option<Address> = option(|i| mailbox(i).map(|i| Some(i)), None);
            option(cfws, ());
            token(b',');
            option(cfws, ());

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
            option(cfws, ());
            token(b',');
            option(cfws, ());

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
pub fn obs_id_left(i: Input<u8>) -> U8Result<Vec<u8>> {
    local_part(i)
}

// obs-id-right    =       domain
pub fn obs_id_right(i: Input<u8>) -> U8Result<Vec<u8>> {
    domain(i)
}
