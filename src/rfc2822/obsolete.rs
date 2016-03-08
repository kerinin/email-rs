use chomp::*;

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
        skip_many(|i| lf(i));
        skip_many(|i| cr(i));
        let c = obs_char();
        skip_many(|i| lf(i));
        skip_many(|i| cr(i));

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
        skip_many1(|i| wsp(i));
        skip_many(|i| parse!{i;
            crlf();
            skip_many1(|i| wsp(i));
        })
    }
}

// obs-phrase = word *(word / "." / CFWS)
pub fn obs_phrase(i: Input<u8>) -> U8Result<Vec<u8>> {
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
