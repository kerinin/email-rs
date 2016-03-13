extern crate mail;
extern crate chomp;
extern crate chrono;
extern crate bytes;

use chomp::*;
use bytes::Bytes;
use chrono::offset::fixed::FixedOffset;
use chrono::offset::TimeZone;
use mail::rfc2822::*;
use mail::rfc2822::message::message;

#[test]
#[ignore]
fn example_1_1_1() {
    let raw = include_bytes!("example_1_1.1.eml");
    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    let email = msg.unwrap();
    for field in email.fields.iter() {
        assert!(!field.is_unstructured());
        assert!(!field.is_malformed());
    }

    let from = AddressesField{
        addresses: vec!(Address::Mailbox{
            local_part: "jdoe".to_string(),
            domain: "machine.example".to_string(),
            display_name: Some(Bytes::from_slice(b"John Doe")),
        }),
    };
    assert_eq!(email.from(), Some(&from));

    let to = AddressesField{
        addresses: vec!(Address::Mailbox{
            local_part: "mary".to_string(),
            domain: "example.net".to_string(),
            display_name: Some(Bytes::from_slice(b"Mary Smith")),
        }),
    };
    assert_eq!(email.to(), Some(&to));

    let subj = UnstructuredField{
        data: Bytes::from_slice(b"Saying Hello"),
    };
    assert_eq!(email.subject(), Some(&subj));

    let date = DateTimeField{
        date_time: FixedOffset::east(-600).ymd(1997, 11, 21).and_hms(9,55,0),
    };
    assert_eq!(email.date(), Some(&date));

    let msgid = MessageIDField{
        message_id: MessageID{
            id_left: Bytes::from_slice(b"1234"),
            id_right: Bytes::from_slice(b"local.machine.example"),
        },
    };
    assert_eq!(email.message_id(), Some(&msgid));
}

#[test]
fn example_1_1_2() {
    let raw = include_bytes!("example_1_1.2.eml");
    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    for field in msg.unwrap().fields.iter() {
        assert!(!field.is_unstructured());
        assert!(!field.is_malformed());
    }
}

#[test]
fn example_1_2() {
    let raw = include_bytes!("example_1_2.eml");
    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    for field in msg.unwrap().fields.iter() {
        assert!(!field.is_unstructured());
        assert!(!field.is_malformed());
    }
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
#[ignore]
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
        assert!(!field.is_unstructured());
        assert!(!field.is_malformed());
    }
}

// NOTE: Need to support crazy email address syntax for this
#[test]
#[ignore]
fn example_6_1() {
    let raw = include_bytes!("example_6_1.eml");
    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    for field in msg.unwrap().fields.iter() {
        assert!(!field.is_unstructured());
        assert!(!field.is_malformed());
    }
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

// NOTE: Need to support obsolete fields syntax for this
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
