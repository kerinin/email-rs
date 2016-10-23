//! RFC2045 specifies MIME formatting of message bodies
use std::collections::HashMap;

use chomp::types::*;
use chomp::parsers::*;
use chomp::combinators::*;

use super::*;
use super::rfc5322::*;
use super::util::*;

// entity-headers := [ content CRLF ]
//                   [ encoding CRLF ]
//                   [ id CRLF ]
//                   [ description CRLF ]
//                   *( MIME-extension-field CRLF )
//
// MIME-message-headers := entity-headers
//                         fields
//                         version CRLF
//                         ; The ordering of the header
//                         ; fields implied by this BNF
//                         ; definition should be ignored.
//
// MIME-part-headers := entity-headers
//                      [ fields ]
//                      ; Any field not beginning with
//                      ; "content-" can have no defined
//                      ; meaning and may be ignored.
//                      ; The ordering of the header
//                      ; fields implied by this BNF
//                      ; definition should be ignored.

// version := "MIME-Version" ":" 1*DIGIT "." 1*DIGIT
// Note that the MIME-Version header field is required at the top level
// of a message.  It is not required for each body part of a multipart
// entity.  It is required for the embedded headers of a body of type
// "message/rfc822" or "message/partial" if and only if the embedded
// message is itself claimed to be MIME-conformant.
//
// NOTE TO IMPLEMENTORS:  When checking MIME-Version values any RFC 822
// comment strings that are present must be ignored.  In particular, the
// following four MIME-Version fields are equivalent:
//
//   MIME-Version: 1.0
//
//   MIME-Version: 1.0 (produced by MetaSend Vx.x)
//
//   MIME-Version: (produced by MetaSend Vx.x) 1.0
//
//   MIME-Version: 1.(produced by MetaSend Vx.x)0
// NOTE: Implementing as:
// version := "MIME-Version" ":" *(comment) 1*DIGIT *(comment) "." *(comment) 1*DIGIT *(comment)
pub fn version<I: U8Input>(i: I) -> SimpleResult<I, Field<I>> {
    drop_field_name(i, b"MIME-Version").then(|i| {
        option(i, comment, ()).then(|i| {
            parse_digits(i, (1..)).bind(|i, l| {
                option(i, comment, ()).then(|i| {
                    token(i, b'.').then(|i| {
                        option(i, comment, ()).then(|i| {
                            parse_digits(i, (1..)).bind(|i, r| {
                                option(i, comment, ()).then(|i| {
                                    let f = VersionField { top_version: l, sub_version: r};
                                    i.ret(Field::MIMEVersion(f))
                                })
                            })
                        })
                    })
                })
            })
        })
    })
}

#[derive(PartialEq)]
pub enum TopLevel {
    Text,
    Image,
    Audio,
    Video,
    Application,
    Message,
    Multipart,
    Ext(String),
}

impl TopLevel {
    pub fn to_string(&self) -> String {
        match self {
            &TopLevel::Text => "text".to_string(),
            &TopLevel::Image => "image".to_string(),
            &TopLevel::Audio => "audio".to_string(),
            &TopLevel::Video => "video".to_string(),
            &TopLevel::Application => "application".to_string(),
            &TopLevel::Message => "message".to_string(),
            &TopLevel::Multipart => "multipart".to_string(),
            &TopLevel::Ext(ref s) => s.clone(),
        }
    }
}

#[derive(PartialEq)]
pub enum SubLevel {
    Plain,
    Ext(String),
}

impl SubLevel {
    pub fn to_string(&self) -> String {
        match self {
            &SubLevel::Plain => "plain".to_string(),
            &SubLevel::Ext(ref s) => s.clone(),
        }
    }
}

// content := "Content-Type" ":" type "/" subtype
//            *(";" parameter)
//            ; Matching of media type and subtype
//            ; is ALWAYS case-insensitive.
// Default RFC 822 messages without a MIME Content-Type header are taken
// by this protocol to be plain text in the US-ASCII character set,
// which can be explicitly specified as:
//
//   Content-type: text/plain; charset=us-ascii
//
// This default is assumed if no Content-Type header field is specified.
// It is also recommend that this default be assumed when a
// syntactically invalid Content-Type header field is encountered. In
// the presence of a MIME-Version header field and the absence of any
// Content-Type header field, a receiving User Agent can also assume
// that plain US-ASCII text was the sender's intent.  Plain US-ASCII
// text may still be assumed in the absence of a MIME-Version or the
// presence of an syntactically invalid Content-Type header field, but
// the sender's intent might have been otherwise.
//
// The type, subtype, and parameter names are not case sensitive.  For
// example, TEXT, Text, and TeXt are all equivalent top-level media
// types.  Parameter values are normally case sensitive, but sometimes
// are interpreted in a case-insensitive fashion, depending on the
// intended use.  (For example, multipart boundaries are case-sensitive,
// but the "access-type" parameter for message/External-body is not
// case-sensitive.)
// TODO: parameter parsing
pub fn content<I: U8Input>(i: I) -> SimpleResult<I, Field<I>> {
    drop_field_name(i, b"Content-Type").then(|i| {
        option(i, |i| {
            mime_type(i).bind(|i, t| {
                token(i, b'/').then(|i| {
                    subtype(i).bind(|i, s| {
                        many(i, |i| {
                            token(i, b';').then(parameter)
                        }).bind(|i, mut params: Vec<(I::Buffer, Vec<I::Buffer>)>| {
                            let mut p = HashMap::with_capacity(params.len());
                            p = params.into_iter().fold(p, |mut p, (attr, val)| {
                                let v: Vec<u8> = val.into_iter().flat_map(|v| v.into_vec()).collect();
                                p.insert( unsafe { String::from_utf8_unchecked(attr.into_vec()) }, v);
                                p
                            });

                            let f = ContentTypeField {top_level: t, sub_level: s, params: p};
                            i.ret(Field::ContentType(f))
                        })
                    })
                })
            })
        }, Field::ContentType(ContentTypeField {top_level: TopLevel::Text, sub_level: SubLevel::Plain, params: HashMap::new()}))
    })
}

// type := discrete-type / composite-type
pub fn mime_type<I: U8Input>(i: I) -> SimpleResult<I, TopLevel> {
    or(i, discrete_type, composite_type)
}

// discrete-type := "text" / "image" / "audio" / "video" /
//                  "application" / extension-token
pub fn discrete_type<I: U8Input>(i: I) -> SimpleResult<I, TopLevel> {
    or(i, |i| downcased_string(i, b"text").then(|i| i.ret(TopLevel::Text)),
    |i| or(i, |i| downcased_string(i, b"image").then(|i| i.ret(TopLevel::Image)),
    |i| or(i, |i| downcased_string(i, b"audio").then(|i| i.ret(TopLevel::Audio)),
    |i| or(i, |i| downcased_string(i, b"video").then(|i| i.ret(TopLevel::Video)),
    |i| or(i, |i| downcased_string(i, b"application").then(|i| i.ret(TopLevel::Application)),
              |i| type_extension_token(i))))))
}

// composite-type := "message" / "multipart" / extension-token
pub fn composite_type<I: U8Input>(i: I) -> SimpleResult<I, TopLevel> {
    or(i, |i| downcased_string(i, b"message").then(|i| i.ret(TopLevel::Message)),
    |i| or(i, |i| downcased_string(i, b"multipart").then(|i| i.ret(TopLevel::Multipart)),
              |i| type_extension_token(i)))
}

// extension-token := ietf-token / x-token
// TODO: Read 2048 and find any ietf-token definitions that exist
pub fn type_extension_token<I: U8Input>(i: I) -> SimpleResult<I, TopLevel> {
    type_x_token(i)
}
pub fn subtype_extension_token<I: U8Input>(i: I) -> SimpleResult<I, SubLevel> {
    subtype_x_token(i)
}
pub fn mechanism_extension_token<I: U8Input>(i: I) -> SimpleResult<I, Encoding> {
    mechanism_x_token(i)
}


// ietf-token := <An extension token defined by a
//                standards-track RFC and registered
//                with IANA.>

// x-token := <The two characters "X-" or "x-" followed, with
//             no intervening white space, by any token>
pub fn type_x_token<I: U8Input>(i: I) -> SimpleResult<I, TopLevel> {
    matched_by(i, |i| {
        downcased_string(i, b"X-").then(mime_token)
    }).map(|(buf, _)| {
        TopLevel::Ext(unsafe { String::from_utf8_unchecked(buf.into_vec()) } )
    })
}
pub fn subtype_x_token<I: U8Input>(i: I) -> SimpleResult<I, SubLevel> {
    matched_by(i, |i| {
        downcased_string(i, b"X-").then(mime_token)
    }).map(|(buf, _)| {
        SubLevel::Ext(unsafe { String::from_utf8_unchecked(buf.into_vec()) } )
    })
}
pub fn mechanism_x_token<I: U8Input>(i: I) -> SimpleResult<I, Encoding> {
    matched_by(i, |i| {
        downcased_string(i, b"X-").then(mime_token)
    }).map(|(buf, _)| {
        Encoding::Ext(unsafe { String::from_utf8_unchecked(buf.into_vec()) } )
    })
}

// subtype := extension-token / iana-token
// TODO: Read 2048 and find any iana-token definitions that exist
pub fn subtype<I: U8Input>(i: I) -> SimpleResult<I, SubLevel> {
    subtype_extension_token(i)
}

// iana-token := <A publicly-defined extension token. Tokens
//                of this form must be registered with IANA
//                as specified in RFC 2048.>

// parameter := attribute "=" value
pub fn parameter<I: U8Input>(i: I) -> SimpleResult<I, (I::Buffer, Vec<I::Buffer>)> {
    attribute(i).bind(|i, a| {
        token(i, b'=').then(|i| {
            value(i).bind(|i, v| {
                i.ret((a, v))
            })
        })
    })
}

// attribute := token
//              ; Matching of attributes
//              ; is ALWAYS case-insensitive.
pub fn attribute<I: U8Input>(i: I) -> SimpleResult<I, I::Buffer> {
    mime_token(i)
}

// value := token / quoted-string
// Note that the value of a quoted string parameter does not include the
// quotes.  That is, the quotation marks in a quoted-string are not a
// part of the value of the parameter, but are merely used to delimit
// that parameter value.  In addition, comments are allowed in
// accordance with RFC 822 rules for structured header fields.  Thus the
// following two forms
//   Content-type: text/plain; charset=us-ascii (Plain text)
//
//   Content-type: text/plain; charset="us-ascii"
//
// are completely equivalent.
pub fn value<I: U8Input>(i: I) -> SimpleResult<I, Vec<I::Buffer>> {
    or(i, |i| mime_token(i).map(|v| vec!(v)), quoted_string)
}


// token := 1*<any (US-ASCII) CHAR except SPACE, CTLs,
//             or tspecials>
//
// tspecials :=  "(" / ")" / "<" / ">" / "@" /
//               "," / ";" / ":" / "\" / <">
//               "/" / "[" / "]" / "?" / "="
//               ; Must be in quoted-string,
//               ; to use within parameter values
// SPACE       =  %d32
// CTL         =  <any ASCII control           ; %d0-31
//                 character and DEL>          ; %d127
// TSPECIAL    =       %d34 \       ; "
//                     %d40 \       ; (
//                     %d41 \       ; )
//                     %d44 \       ; ,
//                     %d47 \       ; /
//                     %d58 \       ; :
//                     %d59 \       ; ;
//                     %d60 \       ; <
//                     %d61 \       ; =
//                     %d62 \       ; >
//                     %d63 \       ; ?
//                     %d64 \       ; @
//                     %d91 \       ; [
//                     %d92 \       ; \
//                     %d93 \       ; ]
// TODO: Set TSPECIAL to false
const MIME_TOKEN: [bool; 256] = [
    //  0      1      2      3      4      5      6      7      8      9     10     11     12     13     14     15     16     17     18     19
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, //   0 -  19
    false, false, false, false, false, false, false, false, false, false, false, false, true,  true,  false, true,  true,  true,  true,  true,  //  20 -  39
    false, false, true,  true,  false, true,  true,  false, true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false, false, //  40 -  59
    false, false, false, false, false, true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  //  60 -  79
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false, false, false, true,  true,  true,  true,  true,  true,  //  80 -  99
    true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  // 100 - 119
    true,  true,  true,  true,  true,  true,  true,  false, false, false, false, false, false, false, false, false, false, false, false, false, // 120 - 139
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 140 - 159
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 160 - 179
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 180 - 199
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 200 - 219
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, // 220 - 239
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false                              // 240 - 256
];
pub fn mime_token<I: U8Input>(i: I) -> SimpleResult<I, I::Buffer> {
    take_while(i, |t| MIME_TOKEN[t as usize])
}

#[derive(PartialEq)]
pub enum Encoding {
    SevenBit,
    EightBit,
    Binary,
    QuotedPrintable,
    Base64,
    Ext(String),
}

impl Encoding {
    pub fn to_string(&self) -> String {
        match self {
            &Encoding::SevenBit => "7bit".to_string(),
            &Encoding::EightBit => "8bit".to_string(),
            &Encoding::Binary => "binary".to_string(),
            &Encoding::QuotedPrintable => "quoted-printable".to_string(),
            &Encoding::Base64 => "base64".to_string(),
            &Encoding::Ext(ref s) => s.clone(),
        }
    }
}

// encoding := "Content-Transfer-Encoding" ":" mechanism
pub fn encoding<I: U8Input>(i: I) -> SimpleResult<I, Field<I>> {
    drop_field_name(i, b"Content-Transfer-Encoding").then(|i| {
        mechanism(i).bind(|i, m| {
            let f = EncodingField {encoding: m};

            i.ret(Field::ContentTransferEncoding(f))
        })
    })
}

// mechanism := "7bit" / "8bit" / "binary" /
//              "quoted-printable" / "base64" /
//              ietf-token / x-token
// TODO: Find ietf-token values
pub fn mechanism<I: U8Input>(i: I) -> SimpleResult<I, Encoding> {
    or(i, |i| downcased_string(i, b"7bit").then(|i| i.ret(Encoding::SevenBit)),
    |i| or(i, |i| downcased_string(i, b"8bit").then(|i| i.ret(Encoding::EightBit)),
    |i| or(i, |i| downcased_string(i, b"binary").then(|i| i.ret(Encoding::Binary)),
    |i| or(i, |i| downcased_string(i, b"quoted-printable").then(|i| i.ret(Encoding::QuotedPrintable)),
    |i| or(i, |i| downcased_string(i, b"base64").then(|i| i.ret(Encoding::Base64)),
              |i| mechanism_extension_token(i))))))
}

// These values are not case sensitive -- Base64 and BASE64 and bAsE64
//    are all equivalent.  An encoding type of 7BIT requires that the body
//    is already in a 7bit mail-ready representation.  This is the default
//    value -- that is, "Content-Transfer-Encoding: 7BIT" is assumed if the
//    Content-Transfer-Encoding header field is not present.
//
// quoted-printable := qp-line *(CRLF qp-line)
// (1)   An "=" followed by two hexadecimal digits, one or both
//       of which are lowercase letters in "abcdef", is formally
//       illegal. A robust implementation might choose to
//       recognize them as the corresponding uppercase letters.
//
// (2)   An "=" followed by a character that is neither a
//       hexadecimal digit (including "abcdef") nor the CR
//       character of a CRLF pair is illegal.  This case can be
//       the result of US-ASCII text having been included in a
//       quoted-printable part of a message without itself
//       having been subjected to quoted-printable encoding.  A
//       reasonable approach by a robust implementation might be
//       to include the "=" character and the following
//       character in the decoded data without any
//       transformation and, if possible, indicate to the user
//       that proper decoding was not possible at this point in
//       the data.
//
// (3)   An "=" cannot be the ultimate or penultimate character
//       in an encoded object.  This could be handled as in case
//       (2) above.
//
// (4)   Control characters other than TAB, or CR and LF as
//       parts of CRLF pairs, must not appear. The same is true
//       for octets with decimal values greater than 126.  If
//       found in incoming quoted-printable data by a decoder, a
//       robust implementation might exclude them from the
//       decoded data and warn the user that illegal characters
//       were discovered.
//
// (5)   Encoded lines must not be longer than 76 characters,
//       not counting the trailing CRLF. If longer lines are
//       found in incoming, encoded data, a robust
//       implementation might nevertheless decode the lines, and
//       might report the erroneous encoding to the user.
//
// qp-line := *(qp-segment transport-padding CRLF)
//            qp-part transport-padding
//
// qp-part := qp-section
//            ; Maximum length of 76 characters
//
// qp-segment := qp-section *(SPACE / TAB) "="
//               ; Maximum length of 76 characters
//
// qp-section := [*(ptext / SPACE / TAB) ptext]
//
// ptext := hex-octet / safe-char
//
// safe-char := <any octet with decimal value of 33 through
//              60 inclusive, and 62 through 126>
//              ; Characters not listed as "mail-safe" in
//              ; RFC 2049 are also not recommended.
//
// hex-octet := "=" 2(DIGIT / "A" / "B" / "C" / "D" / "E" / "F")
//              ; Octet must be used for characters > 127, =,
//              ; SPACEs or TABs at the ends of lines, and is
//              ; recommended for any character not listed in
//              ; RFC 2049 as "mail-safe".
//
// transport-padding := *LWSP-char
//                      ; Composers MUST NOT generate
//                      ; non-zero length transport
//                      ; padding, but receivers MUST
//                      ; be able to handle padding
//                      ; added by message transports.
//
// id := "Content-ID" ":" msg-id
pub fn id<I: U8Input>(i: I) -> SimpleResult<I, Field<I>> {
    drop_field_name(i, b"Content-ID").then(|i| {
        till_crlf(i).bind(|i, v| {
            let value = MessageIDField {data: v};

            i.ret(Field::ContentID(value))
        })
    })
}

// description := "Content-Description" ":" *text
pub fn description<I: U8Input>(i: I) -> SimpleResult<I, Field<I>> {
    drop_field_name(i, b"Content-Description").then(|i| {
        till_crlf(i).bind(|i, v| {
            let value = TextField {data: v};

            i.ret(Field::ContentDescription(value))
        })
    })
}
