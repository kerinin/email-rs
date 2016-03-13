use chomp::*;
use bytes::Bytes;

use rfc2822::*;
use rfc2822::fields::*;
use rfc2822::primitive::*;

// body            =       *(*998text CRLF) *998text
pub fn body(i: Input<u8>) -> U8Result<Bytes> {
    matched_by(i, |i| skip_many(i, text)).map(|(v, _)| Bytes::from_slice(v))
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

                let mut maybe_date = None;
                let mut maybe_from = None;

                for field in fields.iter() {
                    match field {
                        &Field::Date(ref f) => maybe_date = Some(f.clone()),
                        &Field::From(ref f) => maybe_from = Some(f.clone()),
                        _ => {},
                    }
                }

                if let (Some(date), Some(from)) = (maybe_date, maybe_from) {
                    i.ret(Message{
                        date: date,
                        from: from,
                        traces: traces, 
                        fields: fields, 
                        body: body,
                    })
                } else {
                    i.err(Error::Unexpected)
                }
            })
        })
    })
}
