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

        ret Field::OrigDate(d)
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

// resent-from     =       "Resent-From:" mailbox-list CRLF

// resent-sender   =       "Resent-Sender:" mailbox CRLF

// resent-to       =       "Resent-To:" address-list CRLF

// resent-cc       =       "Resent-Cc:" address-list CRLF

// resent-bcc      =       "Resent-Bcc:" (address-list / [CFWS]) CRLF

// resent-msg-id   =       "Resent-Message-ID:" msg-id CRLF

// trace           =       [return]
//                         1*received

// return          =       "Return-Path:" path CRLF

// path            =       ([CFWS] "<" ([CFWS] / addr-spec) ">" [CFWS]) /
//                         obs-path

// received        =       "Received:" name-val-list ";" date-time CRLF

// name-val-list   =       [CFWS] [name-val-pair *(CFWS name-val-pair)]

// name-val-pair   =       item-name CFWS item-value

// item-name       =       ALPHA *(["-"] (ALPHA / DIGIT))

// item-value      =       1*angle-addr / addr-spec /
//                          atom / domain / msg-id

// optional-field  =       field-name ":" unstructured CRLF

// field-name      =       1*ftext

// ftext           =       %d33-57 /               ; Any character except
//                         %d59-126                ;  controls, SP, and
//                                                 ;  ":".

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
