#[macro_use]
extern crate bencher;
extern crate chomp;
extern crate mail;

use bencher::Bencher;
use chomp::*;
use mail::*;
use mail::rfc5322::raw_headers;

fn example_1_1_1(b: &mut Bencher) {
    let raw = include_bytes!("examples/1_1.1.eml");

    b.iter(|| {
        parse_only(raw_headers, raw)
    })
}

fn example_1_1_2(b: &mut Bencher) {
    let raw = include_bytes!("examples/1_1.2.eml");

    b.iter(|| {
        parse_only(raw_headers, raw)
    })
}

fn example_1_2(b: &mut Bencher) {
    let raw = include_bytes!("examples/1_2.eml");

    b.iter(|| {
        parse_only(raw_headers, raw)
    })
}

fn example_1_3(b: &mut Bencher) {
    let raw = include_bytes!("examples/1_3.eml");

    b.iter(|| {
        parse_only(raw_headers, raw)
    })
}

fn example_2_1(b: &mut Bencher) {
    let raw = include_bytes!("examples/2.1.eml");

    b.iter(|| {
        parse_only(raw_headers, raw)
    })
}

fn example_2_2(b: &mut Bencher) {
    let raw = include_bytes!("examples/2.2.eml");

    b.iter(|| {
        parse_only(raw_headers, raw)
    })
}

fn example_2_3(b: &mut Bencher) {
    let raw = include_bytes!("examples/2.3.eml");

    b.iter(|| {
        parse_only(raw_headers, raw)
    })
}

fn example_3_1(b: &mut Bencher) {
    let raw = include_bytes!("examples/3.1.eml");

    b.iter(|| {
        parse_only(raw_headers, raw)
    })
}

fn example_3_2(b: &mut Bencher) {
    let raw = include_bytes!("examples/3.2.eml");

    b.iter(|| {
        parse_only(raw_headers, raw)
    })
}

fn example_4(b: &mut Bencher) {
    let raw = include_bytes!("examples/4.eml");

    b.iter(|| {
        parse_only(raw_headers, raw)
    })
}

fn example_5(b: &mut Bencher) {
    let raw = include_bytes!("examples/5.eml");

    b.iter(|| {
        parse_only(raw_headers, raw)
    })
}

fn example_6_1(b: &mut Bencher) {
    let raw = include_bytes!("examples/6_1.eml");

    b.iter(|| {
        parse_only(raw_headers, raw)
    })
}

fn example_6_2(b: &mut Bencher) {
    let raw = include_bytes!("examples/6_2.eml");

    b.iter(|| {
        parse_only(raw_headers, raw)
    })
}

fn example_6_3(b: &mut Bencher) {
    let raw = include_bytes!("examples/6_3.eml");

    b.iter(|| {
        parse_only(raw_headers, raw)
    })
}

benchmark_group!(
    benches, 
    example_1_1_1,
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
