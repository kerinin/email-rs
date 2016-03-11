use chomp::*;

use rfc2822::obsolete::*;

pub fn cr(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |i| i == 13)
}

pub fn lf(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |i| i == 10)
}

pub fn dquote(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |i| i == 34)
}

pub fn crlf(i: Input<u8>) -> U8Result<&[u8]> {
    string(i, &[13,10][..])
}

// the space (SP, ASCII value 32) and horizontal tab (HTAB, ASCII value 9) characters
// (together known as the white space characters, WSP)
pub fn wsp(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |i| (i == 32) || (i == 9))
}

pub fn digit(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |i| 48 <= i && i <= 57)
}

pub fn alpha(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |i| (65 <= i && i <= 90) || (97 <= i && i <= 122))
}

// US-ASCII control characters that do not include the carriage return, 
// line feed, and white space characters
// NO-WS-CTL = %d1-8 / %d11 / %d12 / %d14-31 / %d127
pub fn no_ws_ctl(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |i| (1 <= i && i <= 8) || (i == 11) || (i == 12) || (14 <= i && i <= 31) || (i == 127))
}

// Characters excluding CR and LF
// text = %d1-9 / %d11 / %d12 / %d14-127 / obs-text 
pub fn text(i: Input<u8>) -> U8Result<u8> {
    parse!{i;
        or( 
            |i| satisfy(i, |c| (1 <= c && c <= 9) || (c == 11) || (c == 12) || (14 <= c && c <= 127)),
            obs_text,
            )}
}
