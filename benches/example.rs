#[macro_use]
extern crate bencher;
extern crate chomp;
extern crate mail;

use bencher::Bencher;
use chomp::*;
use mail::rfc5322::{message, raw_fields};

fn example_1_1_1(b: &mut Bencher) {
    let raw = include_bytes!("examples/1_1.1.eml");

    b.iter(|| {
        parse_only(raw_fields, raw)
    })
}

fn example_1_1_1_date(b: &mut Bencher) {
    let raw = include_bytes!("examples/1_1.1.eml");

    let msg = parse_only(message, raw).unwrap();
    let field = msg.date().unwrap();
    b.iter(|| {
        field.date_time()
    })
}

fn example_1_1_1_from(b: &mut Bencher) {
    let raw = include_bytes!("examples/1_1.1.eml");

    let msg = parse_only(message, raw).unwrap();
    let field = msg.from().unwrap();
    b.iter(|| {
        field.addresses()
    })
}

fn example_1_1_1_message_id(b: &mut Bencher) {
    let raw = include_bytes!("examples/1_1.1.eml");

    let msg = parse_only(message, raw).unwrap();
    let field = msg.message_id().unwrap();
    b.iter(|| {
        field.message_id()
    })
}

fn example_1_1_1_subject(b: &mut Bencher) {
    let raw = include_bytes!("examples/1_1.1.eml");

    let msg = parse_only(message, raw).unwrap();
    let field = msg.subject().unwrap();
    b.iter(|| {
        field.to_string()
    })
}

fn example_1_1_2(b: &mut Bencher) {
    let raw = include_bytes!("examples/1_1.2.eml");

    b.iter(|| {
        parse_only(raw_fields, raw)
    })
}

fn example_1_2(b: &mut Bencher) {
    let raw = include_bytes!("examples/1_2.eml");

    b.iter(|| {
        parse_only(raw_fields, raw)
    })
}

fn example_1_3(b: &mut Bencher) {
    let raw = include_bytes!("examples/1_3.eml");

    b.iter(|| {
        parse_only(raw_fields, raw)
    })
}

fn example_2_1(b: &mut Bencher) {
    let raw = include_bytes!("examples/2.1.eml");

    b.iter(|| {
        parse_only(raw_fields, raw)
    })
}

fn example_2_2(b: &mut Bencher) {
    let raw = include_bytes!("examples/2.2.eml");

    b.iter(|| {
        parse_only(raw_fields, raw)
    })
}

fn example_2_3(b: &mut Bencher) {
    let raw = include_bytes!("examples/2.3.eml");

    b.iter(|| {
        parse_only(raw_fields, raw)
    })
}

fn example_3_1(b: &mut Bencher) {
    let raw = include_bytes!("examples/3.1.eml");

    b.iter(|| {
        parse_only(raw_fields, raw)
    })
}

fn example_3_2(b: &mut Bencher) {
    let raw = include_bytes!("examples/3.2.eml");

    b.iter(|| {
        parse_only(raw_fields, raw)
    })
}

fn example_4(b: &mut Bencher) {
    let raw = include_bytes!("examples/4.eml");

    b.iter(|| {
        parse_only(raw_fields, raw)
    })
}

fn example_5(b: &mut Bencher) {
    let raw = include_bytes!("examples/5.eml");

    b.iter(|| {
        parse_only(raw_fields, raw)
    })
}

fn example_6_1(b: &mut Bencher) {
    let raw = include_bytes!("examples/6_1.eml");

    b.iter(|| {
        parse_only(raw_fields, raw)
    })
}

fn example_6_2(b: &mut Bencher) {
    let raw = include_bytes!("examples/6_2.eml");

    b.iter(|| {
        parse_only(raw_fields, raw)
    })
}

fn example_6_3(b: &mut Bencher) {
    let raw = include_bytes!("examples/6_3.eml");

    b.iter(|| {
        parse_only(raw_fields, raw)
    })
}

benchmark_group!(
    benches, 
    example_1_1_1,
    example_1_1_1_date,
    example_1_1_1_from,
    example_1_1_1_message_id,
    example_1_1_1_subject,
    example_1_1_2,
    example_1_2,
    example_1_3,
    example_2_1,
    example_2_2,
    example_2_3,
    example_3_1,
    example_3_2,
    example_4,
    example_5,
    example_6_1,
    example_6_2,
    example_6_3
    );

benchmark_main!(benches);
