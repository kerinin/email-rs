use std::iter::FromIterator;

use chomp::*;
use rfc2822::*;
use rfc2822::atom::*;
use rfc2822::folding::*;
use rfc2822::misc::*;
use rfc2822::obsolete::*;
use rfc2822::primitive::*;
use rfc2822::quoted::*;

// display-name = phrase
pub fn display_name(i: Input<u8>) -> U8Result<Vec<u8>> {
    phrase(i)
}

#[test]
fn test_display_name() {
    let i = b"John Doe";
    let msg = parse_only(display_name, i);
    assert!(msg.is_ok());
}

// local-part = dot-atom / quoted-string / obs-local-part
// NOTE: `quoted-string` includes `@` (as does obs-local-part, since it includes
// `quoted-string`)
pub fn local_part(i: Input<u8>) -> U8Result<Vec<u8>> {
    let a = |i| {
        dot_atom(i).bind(|i, v| {
            i.ret(FromIterator::from_iter(v.iter().map(|i| i.clone())))
        })
    };

    let b = |i| {
        quoted_string_not(i, |c| c == b'@')
    };

    let c = |i| {
        obs_local_part(i).bind(|i, v| {
            i.ret(FromIterator::from_iter(v.iter().map(|i| i.clone())))
        })
    };

    or(i, a, |i| or(i, b, c))
}

// dtext           =       NO-WS-CTL /     ; Non white space controls
//
//                         %d33-90 /       ; The rest of the US-ASCII
//                         %d94-126        ;  characters not including "[",
//                                         ;  "]", or "\"
pub fn dtext(i: Input<u8>) -> U8Result<u8> {
    or(i, 
       no_ws_ctl,
       |i| satisfy(i, |i| (33 <= i && i <= 90) || (94 <= i && i <= 126)),
       )
}

// dcontent = dtext / quoted-pair
pub fn dcontent(i: Input<u8>) -> U8Result<u8> {
    or(i, dtext, quoted_pair)
}

// domain-literal = [CFWS] "[" *([FWS] dcontent) [FWS] "]" [CFWS]
pub fn domain_literal(i: Input<u8>) -> U8Result<Vec<u8>> {
    parse!{i;
        option(cfws, ());
        token(b'[');
        let cs = many(|i| { parse!{i;
            option(fws, ());
            dcontent()
        }});
        option(fws, ());
        token(b']');
        option(cfws, ());

        ret cs
    }
}

// domain = dot-atom / domain-literal / obs-domain
pub fn domain(i: Input<u8>) -> U8Result<Vec<u8>> {
    or(i,
       |i| dot_atom(i).bind(|i, v| i.ret(FromIterator::from_iter(v.iter().map(|i| i.clone())))),
       |i| or(i,
              domain_literal,
              |i| obs_domain(i).bind(|i, v| i.ret(FromIterator::from_iter(v.iter().map(|i| i.clone())))),
              ))
}

#[test]
fn test_domain() {
    let i = b"machine.example";
    let msg = parse_only(domain, i);
    assert!(msg.is_ok());
}

// addr-spec = local-part "@" domain
pub fn addr_spec(i: Input<u8>) -> U8Result<Address> {
    local_part(i).bind(|i, l| {
        token(i, b'@').then(|i| {
            domain(i).bind(|i, d| {
                i.ret( Address::Mailbox{
                    local_part: String::from_utf8(l).unwrap(), 
                    domain: String::from_utf8(d).unwrap(),
                    display_name: None,
                })
            })
        })
    })
}

#[test]
fn test_addr_spec() {
    let i = b"jdoe@machine.example";
    let msg = parse_only(addr_spec, i);
    assert!(msg.is_ok());
}

// angle-addr = [CFWS] "<" addr-spec ">" [CFWS] / obs-angle-addr
// NOTE: Omitting `obs-angle-addr` becasue there be dragons - this is technically
// a legal email: <@foo.com@bar.com,@baz.con:me@example.com>
pub fn angle_addr(i: Input<u8>) -> U8Result<Address> {
    option(i, cfws, ()).then(|i| {
        token(i, b'<').then(|i| {
            addr_spec(i).bind(|i, a| {
                token(i, b'>').then(|i| {
                    option(i, cfws, ()).then(|i| {
                        i.ret(a)
                    })
                })
            })
        })
    })
}

#[test]
fn test_angle_addr() {
    let i = b"<jdoe@machine.example>";
    let msg = parse_only(angle_addr, i);
    assert!(msg.is_ok());
}

// name-addr = [display-name] angle-addr
pub fn name_addr(i: Input<u8>) -> U8Result<Address> {
    let r = parse!{i;
        let n = display_name();
        let a = angle_addr();

        ret (n, a)
    };

    r.bind(|i, (r, a)| {
        match a {
            Address::Mailbox{local_part: l, domain: d, display_name: _} => i.ret(Address::Mailbox{
                local_part: l,
                domain: d,
                // NOTE: Encoding?
                display_name: Some(String::from_utf8(r).unwrap()),
            }),
            // NOTE: This _would_ be unexpected, as `display_name` should always
            // return an Address::Mailbox
            _ => i.err(Error::Unexpected),
        }
    })
}

#[test]
fn test_name_addr() {
    let i = b"John Doe <jdoe@machine.example>";
    let msg = parse_only(name_addr, i);
    assert!(msg.is_ok());
}

// mailbox = name-addr / addr-spec
pub fn mailbox(i: Input<u8>) -> U8Result<Address> {
    or(i, name_addr, addr_spec)
}

// mailbox-list = (mailbox *("," mailbox)) / obs-mbox-list
pub fn mailbox_list(i: Input<u8>) -> U8Result<Vec<Address>> {
    or(i,
       |i| parse!{i;
           let m1: Address = mailbox();
           let ms: Vec<Address> = many(|i| { parse!{i;
               token(b',');
               mailbox()
           }});

           ret {
               let mut v = Vec::with_capacity(ms.len() + 1);
               v.push(m1.clone());
               for m in ms.iter() {
                   v.push(m.clone())
               }
               v
           }
       },
       obs_mbox_list,
       )
}
#[test]
fn test_mailbox_list() {
    let i = b"John Doe <jdoe@machine.example>";
    let msg = parse_only(mailbox_list, i);
    assert!(msg.is_ok());
}

// group           =       display-name ":" [mailbox-list / CFWS] ";"
//                         [CFWS]
pub fn group(i: Input<u8>) -> U8Result<Address> {
    parse!{i;
        let n = display_name();
        token(b':');
        let ms = option(|i| or(i,
                               mailbox_list, 
                               |i| cfws(i).then(|i| i.ret(vec!())),
                               ), vec!());
        token(b';');

        ret Address::Group{
            // NOTE: Encoding?
            display_name: String::from_utf8(n).unwrap(),
            mailboxes: ms,
        }
    }
}


// address = mailbox / group
pub fn address(i: Input<u8>) -> U8Result<Address> {
    or(i, mailbox, group)
}

// address-list    =       (address *("," address)) / obs-addr-list
pub fn address_list(i: Input<u8>) -> U8Result<Vec<Address>> {
    or(i, |i| sep_by1(i, address, |i| token(i, b',')), obs_addr_list)
}
