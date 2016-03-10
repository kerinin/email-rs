use std::iter::FromIterator;

use chomp::*;

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
    parse!{i;
        string(b"Date:");
        let d = date_time();
        crlf();

        ret Field::Date(d)
    }
}

// from            =       "From:" mailbox-list CRLF
pub fn from(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"From:");
        let l = mailbox_list();
        crlf();

        ret Field::From(l)
    }
}

// sender          =       "Sender:" mailbox CRLF
pub fn sender(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"Sender:");
        let l = mailbox();
        crlf();

        ret Field::Sender(l)
    }
}

// reply-to        =       "Reply-To:" address-list CRLF
pub fn reply_to(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"Reply-To:");
        let l = address_list();
        crlf();

        ret Field::ReplyTo(l)
    }
}

// to              =       "To:" address-list CRLF
pub fn to(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"To:");
        let l = address_list();
        crlf();

        ret Field::To(l)
    }
}

// cc              =       "Cc:" address-list CRLF
pub fn cc(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"Cc:");
        let l = address_list();
        crlf();

        ret Field::Cc(l)
    }
}

// bcc             =       "Bcc:" (address-list / [CFWS]) CRLF
pub fn bcc(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"Bcc:");
        let l = address_list();
        crlf();

        ret Field::Bcc(l)
    }
}

// no-fold-quote   =       DQUOTE *(qtext / quoted-pair) DQUOTE
pub fn no_fold_quote(i: Input<u8>) -> U8Result<Vec<u8>> {
    parse!{i;
        dquote();
        let t = many(|i| or(i, qtext, quoted_pair));
        dquote();

        ret t
    }
}

// id-left         =       dot-atom-text / no-fold-quote / obs-id-left
pub fn id_left(i: Input<u8>) -> U8Result<Vec<u8>> {
    or(i, 
       |i| dot_atom_text(i).map(|i| {
           let mut v = Vec::with_capacity(i.len());
           v.extend(i);
           v
       }), 
       |i| or(i, no_fold_quote, obs_id_left),
       )
}

// no-fold-literal =       "[" *(dtext / quoted-pair) "]"
pub fn no_fold_literal(i: Input<u8>) -> U8Result<Vec<u8>> {
    parse!{i;
        token(b'[');
        let t = many(|i| or(i, dtext, quoted_pair));
        token(b'[');

        ret t
    }
}

// id-right        =       dot-atom-text / no-fold-literal / obs-id-right
pub fn id_right(i: Input<u8>) -> U8Result<Vec<u8>> {
    or(i, 
       |i| dot_atom_text(i).map(|i| {
           let mut v = Vec::with_capacity(i.len());
           v.extend(i);
           v
       }), 
       |i| or(i, no_fold_literal, obs_id_right),
       )
}

// msg-id          =       [CFWS] "<" id-left "@" id-right ">" [CFWS]
pub fn msg_id(i: Input<u8>) -> U8Result<Vec<u8>> {
    parse!{i;
        option(cfws, ());
        token(b'<');
        let l = id_left();
        token(b'@');
        let r = id_right();
        token(b'>');
        option(cfws, ());

        ret {
            // NOTE: See if we can rely on `id_left` and `id_right` being
            // continguous so we can just use `matched_by` here...
            let mut v = Vec::with_capacity(l.len() + r.len() + 3);
            v.push(b'<');
            v.extend(l);
            v.push(b'@');
            v.extend(r);
            v.push(b'>');
            v
        }

    }
}

// message-id      =       "Message-ID:" msg-id CRLF
pub fn message_id(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"Message-ID:");
        let id = msg_id();
        crlf();

        ret Field::MessageID(id)
    }
}

// in-reply-to     =       "In-Reply-To:" 1*msg-id CRLF
pub fn in_reply_to(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"In-Reply-To:");
        let ids = many1(msg_id);
        crlf();

        ret Field::InReplyTo(ids)
    }
}

// references      =       "References:" 1*msg-id CRLF
pub fn references(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"References:");
        let ids = many1(msg_id);
        crlf();

        ret Field::References(ids)
    }
}

// subject         =       "Subject:" unstructured CRLF
pub fn subject(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"Subject:");
        let u = unstructured();
        crlf();

        ret Field::Subject(u)
    }
}

// comments        =       "Comments:" unstructured CRLF
pub fn comments(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"Comments:");
        let u = unstructured();
        crlf();

        ret Field::Comments(u)
    }
}

// keywords        =       "Keywords:" phrase *("," phrase) CRLF
pub fn keywords(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"Keywords:");
        let kws = sep_by1(phrase, |i| token(i, b',')); 
        crlf();

        ret Field::Keywords(kws)
    }
}

// resent-date     =       "Resent-Date:" date-time CRLF
pub fn resent_date(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"Resent-Date:");
        let d = date_time();
        crlf();

        ret Field::ResentDate(d)
    }
}

// resent-from     =       "Resent-From:" mailbox-list CRLF
pub fn resent_from(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"Resent-From:");
        let l = mailbox_list();
        crlf();

        ret Field::ResentFrom(l)
    }
}

// resent-sender   =       "Resent-Sender:" mailbox CRLF
pub fn resent_sender(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"Resent-Sender:");
        let l = mailbox();
        crlf();

        ret Field::ResentSender(l)
    }
}

// resent-to       =       "Resent-To:" address-list CRLF
pub fn resent_to(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"Resent-To:");
        let l = address_list();
        crlf();

        ret Field::ResentTo(l)
    }
}

// resent-cc       =       "Resent-Cc:" address-list CRLF
pub fn resent_cc(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"Resent-Cc:");
        let l = address_list();
        crlf();

        ret Field::ResentCc(l)
    }
}

// resent-bcc      =       "Resent-Bcc:" (address-list / [CFWS]) CRLF
pub fn resent_bcc(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"Resent-Bcc:");
        let l = address_list();
        crlf();

        ret Field::ResentBcc(l)
    }
}

// resent-msg-id   =       "Resent-Message-ID:" msg-id CRLF
pub fn resent_msg_id(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"Resent-Message-ID:");
        let id = msg_id();
        crlf();

        ret Field::ResentMessageID(id)
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
        option(cfws, ());
        token(b'<');
        option(cfws, ());
        let a = addr_spec();
        token(b'>');
        option(cfws, ());

        ret a
    }
}

// return-path     =       "Return-Path:" path CRLF
pub fn return_path(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"Return-Path:");
        let p = path();
        crlf();

        ret Field::ReturnPath(p)
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
pub fn item_value(i: Input<u8>) -> U8Result<ReceivedValue> {
    or(i, 
       |i| many1(i, angle_addr).map(|a| ReceivedValue::Addresses(a)),
       |i| or(i, |i| addr_spec(i).map(|a| ReceivedValue::Address(a)),
       |i| or(i, |i| atom(i).map(|v| ReceivedValue::Text(FromIterator::from_iter(v.iter().map(|i| i.clone())))),
       |i| or(i, |i| domain(i).map(|v| ReceivedValue::Domain(v)), 
              |i| msg_id(i).map(|v| ReceivedValue::MessageID(v))))))
}

// name-val-pair   =       item-name CFWS item-value
pub fn name_val_pair(i: Input<u8>) -> U8Result<(&[u8], ReceivedValue)> {
    parse!{i;
        let n = item_name();
        cfws();
        let v = item_value();

        ret (n, v)
    }
}
// name-val-list   =       [CFWS] [name-val-pair *(CFWS name-val-pair)]
pub fn name_val_list(i: Input<u8>) -> U8Result<Vec<(&[u8], ReceivedValue)>> {
    parse!{i;
        cfws();
        sep_by(name_val_pair, cfws)
    }
}

// received        =       "Received:" name-val-list ";" date-time CRLF
pub fn received(i: Input<u8>) -> U8Result<Field> {
    parse!{i;
        string(b"Received:");
        let nvs = name_val_list();
        token(b';');
        let dt = date_time();
        crlf();

        ret {
            let name_values = nvs.into_iter().map(|(n, v)| {
                let name = FromIterator::from_iter(n.iter().map(|i| i.clone()));
                (name, v)
            }).collect();

            Field::Received(name_values, dt)
        }
    }
}

// trace           =       [return-path] 1*received
pub fn trace(i: Input<u8>) -> U8Result<(Option<Field>, Vec<Field>)> {
    parse!{i;
        let rp: Option<Field> = option(|i| return_path(i).map(|r| Some(r)), None);
        let rs = many1(received);

        ret (rp, rs)
    }
}

// ftext           =       %d33-57 /               ; Any character except
//                         %d59-126                ;  controls, SP, and
//                                                 ;  ":".
pub fn ftext(i: Input<u8>) -> U8Result<u8> {
    satisfy(i, |i| (33 <= i && i <= 57) || (59 <= i && i <= 126))
}

// field-name      =       1*ftext
pub fn field_name(i: Input<u8>) -> U8Result<Vec<u8>> {
    many1(i, ftext)
}

// optional-field  =       field-name ":" unstructured CRLF
pub fn optional_field(i: Input<u8>) -> U8Result<(Vec<u8>, Vec<u8>)> {
    parse!{i;
        let n = field_name();
        let v = unstructured();
        crlf();

        ret (n, v)
    }
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
