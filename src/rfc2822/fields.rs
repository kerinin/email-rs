// orig-date       =       "Date:" date-time CRLF

// from            =       "From:" mailbox-list CRLF

// sender          =       "Sender:" mailbox CRLF

// reply-to        =       "Reply-To:" address-list CRLF

// to              =       "To:" address-list CRLF

// cc              =       "Cc:" address-list CRLF

// bcc             =       "Bcc:" (address-list / [CFWS]) CRLF

// message-id      =       "Message-ID:" msg-id CRLF

// in-reply-to     =       "In-Reply-To:" 1*msg-id CRLF

// references      =       "References:" 1*msg-id CRLF

// msg-id          =       [CFWS] "<" id-left "@" id-right ">" [CFWS]

// id-left         =       dot-atom-text / no-fold-quote / obs-id-left

// id-right        =       dot-atom-text / no-fold-literal / obs-id-right

// no-fold-quote   =       DQUOTE *(qtext / quoted-pair) DQUOTE

// subject         =       "Subject:" unstructured CRLF

// comments        =       "Comments:" unstructured CRLF

// keywords        =       "Keywords:" phrase *("," phrase) CRLF

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
