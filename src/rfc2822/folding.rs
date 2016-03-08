use chomp::*;

use rfc2822::obsolete::*;
use rfc2822::primitive::*;
use rfc2822::quoted::*;

// Folding white space
// FWS = ([*WSP CRLF] 1*WSP) / obs-FWS
// Consumes matches & returns ()
pub fn fws(i: Input<u8>) -> U8Result<()> {
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
pub fn ctext(i: Input<u8>) -> U8Result<()> {
    or(i,
       |i| no_ws_ctl(i).then(|i| i.ret(())),
       |i| satisfy(i, |i| (33 <= i && i <= 39) || (42 <= i && i <= 91) || (93 <= i && i <= 126)).then(|i| i.ret(())),
       )
}

// comment = "(" *([FWS] ccontent) [FWS] ")"
// Consumes matches & returns ()
pub fn comment(i: Input<u8>) -> U8Result<()> {
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
pub fn ccontent(i: Input<u8>) -> U8Result<()> {
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
pub fn cfws(i: Input<u8>) -> U8Result<()> {
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
