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
               skip_many(wsp);
               crlf();

               ret ()
           }}, ());
           skip_many1(wsp);
       }},
       obs_fws,
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
            option(fws, ());
            ccontent()
        }} );
        option(fws, ());
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
              comment,
              )
       },)
}

fn fws_comment(i: Input<u8>) -> U8Result<()> {
    // println!("fws_comment({:?})", i);
    option(i, fws, ()).then(|i| {
        // println!("fws_comment.option(fws).then({:?})", i);
        comment(i)
    })
}

// CFWS = *([FWS] comment) (([FWS] comment) / FWS)
//
// This is tricky for a greedy matcher.
// What's happening here is that *([FWS] comment) is consuming all the instances
// of that pattern, and then the ([FWS] comment) fails
//
// This should be equivalent:
// CFWS = 1*([FWS] comment) [FWS] / (*([FWS] comment) FWS)
//
pub fn cfws(i: Input<u8>) -> U8Result<()> {
    // println!("cfws({:?})", i);

    let repeat = |i| {
        many1(i, fws_comment).map(|_: Vec<()>| ()).then(|i| {
            option(i, fws, ()).then(|i| {
                // println!("cfws.repeat");

                i.ret(())
            })
        })
    };

    let fws_term = |i| {
        many(i, fws_comment).map(|_: Vec<()>| ()).then(|i| {
            // println!("cfws.fws_term");
            fws(i)
        })
    };

    or(i, repeat, fws_term).then(|i| {
        // println!("-> cfws.or(repeat, fws_term).then({:?})", i);
        i.ret(())
    })
}

#[test]
fn test_cfws() {
    let i = b"(his account)";
    let msg = parse_only(cfws, i);
    assert!(msg.is_ok());

    let i = b"\r\n ";
    let msg = parse_only(cfws, i);
    assert!(msg.is_ok());

    let i = b"(comment)\r\n ";
    let msg = parse_only(cfws, i);
    assert!(msg.is_ok());

    let i = b"(comment)(comment)\r\n ";
    let msg = parse_only(cfws, i);
    assert!(msg.is_ok());

    let i = b"(comment)\r\n (comment)\r\n ";
    let msg = parse_only(cfws, i);
    assert!(msg.is_ok());

    let i = b"\r\n (comment)\r\n (comment)\r\n ";
    let msg = parse_only(cfws, i);
    assert!(msg.is_ok());
}
