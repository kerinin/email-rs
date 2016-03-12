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

// local-part = dot-atom / quoted-string / obs-local-part
// NOTE: `quoted-string` includes `@` (as does obs-local-part, since it includes
// `quoted-string`)
pub fn local_part(i: Input<u8>) -> U8Result<Vec<u8>> {
    println!("local_part({:?})", i);
    let a = |i| {
        dot_atom(i).bind(|i, v| {
            println!("local_part.dot_atom.bind({:?}, {:?})", i, v);
            i.ret(FromIterator::from_iter(v.iter().map(|i| i.clone())))
        })
    };

    let b = |i| {
        quoted_string_not(i, |c| c == b'@')
    };

    let c = |i| {
        obs_local_part(i).bind(|i, v| {
            println!("local_part.obs_local_part.bind({:?}, {:?})", i, v);
            i.ret(FromIterator::from_iter(v.iter().map(|i| i.clone())))
        })
    };

    or(i, a, |i| or(i, b, c))
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
//
// NOTE: In some cases, dot-atom successfully matches a subset of the 
// "correct" value.
//
// To demonstrate, lets expand domain out (substituting dot-atom, dot-atom-text,
// obs-domain and atom):
//
// dot-atom = [CFWS] 1*atext *("." 1*atext) [CFWS]
// obs-domain = [CFWS] 1*atext [CFWS] *("." ([CFWS] 1*atext [CFWS]))
//
// As you can see, obs-domain is a superset of dot-atom.  We'll check it first
// yielding the effective pattern:
//
// domain = obs-domain / dot-atom / domain-literal
//
pub fn domain(i: Input<u8>) -> U8Result<Vec<u8>> {
    println!("domain({:?})", i);

    let a = |i| {
        dot_atom(i).bind(|i, v| {
            println!("domain.dot_atom.bind({:?}, {:?})", i, v);

            i.ret(FromIterator::from_iter(v.iter().map(|i| i.clone())))
        })
    };

    let b = |i| {
        domain_literal(i).bind(|i, v| {
            println!("domain.domain_literal.bind({:?}, {:?})", i, v);
            
            i.ret(v)
        })
    };

    let c = |i| {
        obs_domain(i).bind(|i, v| {
            println!("domain.obs_domain.bind({:?}, {:?})", i, v);

            i.ret(FromIterator::from_iter(v.iter().map(|i| i.clone())))
        })
    };

    or(i, c, |i| or(i, a, b))
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

// addr-spec = local-part "@" domain
pub fn addr_spec(i: Input<u8>) -> U8Result<Address> {
    println!("addr_spec({:?})", i);
    local_part(i).bind(|i, l| {
        println!("addr_spec.local_part.bind({:?}, {:?})", i, l);
        token(i, b'@').then(|i| {
            println!("addr_spec.token(@)");
            domain(i).bind(|i, d| {
                println!("addr_spec.domain.bind({:?})", d);

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
    // let i = b"@machine.tld:mary@example.net";
    // let msg = parse_only(addr_spec, i);
    // assert!(msg.is_ok());

    let i = b"pete(his account)@silly.test(his host)";
    let msg = parse_only(addr_spec, i);
    assert!(msg.is_ok());

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
    let i = b"<pete(his account)@silly.test(his host)>";
    let msg = parse_only(angle_addr, i);
    assert!(msg.is_ok());

    let i = b"<jdoe@machine.example>";
    let msg = parse_only(angle_addr, i);
    assert!(msg.is_ok());
}

// name-addr = [display-name] angle-addr
//
// NOTE: In some cases, display-name successfully matches a subset of the 
// "correct" value.  We need to ensure that [CFWS] "<" follows the match.
//
// To demonstrate, lets expand display-name out:
//
// display-name = phrase
// phrase = 1*word / obs-phrase
// word = atom / quoted-string
// atom = [CFWS] 1*atext [CFWS]
// atext           =       ALPHA / DIGIT / ; Any character except controls,
//                         "!" / "#" /     ;  SP, and specials.
//                         "$" / "%" /     ;  Used for atoms
//                         "&" / "'" /
//                         "*" / "+" /
//                         "-" / "/" /
//                         "=" / "?" /
//                         "^" / "_" /
//                         "`" / "{" /
//                         "|" / "}" /
//                         "~"
// obs-phrase = word *(word / "." / CFWS)
//
// So specifically, foo.bar <foo@example.com> fails to match becasue "foo" 
// matches phrase, but ".bar <foo@example.com>" does not match angle-addr.
//
// To "fix" this, I'm using obs-phrase rather than display-name.  obs-phrase is
// a superset of 1*word, which means it covers phrase, and therefore display-name.
// The effective pattern is:
//
// name-addr = [obs-phrase] angle-addr
//
pub fn name_addr(i: Input<u8>) -> U8Result<Address> {
    println!("name_addr({:?})", i);

    option(i, |i| obs_phrase(i).map(|v| Some(v)), None).bind(|i, n| {
        println!("name_addr.obs_phrase.bind({:?}, {:?})", i, n);
        
        angle_addr(i).bind(|i, a| {
            println!("name_addr.angle_addr.bind({:?}, {:?})", i, a);

            match a {
                Address::Mailbox{local_part: l, domain: d, display_name: _} => {
                    let mb = Address::Mailbox{
                        local_part: l,
                        domain: d,
                        display_name: n,
                    };
                    println!("-> name_addr({:?})", mb);
                    i.ret(mb)
                },
                // NOTE: This _would_ be unexpected, as `display_name` should always
                // return an Address::Mailbox
                _ => i.err(Error::Unexpected),
            }
        })
    })
}

#[test]
fn test_name_addr() {
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

// mailbox = name-addr / addr-spec
pub fn mailbox(i: Input<u8>) -> U8Result<Address> {
    println!("mailbox({:?})", i);
    or(i, name_addr, addr_spec).bind(|i, v| {
        println!("-> mailbox({:?})", v);
        i.ret(v)
    })
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

// group           =       display-name ":" [mailbox-list / CFWS] ";"
//                         [CFWS]
//
// NOTE: Using obs-phrase in place of display-name - see comment on name-addr
// for more detail.  Effective pattern:
//
// group           =       obs-phrase ":" [mailbox-list / CFWS] ";" [CFWS]
pub fn group(i: Input<u8>) -> U8Result<Address> {
    println!("group({:?})", i);

    obs_phrase(i).bind(|i, n| {
        println!("group.obs_phrase.bind({:?})", n);

        token(i, b':').then(|i| {
            println!("group.token(:)");

            let list_or_none = |i| {
                or(i, mailbox_list, |i| cfws(i).map(|_| vec!()))
            };

            option(i, list_or_none, vec!()).bind(|i, ms| {
                println!("group.option(list_or_none).bind({:?}, {:?})", i, ms);

                token(i, b';').then(|i| {
                    println!("group.token(;)");

                    option(i, cfws, ()).then(|i| {
                        println!("group.option(cfws)");

                        let g = Address::Group{
                            // NOTE: Encoding?
                            display_name: String::from_utf8(n).unwrap(),
                            mailboxes: ms,
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


// address = mailbox / group
pub fn address(i: Input<u8>) -> U8Result<Address> {
    println!("address({:?})", i);
    or(i, mailbox, group).bind(|i, v| {
        println!("-> address({:?})", v);
        i.ret(v)
    })
}

// address-list    =       (address *("," address)) / obs-addr-list
//
// NOTE: Accepting obsolete syntax first as it's more generic
pub fn address_list(i: Input<u8>) -> U8Result<Vec<Address>> {
    println!("address_list({:?})", i);
    let a = |i| sep_by1(i, address, |i| token(i, b','));

    or(i, obs_addr_list, a).bind(|i, v| {
        println!("-> address_list({:?})", v);
        i.ret(v)
    })
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
