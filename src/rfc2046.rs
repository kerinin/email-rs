//! RFC2046 provides initial media type definitions
//! https://tools.ietf.org/html/rfc2046

// boundary := 0*69<bchars> bcharsnospace
//
// bchars := bcharsnospace / " "
//
// bcharsnospace := DIGIT / ALPHA / "'" / "(" / ")" /
//                  "+" / "_" / "," / "-" / "." /
//                  "/" / ":" / "=" / "?"
//
// body-part := <"message" as defined in RFC 822, with all
//               header fields optional, not starting with the
//               specified dash-boundary, and with the
//               delimiter not occurring anywhere in the
//               body part.  Note that the semantics of a
//               part differ from the semantics of a message,
//               as described in the text.>
//
// close-delimiter := delimiter "--"
//
// dash-boundary := "--" boundary
//                  ; boundary taken from the value of
//                  ; boundary parameter of the
//                  ; Content-Type field.
//
// delimiter := CRLF dash-boundary
//
// discard-text := *(*text CRLF)
//                 ; May be ignored or discarded.
//
// encapsulation := delimiter transport-padding
//                  CRLF body-part
//
// epilogue := discard-text
//
// multipart-body := [preamble CRLF]
//                   dash-boundary transport-padding CRLF
//                   body-part *encapsulation
//                   close-delimiter transport-padding
//                   [CRLF epilogue]
//
// preamble := discard-text
//
// transport-padding := *LWSP-char
//                      ; Composers MUST NOT generate
//                      ; non-zero length transport
//                      ; padding, but receivers MUST
//                      ; be able to handle padding
//                      ; added by message transports.
