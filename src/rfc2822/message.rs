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
    println!("message({:?})", i);

    // let f = or(fields, obs_fields);
    fields(i).bind(|i, (traces, fields)| {
        println!("message.fields.bind({:?}, ({:?}, {:?}))", i, traces, fields);
        crlf(i).then(|i| {
            println!("message.crlf.then({:?})", i);
            body(i).bind(|i, body| {
                println!("message.body.bind({:?}, {:?})", i, body);
                i.ret(Message{traces: traces, fields: fields, body: body})
            })
        })
    })
}
