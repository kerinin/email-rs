use chomp::*;

use rfc2822::*;
use rfc2822::fields::*;
use rfc2822::primitive::*;

// body            =       *(*998text CRLF) *998text
pub fn body(i: Input<u8>) -> U8Result<Vec<u8>> {
    many(i, text)
}

// message         =       (fields / obs-fields)
//                         [CRLF body]
// TODO: Implement obs-fields
pub fn message(i: Input<u8>) -> U8Result<Message> {
    parse!{i;
        // let f = or(fields, obs_fields);
        let f = fields();
        crlf();
        let b = body();

        ret Message{fields: f, body: b}
    }
}
