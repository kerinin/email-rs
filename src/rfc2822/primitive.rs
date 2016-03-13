use chomp::*;

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
const WSP: [bool; 256] = [
    //  0      1      2      3      4      5      6      7      8      9     10     11     12     13     14     15     16     17     18     19
    false, false, false, false, false, false, false, false, false, true,  false, false, false, false, false, false, false, false, false, false, //   0 -  19
    false, false, false, false, false, false, false, false, false, false, false, false, true,  false, false, false, false, false, false, false, //  20 -  39
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, //  40 -  59
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, //  60 -  79
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, //  80 -  99
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 100 - 119
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 120 - 139
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 140 - 159
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 160 - 179
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 180 - 199
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 200 - 219
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 220 - 239
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false                              // 240 - 256
];
pub fn wsp(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |c| WSP[c as usize])
}

const DIGIT: [bool; 256] = [
    //  0      1      2      3      4      5      6      7      8      9     10     11     12     13     14     15     16     17     18     19
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, //   0 -  19
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, //  20 -  39
    false, false, false, false, false, false, false, false, true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false, false, //  40 -  59
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, //  60 -  79
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, //  80 -  99
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 100 - 119
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 120 - 139
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 140 - 159
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 160 - 179
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 180 - 199
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 200 - 219
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 220 - 239
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false                              // 240 - 256
];
pub fn digit(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |c| DIGIT[c as usize])
}

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
pub fn alpha(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |c| ALPHA[c as usize])
}

// US-ASCII control characters that do not include the carriage return, 
// line feed, and white space characters
// NO-WS-CTL = %d1-8 / %d11 / %d12 / %d14-31 / %d127
const NO_WS_CTL: [bool; 256] = [
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
pub fn no_ws_ctl(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |c| NO_WS_CTL[c as usize])
}

// Characters excluding CR and LF
// text = %d1-9 / %d11 / %d12 / %d14-127 / obs-text 
// obs-text = *LF *CR *(obs-char *LF *CR)
// obs-char = %d0-9 / %d11 / %d12 / %d14-127
// Expanding to:
//
// text = %d0-9 / %d10 / %d11 / %d12 / %13 %d14-127
//      = %d0-127
const TEXT: [bool; 256] = [
    //  0      1      2      3      4      5      6      7      8      9     10     11     12     13     14     15     16     17     18     19
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  //   0 -  19
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
pub fn text(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |c| TEXT[c as usize])
}

/*
mod bench {
    extern crate test;

    use self::test::Bencher;
    use chomp::*;
    use super::*;

    #[bench]
    fn bench_text(b: &mut Bencher) {
        let raw: Vec<u8> = (0..10000).map(|i| (i % 128) as u8).collect();
        let input = test::black_box(&raw[..]);

        b.iter(|| {
            let p: Result<Vec<u8>, _> = parse_only(|i| many(i, text), input);
            p
        })
    }
}
*/
