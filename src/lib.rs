//! Parser/Serializer for emails

#[macro_use]
extern crate chomp;

mod rfc2822;

/*
   struct Message {
   date: Header::OrigDate,
   from: Header::From,
   sender: Option<Header::Sender>,
   reply_to: Option<Header::ReplyTo>,
   to: Option<Header::To>,
   cc: Option<Header::Cc>,
   bcc: Option<Header::Bcc>,
   message_id: Option<Header::MessageID>,
   in_reply_to: Option<Header::InReplyTo>,
   references: Option<Header::References>,
   subject: Option<Header::Subject>,
   return_path: Vec<Header::ReturnPath>,
   received: Vec<Header::Received>,
   optional_headers: Vec<Header::Unstructured>,

   headers: Vec<Header>,
   body: [u8],
   }

   enum Header {
   ReturnPath(String),
   Received(String, DateTime),
   ResentDate(DateTime),
   ResentFrom(Vec<Address>),
   ResentSender(Address),
   ResentTo(Vec<Address>),
   ResentCc(Vec<Address>),
   ResentBcc(Vec<Address>),
   ResentMsgID(String),
   OrigDate(DateTime),
   From(Vec<Address>),
   Sender(Address),
   ReplyTo(Vec<Address>),
   To(Vec<Address>),
   Cc(Vec<Address>),
   Bcc(Vec<Address>),
   MessageID(String),
   InReplyTo(Vec<String>),
   References(Vec<String>),
   Subject(String),
   Comments(String),
   Keywords(Vec<String>),
   Unstructured { name: [u8], value: [u8] },
   }

   type Body = (String);

   struct Address{
   display: String,
   local: String,
   domain: String,
   }

#[cfg(test)]
mod test {
#[test]
fn it_works() {
}
}
*/
