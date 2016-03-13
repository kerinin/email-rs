use chomp::*;
use chrono::naive::time::NaiveTime;
use chrono::naive::date::NaiveDate;
use chrono::naive::datetime::NaiveDateTime;
use chrono::datetime::DateTime;
use chrono::offset::fixed::FixedOffset;
use bytes::Bytes;

use util::*;
use rfc2822::*;
use rfc2822::folding::*;
use rfc2822::obsolete::*;

// day-name        =       "Mon" / "Tue" / "Wed" / "Thu" /
//                         "Fri" / "Sat" / "Sun"
pub fn day_name(i: Input<u8>) -> U8Result<Day> {
    or(i, |i| string(i, b"Mon").then(|i| i.ret(Day::Mon)),
    |i| or(i, |i| string(i, b"Tue").then(|i| i.ret(Day::Tue)),
    |i| or(i, |i| string(i, b"Wed").then(|i| i.ret(Day::Wed)),
    |i| or(i, |i| string(i, b"Thu").then(|i| i.ret(Day::Thu)),
    |i| or(i, |i| string(i, b"Fri").then(|i| i.ret(Day::Fri)),
    |i| or(i, |i| string(i, b"Sat").then(|i| i.ret(Day::Sat)),
    |i| string(i, b"Sun").then(|i| i.ret(Day::Sun))))))))
}

// day-of-week = ([FWS] day-name) / obs-day-of-week
pub fn day_of_week(i: Input<u8>) -> U8Result<Day> {
    parse!{i;
        or(
            |i| { parse!{i;
                option(fws, Bytes::empty());
                day_name()
            }},
            obs_day_of_week,
            )
    }
}

// day = ([FWS] 1*2DIGIT) / obs-day
pub fn day(i: Input<u8>) -> U8Result<usize> {
    println!("day({:?})", i);
    let a = |i| {
        println!("day.a({:?})", i);
        option(i, fws, Bytes::empty()).then(|i| {
            println!("day.option(fws).then({:?})", i);
            parse_digits(i, (1..3))
        })
    };

    or(i, a, obs_day)
}

// month-name      =       "Jan" / "Feb" / "Mar" / "Apr" /
//                         "May" / "Jun" / "Jul" / "Aug" /
//                         "Sep" / "Oct" / "Nov" / "Dec"
pub fn month_name(i: Input<u8>) -> U8Result<Month> {
    or(i, |i| string(i, b"Jan").then(|i| i.ret(Month::Jan)),
    |i| or(i, |i| string(i, b"Feb").then(|i| i.ret(Month::Feb)),
    |i| or(i, |i| string(i, b"Mar").then(|i| i.ret(Month::Mar)),
    |i| or(i, |i| string(i, b"Apr").then(|i| i.ret(Month::Apr)),
    |i| or(i, |i| string(i, b"May").then(|i| i.ret(Month::May)),
    |i| or(i, |i| string(i, b"Jun").then(|i| i.ret(Month::Jun)),
    |i| or(i, |i| string(i, b"Jul").then(|i| i.ret(Month::Jul)),
    |i| or(i, |i| string(i, b"Aug").then(|i| i.ret(Month::Aug)),
    |i| or(i, |i| string(i, b"Sep").then(|i| i.ret(Month::Sep)),
    |i| or(i, |i| string(i, b"Oct").then(|i| i.ret(Month::Oct)),
    |i| or(i, |i| string(i, b"Nov").then(|i| i.ret(Month::Nov)),
    |i| string(i, b"Dec").then(|i| i.ret(Month::Dec)))))))))))))
}

// month = (FWS month-name FWS) / obs-month
pub fn month(i: Input<u8>) -> U8Result<Month> {
    parse!{i;
        or(
            |i| { parse!{i;
                fws();
                let m = month_name();
                fws();

                ret m
            }},
            obs_month,
            )
    }
}

// year = 4*DIGIT / obs-year
pub fn year(i: Input<u8>) -> U8Result<usize> {
    let a = |i| {
        parse_digits(i, (4..)).bind(|i, v| {
            println!("year.parse_digits.bind({:?}, {:?})", i, v);
            i.ret(v)
        })
    };

    let b = |i| {
        obs_year(i).bind(|i, v| {
            println!("year.obs_year.bind({:?}, {:?})", i, v);
            i.ret(v)
        })
    };

    or(i, a, b)
}

// date = day month year
pub fn date(i: Input<u8>) -> U8Result<NaiveDate> {
    println!("date({:?})", i);

    day(i).bind(|i, d| {
        println!("date.day.bind({:?}, {:?})", i, d);

        month(i).bind(|i, m| {
            println!("date.month.bind({:?}, {:?})", i, m);

            year(i).bind(|i, y| {
                println!("date.year.bind({:?}, {:?})", i, y);

                i.ret(NaiveDate::from_ymd(y as i32, m as u32, d as u32))
            })
        })
    })
}

#[test]
fn test_date() {
    let i = b"21 Nov 97";
    let msg = parse_only(date, i);
    assert!(msg.is_ok());
}

// hour = 2DIGIT / obs-hour
pub fn hour(i: Input<u8>) -> U8Result<usize> {
    parse!{i;
        or(
            |i| parse_digits(i, 2),
            obs_hour,
            )
    }
}

// minute = 2DIGIT / obs-minute
pub fn minute(i: Input<u8>) -> U8Result<usize> {
    parse!{i;
        or(
            |i| parse_digits(i, 2),
            obs_minute,
            )
    }
}

// second = 2DIGIT / obs-second
pub fn second(i: Input<u8>) -> U8Result<usize> {
    parse!{i;
        or(
            |i| parse_digits(i, 2),
            obs_second,
            )
    }
}
// time-of-day = hour ":" minute [ ":" second ]
pub fn time_of_day(i: Input<u8>) -> U8Result<NaiveTime> {
    parse!{i;
        let h = hour();
        token(b':');
        let m = minute();
        let s = option(|i| { parse!{i;
            token(b':');
            second()
        }}, 0);

        ret NaiveTime::from_hms(h as u32, m as u32, s as u32)
    }
}

// zone = (( "+" / "-" ) 4DIGIT) / obs-zone
pub fn zone(i: Input<u8>) -> U8Result<FixedOffset> {
    println!("zone({:?})", i);
    
    let a = |i| {
        or(i, |i| token(i, b'+'), |i| token(i, b'-')).bind(|i, sign| {
            println!("zone.or(+,-).bind(({:?}, {:?})", i, sign);

            parse_digits(i, 4).bind(|i, offset| {
                println!("zone.parse_digits(4).bind(({:?}, {:?})", i, offset);

                let zone = match sign {
                    b'+' => FixedOffset::east(offset),
                    _ => FixedOffset::west(offset),
                };

                i.ret(zone)
            })
        })
    };

    or(i, a, obs_zone)
}

#[test]
fn test_zone() {
    let i = b"-0330";
    let msg = parse_only(zone, i);
    assert!(msg.is_ok());
}

// time = time-of-day FWS zone
pub fn time(i: Input<u8>) -> U8Result<(NaiveTime, FixedOffset)> {
    time_of_day(i).bind(|i, t| {
        println!("time.time_of_day.bind({:?}, {:?})", i, t);

        fws(i).then(|i| {
            println!("time.fws");

            zone(i).bind(|i, z| {
                println!("time.zone.bind({:?}, {:?})", i, z);

                i.ret((t, z))
            })
        })
    })
}

#[test]
fn test_time() {
    let i = b"09:55:06 GMT";
    let msg = parse_only(time, i);
    assert!(msg.is_ok());

    let i = b"23:32\r\n               -0330";
    let msg = parse_only(time, i);
    assert!(msg.is_ok());
}

// date-time = [ day-of-week "," ] date FWS time [CFWS]
pub fn date_time(i: Input<u8>) -> U8Result<DateTime<FixedOffset>> {
    println!("date_time({:?})", i);

    option(i, |i| {
        println!("date_time.option({:?})", i);

        day_of_week(i).then(|i| {
            println!("date_time.day_of_week({:?})", i);

            token(i, b',').then(|i| {
                println!("date_time.token(b).then({:?})", i);

                i.ret(())
            })
        })
    }, ()).then(|i| {
        println!("date_time.option.then({:?})", i);

        date(i).bind(|i, d| {
            println!("date_time.date.bind({:?}, {:?})", i, d);

            fws(i).then(|i| {
                println!("date_time.fws.then({:?})", i);

                time(i).bind(|i, t| {
                    println!("date_time.time.bind({:?}, {:?})", i, t);

                    option(i, cfws, Bytes::empty()).then(|i| {
                        println!("date_time.option(cfws).then({:?})", i);

                        i.ret(DateTime::from_utc(NaiveDateTime::new(d, t.0), t.1))
                    })
                })
            })
        })
    })
}

#[test]
fn test_date_time() {
    let i = b"21 Nov 97 09:55:06 GMT";
    let msg = parse_only(date_time, i);
    assert!(msg.is_ok());

    let i = b"Thu,\r\n      13\r\n        Feb\r\n          1969\r\n 23:32\r\n               -0330 (Newfoundland Time)\r\n";
    let msg = parse_only(date_time, i);
    assert!(msg.is_ok());
}
