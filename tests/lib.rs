extern crate mail;
extern crate chomp;

use chomp::*;
use mail::rfc2822::message::message;

#[test]
fn example_1_1_1() {
    let raw = include_bytes!("example_1_1.1.eml");
    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    for field in msg.unwrap().fields.iter() {
        assert!(!field.is_unstructured());
    }
}

#[test]
fn example_1_1_2() {
    let raw = include_bytes!("example_1_1.2.eml");
    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    for field in msg.unwrap().fields.iter() {
        assert!(!field.is_unstructured());
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
    }
}

// Requires obsolete header fields
#[test]
#[ignore]
fn example_3_2() {
    let raw = include_bytes!("example_3.2.eml");
    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    for field in msg.unwrap().fields.iter() {
        assert!(!field.is_unstructured());
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
    }
}

#[test]
#[ignore]
fn example_6_1() {
    let raw = include_bytes!("example_6_1.eml");
    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    for field in msg.unwrap().fields.iter() {
        assert!(!field.is_unstructured());
    }
}

#[test]
#[ignore]
fn example_6_2() {
    let raw = include_bytes!("example_6_2.eml");
    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    for field in msg.unwrap().fields.iter() {
        assert!(!field.is_unstructured());
    }
}

#[test]
#[ignore]
fn example_6_3() {
    let raw = include_bytes!("example_6_3.eml");
    let msg = parse_only(message, raw);
    println!("{:?}", msg);

    assert!(msg.is_ok());
    for field in msg.unwrap().fields.iter() {
        assert!(!field.is_unstructured());
    }
}
