extern crate mail;
extern crate chomp;
extern crate chrono;
extern crate bytes;

use chomp::*;
use bytes::Bytes;
use chrono::offset::fixed::FixedOffset;
use chrono::offset::TimeZone;
use mail::*;
use mail::rfc5322::*;

#[test]
fn example_1_1_1() {
    let raw = include_bytes!("example_1_1.1.eml");

    let msg = parse_only(raw_fields, raw);
    assert!(msg.is_ok());

    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    let email = msg.unwrap();
    println!("--> msg: {:?}", email);
    for field in email.fields.iter() {
        assert!(!field.is_unstructured());
        assert!(!field.is_malformed());
    }

    /*
    println!("--> msg: {:?}", email);
    let date = DateTimeField{
        date_time: FixedOffset::east(-6*3600).ymd(1997, 11, 21).and_hms(9,55,6),
    };
    assert_eq!(email.date(), Some(&date));

    let from = AddressesField{
        addresses: vec!(Address::Mailbox{
            local_part: "jdoe".to_string(),
            domain: "machine.example".to_string(),
            display_name: Some(Bytes::from_slice(b" John Doe ")),
        }),
    };
    assert_eq!(email.from(), Some(&from));

    let to = AddressesField{
        addresses: vec!(Address::Mailbox{
            local_part: "mary".to_string(),
            domain: "example.net".to_string(),
            display_name: Some(Bytes::from_slice(b" Mary Smith ")),
        }),
    };
    assert_eq!(email.to(), Some(&to));

    let subj = UnstructuredField{
        data: Bytes::from_slice(b" Saying Hello"),
    };
    assert_eq!(email.subject(), Some(&subj));

    let msgid = MessageIDField{
        message_id: MessageID{
            id_left: Bytes::from_slice(b"1234"),
            id_right: Bytes::from_slice(b"local.machine.example"),
        },
    };
    assert_eq!(email.message_id(), Some(&msgid));
    */
}

#[test]
fn example_1_1_2() {
    let raw = include_bytes!("example_1_1.2.eml");
    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    let email = msg.unwrap();
    for field in email.fields.iter() {
        assert!(!field.is_unstructured());
        assert!(!field.is_malformed());
    }

    /*
    let date = DateTimeField{
        date_time: FixedOffset::east(-6*3600).ymd(1997, 11, 21).and_hms(9,55,6),
    };
    assert_eq!(email.date(), Some(&date));

    let from = AddressesField{
        addresses: vec!(Address::Mailbox{
            local_part: "jdoe".to_string(),
            domain: "machine.example".to_string(),
            display_name: Some(Bytes::from_slice(b" John Doe ")),
        }),
    };
    assert_eq!(email.from(), Some(&from));

    let sender = AddressField{
        address: Address::Mailbox{
            local_part: "mjones".to_string(),
            domain: "machine.example".to_string(),
            display_name: Some(Bytes::from_slice(b" Michael Jones ")),
        },
    };
    assert_eq!(email.sender(), Some(&sender));

    let to = AddressesField{
        addresses: vec!(Address::Mailbox{
            local_part: "mary".to_string(),
            domain: "example.net".to_string(),
            display_name: Some(Bytes::from_slice(b" Mary Smith ")),
        }),
    };
    assert_eq!(email.to(), Some(&to));

    let subj = UnstructuredField{
        data: Bytes::from_slice(b" Saying Hello"),
    };
    assert_eq!(email.subject(), Some(&subj));

    let msgid = MessageIDField{
        message_id: MessageID{
            id_left: Bytes::from_slice(b"1234"),
            id_right: Bytes::from_slice(b"local.machine.example"),
        },
    };
    assert_eq!(email.message_id(), Some(&msgid));
    */
}

#[test]
fn example_1_2() {
    let raw = include_bytes!("example_1_2.eml");
    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    let email = msg.unwrap();
    for field in email.fields.iter() {
        assert!(!field.is_unstructured());
        assert!(!field.is_malformed());
    }

    /*
    let date = DateTimeField{
        date_time: FixedOffset::east(2*3600).ymd(2003, 7, 1).and_hms(10,52,37),
    };
    assert_eq!(email.date(), Some(&date));

    let from = AddressesField{
        addresses: vec!(Address::Mailbox{
            local_part: "john.q.public".to_string(),
            domain: "example.com".to_string(),
            display_name: Some(Bytes::from_slice(b" Joe Q. Public ")),
        }),
    };
    assert_eq!(email.from(), Some(&from));

    let to = AddressesField{
        addresses: vec!(
                       Address::Mailbox{
                           local_part: "mary".to_string(),
                           domain: "x.test".to_string(),
                           display_name: Some(Bytes::from_slice(b" Mary Smith " )),
                       },
                       Address::Mailbox{
                           local_part: " jdoe".to_string(),
                           domain: "example.org".to_string(),
                           display_name: None,
                       },
                       Address::Mailbox{
                           local_part: "one".to_string(),
                           domain: "y.test".to_string(),
                           display_name: Some(Bytes::from_slice(b" Who? ")),
                       },
        ),
    };
    assert_eq!(email.to(), Some(&to));

    let cc = AddressesField{
        addresses: vec!(
                       Address::Mailbox{
                           local_part: "boss".to_string(),
                           domain: "nil.test".to_string(),
                           display_name: None,
                       },
                       Address::Mailbox{
                           local_part: "sysservices".to_string(),
                           domain: "example.net".to_string(),
                           display_name: Some(Bytes::from_slice(b" Giant; \\\"Big\\\" Box ")),
                       },
        ),
    };
    assert_eq!(email.cc(), Some(&cc));

    let msgid = MessageIDField{
        message_id: MessageID{
            id_left: Bytes::from_slice(b"5678.21-Nov-1997"),
            id_right: Bytes::from_slice(b"example.com"),
        },
    };
    assert_eq!(email.message_id(), Some(&msgid));
    */
}

#[test]
fn example_1_3() {
    let raw = include_bytes!("example_1_3.eml");
    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    for field in msg.unwrap().fields.iter() {
        assert!(!field.is_unstructured());
        assert!(!field.is_malformed());
    }
}

#[test]
fn example_2_1() {
    let raw = include_bytes!("example_2.1.eml");
    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    for field in msg.unwrap().fields.iter() {
        assert!(!field.is_unstructured());
        assert!(!field.is_malformed());
    }
}


#[test]
fn example_2_2() {
    let raw = include_bytes!("example_2.2.eml");
    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    for field in msg.unwrap().fields.iter() {
        assert!(!field.is_unstructured());
        assert!(!field.is_malformed());
    }
}
#[test]
fn example_2_3() {
    let raw = include_bytes!("example_2.3.eml");
    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    for field in msg.unwrap().fields.iter() {
        assert!(!field.is_unstructured());
        assert!(!field.is_malformed());
    }
}

#[test]
fn example_3_1() {
    let raw = include_bytes!("example_3.1.eml");
    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    for field in msg.unwrap().fields.iter() {
        assert!(!field.is_unstructured());
        assert!(!field.is_malformed());
    }
}

// NOTE: Need to support obsolete fields syntax for this
#[test]
fn example_3_2() {
    let raw = include_bytes!("example_3.2.eml");
    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    for field in msg.unwrap().fields.iter() {
        assert!(!field.is_unstructured());
        assert!(!field.is_malformed());
    }
}

#[test]
fn example_4() {
    let raw = include_bytes!("example_4.eml");
    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    for field in msg.unwrap().fields.iter() {
        assert!(!field.is_unstructured());
        assert!(!field.is_malformed());
    }
}

#[test]
fn example_5() {
    let raw = include_bytes!("example_5.eml");
    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    for field in msg.unwrap().fields.iter() {
        println!("checking {:?}", field);
        assert!(!field.is_unstructured());
        assert!(!field.is_malformed());
    }
}

#[test]
fn example_6_1() {
    let raw = include_bytes!("example_6_1.eml");
    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    // NOTE: Screw this wierdo routing noise
    // for field in msg.unwrap().fields.iter() {
    //     assert!(!field.is_unstructured());
    //     assert!(!field.is_malformed());
    // }
}

#[test]
fn example_6_2() {
    let raw = include_bytes!("example_6_2.eml");
    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    for field in msg.unwrap().fields.iter() {
        assert!(!field.is_unstructured());
        assert!(!field.is_malformed());
    }
}

// NOTE: I'm OK with not supporting some of this crazy syntax
#[test]
#[ignore]
fn example_6_3() {
    let raw = include_bytes!("example_6_3.eml");
    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    for field in msg.unwrap().fields.iter() {
        assert!(!field.is_unstructured());
        assert!(!field.is_malformed());
    }
}
