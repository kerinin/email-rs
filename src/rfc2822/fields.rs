use std::str;

use chomp::*;
use bytes::{Bytes, ToBytes, ByteStr};

use rfc2822::*;
use rfc2822::address::*;
use rfc2822::atom::*;
use rfc2822::datetime::*;
use rfc2822::folding::*;
use rfc2822::misc::*;
use rfc2822::obsolete::*;
use rfc2822::primitive::*;
use rfc2822::quoted::*;

// orig-date       =       "Date:" date-time CRLF
pub fn orig_date(i: Input<u8>) -> U8Result<Field> {
    println!("orig_date({:?})", i);
    
    string(i, b"Date:").then(|i| {
        println!("orig_date.string(Date:).then({:?})", i);

        date_time(i).bind(|i, d| {
            println!("orig_date.date_time.bind({:?}, {:?})", i, d);

            crlf(i).then(|i| {
                println!("orig_date.crlf.then({:?})", i);

                i.ret(Field::Date(DateTimeField{date_time: d}))
            })
        })
    })
}

#[test]
fn test_orig_date() {
    // NOTE: Need to support obsolete fields syntax for this
    // let i = b"Date  : Fri, 21 Nov 1997 09(comment):   55  :  06 -0600\r\n";
    // let msg = parse_only(orig_date, i);
    // assert!(msg.is_ok());

    let i = b"Date: 21 Nov 97 09:55:06 GMT\r\n";
    let msg = parse_only(orig_date, i);
    assert!(msg.is_ok());

    let i = b"Date: Tue, 1 Jul 2003 10:52:37 +0200\r\n"; 
    let msg = parse_only(orig_date, i);
    assert!(msg.is_ok());

    let i = b"Date: Thu,\r\n      13\r\n        Feb\r\n          1969\r\n 23:32\r\n               -0330 (Newfoundland Time)\r\n";
    let msg = parse_only(orig_date, i);
    assert!(msg.is_ok());

    let i = b"Date: Fri, 21 Nov 1997 09:55:06 -0600\r\n";
    let msg = parse_only(orig_date, i);
    assert!(msg.is_ok());
}

// from            =       "From:" mailbox-list CRLF
pub fn from(i: Input<u8>) -> U8Result<Field> {
    println!("from({:?})", i);

    string(i, b"From:").then(|i| {
        println!("from.string(From:).then({:?})", i);
        mailbox_list(i).bind(|i, l| {
            println!("from.mailbox_list.bind({:?}, {:?})", i, l);
            crlf(i).then(|i| {
                println!("from.crlf.then({:?})", i);
                println!("-> from({:?})", l);
                i.ret(Field::From(AddressesField{addresses: l}))
            })
        })
    })
}

#[test]
fn test_from() {
    // NOTE: Need to support obsolete fields syntax for this
    // let i = b"From  : John Doe <jdoe@machine(comment).  example>\r\n";
    // let msg = parse_only(from, i);
    // assert!(msg.is_ok());

    let i = b"From: Joe Q. Public <john.q.public@example.com>\r\n";
    let msg = parse_only(from, i);
    assert!(msg.is_ok());

    let i = b"From: John Doe <jdoe@machine.example>\r\n";
    let msg = parse_only(from, i);
    assert!(msg.is_ok());

    let i = b"From: Pete(A wonderful \\) chap) <pete(his account)@silly.test(his host)>\r\n";
    let msg = parse_only(from, i);
    assert!(msg.is_ok());
}

// sender          =       "Sender:" mailbox CRLF
pub fn sender(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"Sender:");
        let l = mailbox();
        crlf();

        ret Field::Sender(AddressField{address: l})
    }
}

// reply-to        =       "Reply-To:" address-list CRLF
pub fn reply_to(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"Reply-To:");
        let l = address_list();
        crlf();

        ret Field::ReplyTo(AddressesField{addresses: l})
    }
}

// to              =       "To:" address-list CRLF
pub fn to(i: Input<u8>) -> U8Result<Field> {
    println!("to({:?})", i);

    string(i, b"To:").then(|i| {
        println!("to.string(To:).then({:?})", i);

        address_list(i).bind(|i, l| {
            println!("to.address_list.bind({:?}, {:?})", i, l);

            crlf(i).then(|i| {
                println!("to.crlf.then({:?})", i);
                println!("-> to({:?})", l);

                i.ret(Field::To(AddressesField{addresses: l}))
            })
        })
    })
}

#[test]
fn test_to() {
    // NOTE: Need to support obsolete fields syntax for this
    // let i = b"To    : Mary Smith\r\n__\r\n          <mary@example.net>\r\n";
    // let msg = parse_only(to, i);
    // assert!(msg.is_ok());

    let i = b"To: Mary Smith <mary@example.net>, , jdoe@test   . example\r\n";
    let msg = parse_only(to, i);
    assert!(msg.is_ok());

    let i = b"To:A Group(Some people)\r\n     :Chris Jones <c@(Chris's host.)public.example>,\r\n         joe@example.org,\r\n  John <jdoe@one.test> (my dear friend); (the end of the group)\r\n";
    let msg = parse_only(to, i);
    assert!(msg.is_ok());

    let i = b"To: Mary Smith <mary@example.net>\r\n";
    let msg = parse_only(to, i);
    assert!(msg.is_ok());
}

// cc              =       "Cc:" address-list CRLF
pub fn cc(i: Input<u8>) -> U8Result<Field> {
    println!("cc({:?})", i);

    string(i, b"Cc:").then(|i| {
        println!("cc.string(Cc:).then({:?})", i);

        address_list(i).bind(|i, l| {
            println!("cc.address_list.bind({:?}, {:?})", i, l);

            crlf(i).then(|i| {
                println!("cc.crlf.then({:?})", i);
                println!("-> cc({:?})", l);

                i.ret(Field::Cc(AddressesField{addresses: l}))
            })
        })
    })
}

#[test]
fn test_cc() {
    let i = b"Cc:(Empty list)(start)Undisclosed recipients  :(nobody(that I know))  ;\r\n";
    let msg = parse_only(cc, i);
    assert!(msg.is_ok());
}

// bcc             =       "Bcc:" (address-list / [CFWS]) CRLF
pub fn bcc(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"Bcc:");
        let l = address_list();
        crlf();

        ret Field::Bcc(AddressesField{addresses: l})
    }
}

// no-fold-quote   =       DQUOTE *(qtext / quoted-pair) DQUOTE
pub fn no_fold_quote(i: Input<u8>) -> U8Result<Bytes> {
    dquote(i).then(|i| {
        many(i, |i| or(i, qtext, quoted_pair)).bind(|i, t: Vec<u8>| {
            dquote(i).then(|i| {
                i.ret(Bytes::from_slice(&t[..]))
            })
        })
    })
}

// id-left         =       dot-atom-text / no-fold-quote / obs-id-left
pub fn id_left(i: Input<u8>) -> U8Result<Bytes> {
    or(i, dot_atom_text, |i| or(i, no_fold_quote, obs_id_left))
}

// no-fold-literal =       "[" *(dtext / quoted-pair) "]"
pub fn no_fold_literal(i: Input<u8>) -> U8Result<Bytes> {
    token(i, b'[').then(|i| {
        many(i, |i| or(i, dtext, quoted_pair)).bind(|i, t: Vec<u8>| {
            token(i, b']').then(|i| {
                i.ret(Bytes::from_slice(&t[..]))
            })
        })
    })
}

// id-right        =       dot-atom-text / no-fold-literal / obs-id-right
pub fn id_right(i: Input<u8>) -> U8Result<Bytes> {
    or(i, dot_atom_text, |i| or(i, no_fold_literal, obs_id_right))
}

// msg-id          =       [CFWS] "<" id-left "@" id-right ">" [CFWS]
pub fn msg_id(i: Input<u8>) -> U8Result<MessageID> {
    parse!{i;
        option(cfws, Bytes::empty());
        token(b'<');
        let l = id_left();
        token(b'@');
        let r = id_right();
        token(b'>');
        option(cfws, Bytes::empty());

        ret MessageID{id_left: l, id_right: r}
    }
}

// message-id      =       "Message-ID:" msg-id CRLF
pub fn message_id(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"Message-ID:");
        let id = msg_id();
        crlf();

        ret Field::MessageID(MessageIDField{message_id: id})
    }
}

#[test]
fn test_message_id() {
    // NOTE: Need to support obsolete fields syntax for this
    // let i = b"Message-ID  : <1234   @   local(blah)  .machine .example>\r\n";
    // let msg = parse_only(message_id, i);
    // assert!(msg.is_ok());

    let i = b"Message-ID: <5678.21-Nov-1997@example.com>\r\n";
    let msg = parse_only(message_id, i);
    assert!(msg.is_ok());

    let i = b"Message-ID: <1234@local.machine.example>\r\n";
    let msg = parse_only(message_id, i);
    assert!(msg.is_ok());
}

// in-reply-to     =       "In-Reply-To:" 1*msg-id CRLF
pub fn in_reply_to(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"In-Reply-To:");
        let ids = many1(msg_id);
        crlf();

        ret Field::InReplyTo(MessageIDsField{message_ids: ids})
    }
}

// references      =       "References:" 1*msg-id CRLF
pub fn references(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"References:");
        let ids = many1(msg_id);
        crlf();

        ret Field::References(MessageIDsField{message_ids: ids})
    }
}

// subject         =       "Subject:" unstructured CRLF
pub fn subject(i: Input<u8>) -> U8Result<Field> {
    // println!("subject({:?})", i);
    string(i, b"Subject:").then(|i| {
        // println!("subject.string(Subject:).then({:?})", i);
        unstructured(i).bind(|i, u| {
            // println!("subject.unstructured.bind({:?}, {:?})", i, u);
            crlf(i).then(|i| {
                // println!("subject.crlf.then({:?})", i);
                // println!("-> subject({:?})", u);

                i.ret(Field::Subject(UnstructuredField{data: u.to_bytes()}))
            })
        })
    })
}

#[test]
fn test_subject() {
    // NOTE: Need to support obsolete fields syntax for this
    // let i = b"To    : Mary Smith\r\n";
    // let msg = parse_only(subject, i);
    // assert!(msg.is_ok());

    let i = b"Subject: Saying Hello\r\n";
    let msg = parse_only(subject, i);
    assert!(msg.is_ok());
}

// comments        =       "Comments:" unstructured CRLF
pub fn comments(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"Comments:");
        let u = unstructured();
        crlf();

        ret Field::Comments(UnstructuredField{data: u.to_bytes()})
    }
}

// keywords        =       "Keywords:" phrase *("," phrase) CRLF
pub fn keywords(i: Input<u8>) -> U8Result<Field> {
    string(i, b"Keywords:").then(|i| {
        sep_by1(i, phrase, |i| token(i, b',')).bind(|i, kws: Vec<Bytes>| {
            crlf(i).then(|i| {
                i.ret(Field::Keywords(KeywordsField{keywords: kws}))
            })
        })
    })
}

// resent-date     =       "Resent-Date:" date-time CRLF
pub fn resent_date(i: Input<u8>) -> U8Result<Resent> {
    parse!{i;
        string(b"Resent-Date:");
        let d = date_time();
        crlf();

        ret Resent::Date(DateTimeField{date_time: d})
    }
}

// resent-from     =       "Resent-From:" mailbox-list CRLF
pub fn resent_from(i: Input<u8>) -> U8Result<Resent> {
    parse!{i;
        string(b"Resent-From:");
        let l = mailbox_list();
        crlf();

        ret Resent::From(AddressesField{addresses: l})
    }
}

// resent-sender   =       "Resent-Sender:" mailbox CRLF
pub fn resent_sender(i: Input<u8>) -> U8Result<Resent> {
    parse!{i;
        string(b"Resent-Sender:");
        let l = mailbox();
        crlf();

        ret Resent::Sender(AddressField{address: l})
    }
}

// resent-to       =       "Resent-To:" address-list CRLF
pub fn resent_to(i: Input<u8>) -> U8Result<Resent> {
    parse!{i;
        string(b"Resent-To:");
        let l = address_list();
        crlf();

        ret Resent::To(AddressesField{addresses: l})
    }
}

// resent-cc       =       "Resent-Cc:" address-list CRLF
pub fn resent_cc(i: Input<u8>) -> U8Result<Resent> {
    parse!{i;
        string(b"Resent-Cc:");
        let l = address_list();
        crlf();

        ret Resent::Cc(AddressesField{addresses: l})
    }
}

// resent-bcc      =       "Resent-Bcc:" (address-list / [CFWS]) CRLF
pub fn resent_bcc(i: Input<u8>) -> U8Result<Resent> {
    parse!{i;
        string(b"Resent-Bcc:");
        let l = address_list();
        crlf();

        ret Resent::Bcc(AddressesField{addresses: l})
    }
}

// resent-msg-id   =       "Resent-Message-ID:" msg-id CRLF
pub fn resent_msg_id(i: Input<u8>) -> U8Result<Resent> {
    parse!{i;
        string(b"Resent-Message-ID:");
        let id = msg_id();
        crlf();

        ret Resent::MessageID(MessageIDField{message_id: id})
    }
}

// path            =       ([CFWS] "<" ([CFWS] / addr-spec) ">" [CFWS]) /
//                         obs-path
// NOTE: this allows "<>" as a valid match which is sort of useless, and the
// obs-path definition is janky, so I'm going to modify the pattern to be:
//
// real-path       =       [CFWS] "<" [CFWS] addr-spec ">" [CFWS]
pub fn path(i: Input<u8>) -> U8Result<Address> {
    parse!{i; 
        option(cfws, Bytes::empty());
        token(b'<');
        option(cfws, Bytes::empty());
        let a = addr_spec();
        token(b'>');
        option(cfws, Bytes::empty());

        ret a
    }
}

// return-path     =       "Return-Path:" path CRLF
pub fn return_path(i: Input<u8>) -> U8Result<Address> {
    parse!{i;
        string(b"Return-Path:");
        let p = path();
        crlf();

        ret p
    }
}

// item-name       =       ALPHA *(["-"] (ALPHA / DIGIT))
pub fn item_name(i: Input<u8>) -> U8Result<&[u8]> {
    matched_by(i, |i| parse!{i;
        alpha();
        skip_many(|i| parse!{i;
            option(|i| token(i, b'-'), b'_');
            or(alpha, digit);
        });
    }).map(|(v, _)| v)
}

// item-value      =       1*angle-addr / addr-spec /
//                          atom / domain / msg-id
// pub fn item_value(i: Input<u8>) -> U8Result<ReceivedValue> {
//     or(i, 
//        |i| many1(i, angle_addr).map(|a| ReceivedValue::Addresses(a)),
//        |i| or(i, |i| addr_spec(i).map(|a| ReceivedValue::Address(a)),
//        |i| or(i, |i| atom(i).map(|v| ReceivedValue::Text(FromIterator::from_iter(v.iter().map(|i| i.clone())))),
//        |i| or(i, |i| domain(i).map(|v| ReceivedValue::Domain(v)), 
//               |i| msg_id(i).map(|v| ReceivedValue::MessageID(v))))))
// }

// name-val-pair   =       item-name CFWS item-value
// pub fn name_val_pair(i: Input<u8>) -> U8Result<(&[u8], ReceivedValue)> {
//     println!("name_val_pair({:?})", i);
//     item_name(i).bind(|i, n| {
//         println!("name_val_pair.item_name.bind({:?})", n);
//         cfws(i).then(|i| {
//             println!("name_val_pair.cfws");
//             item_value(i).bind(|i, v| {
//                 println!("name_val_pair.item_value.bind({:?})", v);
//
//                 i.ret((n, v))
//             })
//         })
//     })
// }

// name-val-list   =       [CFWS] [name-val-pair *(CFWS name-val-pair)]
// pub fn name_val_list(i: Input<u8>) -> U8Result<Vec<(&[u8], ReceivedValue)>> {
//     println!("name_val_list({:?})", i);
//     cfws(i).then(|i| {
//         println!("name_val_list.cfws.then");
//         sep_by(i, name_val_pair, cfws).bind(|i, list| {
//             println!("name_val_list.sep_by.bind({:?})", list);
//
//             i.ret(list)
//         })
//     })
// }

// received        =       "Received:" name-val-list ";" date-time CRLF
// NOTE: This field is more complex than I feel like supporting - punting on
// parsing its contents.  Effective match is:
// received        =       "Received:" *(%d0-58 / %d60-255) ";" date-time CRLF
pub fn received(i: Input<u8>) -> U8Result<ReceivedField> {
    // println!("received({:?})", i);
    string(i, b"Received:").then(|i| {
        // println!("received.string(Received:).then");

        take_till(i, |c| c == b';').bind(|i, v: &[u8]| {
            token(i, b';').then(|i| {
                // println!("received.token.then");
                date_time(i).bind(|i, dt| {
                    // println!("received.date_time.bind({:?})", dt);
                    crlf(i).then(|i| {
                        // println!("received.crlf.then");

                        let r = ReceivedField{date_time: dt, data: Bytes::from_slice(v)};

                        i.ret(r)
                    })
                })
            })
        })
    })
}

#[test]
fn test_received() {
    let i = b"Received: from machine.example by x.y.test; 21 Nov 1997 10:01:22 -0600\r\n";
    let msg = parse_only(received, i);
    assert!(msg.is_ok());

    let i = b"Received: from x.y.test\r\n   by example.net\r\n   via TCP\r\n   with ESMTP\r\n   id ABC12345\r\n   for <mary@example.net>;  21 Nov 1997 10:05:43 -0600\r\n";
    let msg = parse_only(received, i);
    assert!(msg.is_ok());
}

// trace           =       [return-path] 1*received
pub fn trace(i: Input<u8>) -> U8Result<(Option<Address>, Vec<ReceivedField>)> {
    println!("trace({:?})", i);
    option(i, |i| {
        return_path(i).map(|r| Some(r))
    }, None).bind(|i, rp| {
        println!("trace.option(return_path).bind({:?}, {:?})", i, rp);
        many1(i, received).bind(|i, rs| {
            println!("trace.many1(received).bind({:?}, {:?})", i, rs);
            println!("-> trace({:?}, {:?})", rp, rs);

            i.ret((rp, rs))
        })
    })
}

// ftext           =       %d33-57 /               ; Any character except
//                         %d59-126                ;  controls, SP, and
//                                                 ;  ":".
pub fn ftext(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |i| (33 <= i && i <= 57) || (59 <= i && i <= 126))
}

// field-name      =       1*ftext
pub fn field_name(i: Input<u8>) -> U8Result<Bytes> {
    matched_by(i, |i| skip_many1(i, ftext)).map(|(v, _)| Bytes::from_slice(v))
}

// optional-field  =       field-name ":" unstructured CRLF
pub fn optional_field(i: Input<u8>) -> U8Result<Field> {
    // println!("optional_field({:?})", i);

    field_name(i).bind(|i, n| {
        // println!("optional_field.field_name.bind({:?}, {:?})", i, n);

        unstructured(i).bind(|i, v| {
            // println!("optional_field.unstructured.bind({:?}, {:?})", i, v);

            crlf(i).then(|i| {
                // println!("optional_field.crlf.then({:?})", i);
                // println!("-> optional({:?}, {:?})", n, v);

                i.ret(Field::Optional(str::from_utf8(n.buf().bytes()).unwrap().to_string(), UnstructuredField{data: v}))
            })
        })
    })
}

// fields          =       *(trace
//                           *(resent-date /
//                            resent-from /
//                            resent-sender /
//                            resent-to /
//                            resent-cc /
//                            resent-bcc /
//                            resent-msg-id))
//                         *(orig-date /
//                         from /
//                         sender /
//                         reply-to /
//                         to /
//                         cc /
//                         bcc /
//                         message-id /
//                         in-reply-to /
//                         references /
//                         subject /
//                         comments /
//                         keywords /
//                         optional-field)
//
//
// Field           Min number      Max number      Notes
// ---------------+---------------+---------------+-----
// trace           0               unlimited       Block prepended - see
//                                                 3.6.7
// resent-date     0*              unlimited*      One per block, required
//                                                 if other resent fields
//                                                 present - see 3.6.6
// resent-from     0               unlimited*      One per block - see
//                                                 3.6.6
// resent-sender   0*              unlimited*      One per block, MUST
//                                                 occur with multi-address
//                                                 resent-from - see 3.6.6
// resent-to       0               unlimited*      One per block - see
//                                                 3.6.6
// resent-cc       0               unlimited*      One per block - see
//                                                 3.6.6
// resent-bcc      0               unlimited*      One per block - see
//                                                 3.6.6
// resent-msg-id   0               unlimited*      One per block - see
//                                                 3.6.6
// orig-date       1               1
// from            1               1               See sender and 3.6.2
// sender          0*              1               MUST occur with multi-
//                                                 address from - see 3.6.2
// reply-to        0               1
// to              0               1
// cc              0               1
// bcc             0               1
// message-id      0*              1               SHOULD be present - see
//                                                 3.6.4
// in-reply-to     0*              1               SHOULD occur in some
//                                                 replies - see 3.6.4
// references      0*              1               SHOULD occur in some
//                                                 replies - see 3.6.4
// subject         0               1
// comments        0               unlimited
// keywords        0               unlimited
// optional-field  0               unlimited
//
// NOTE: This omits some of the structure around trace
pub fn fields(i: Input<u8>) -> U8Result<(Vec<Trace>, Vec<Field>)> {
    many(i, |i| {
        // traces
        trace(i).bind(|i, (return_path, received)| {
            many(i, |i| {
                    or(i, resent_date,
                |i| or(i, resent_from,
                |i| or(i, resent_sender,
                |i| or(i, resent_to,
                |i| or(i, resent_cc,
                |i| or(i, resent_bcc, resent_msg_id))))))

            }).bind(|i, resents| {
                i.ret(Trace{
                    return_path: return_path, 
                    received: received,
                    fields: resents,
                })
            })
        })
    }).bind(|i, traces| {
        many(i, |i| {
                or(i, orig_date,
            |i| or(i, from,
            |i| or(i, sender,
            |i| or(i, reply_to,
            |i| or(i, to,
            |i| or(i, cc,
            |i| or(i, bcc,
            |i| or(i, message_id,
            |i| or(i, in_reply_to,
            |i| or(i, references,
            |i| or(i, subject,
            |i| or(i, comments,
            |i| or(i, comments,
            |i| or(i, keywords, optional_field))))))))))))))

        }).bind(|i, fields| {
            i.ret((traces, fields))
        })
    })
}
