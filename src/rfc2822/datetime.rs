use chomp::*;
use chrono::naive::time::NaiveTime;
use chrono::naive::date::NaiveDate;
use chrono::naive::datetime::NaiveDateTime;
use chrono::datetime::DateTime;
use chrono::offset::fixed::FixedOffset;

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
                option(fws, ());
                day_name()
            }},
            obs_day_of_week,
            )
    }
}

// day = ([FWS] 1*2DIGIT) / obs-day
pub fn day(i: Input<u8>) -> U8Result<usize> {
    parse!{i;
        or(
            |i| { parse!{i;
                option(fws, ());
                parse_digits((1..3))
            }},
            obs_day,
            )
    }
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
    parse!{i;
        or(
            |i| parse_digits(i, (4..)),
            obs_year,
            )
    }
}

// date = day month year
pub fn date(i: Input<u8>) -> U8Result<NaiveDate> {
    parse!{i;
        let d = day();
        let m = month();
        let y = year();

        ret NaiveDate::from_ymd(y as i32, m as u32, d as u32)
    }
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
    parse!{i;
        or(
            |i| { parse!{i;
                let s = satisfy(|i| i == b'+' || i == b'-');
                let n = parse_digits(4);

                ret match s {
                    b'+' => FixedOffset::east(n),
                    _ => FixedOffset::west(n),
                }
            }},
            obs_zone,
            )
    }
}

// time = time-of-day FWS zone
pub fn time(i: Input<u8>) -> U8Result<(NaiveTime, FixedOffset)> {
    parse!{i;
        let t = time_of_day();
        fws();
        let z = zone();

        ret (t, z)
    }
}

// date-time = [ day-of-week "," ] date FWS time [CFWS]
pub fn date_time(i: Input<u8>) -> U8Result<DateTime<FixedOffset>> {
    parse!{i;
        option(|i| parse!{i;
            day_of_week();
            token(b',');

            ret ()
        }, ());
        let d = date();
        fws();
        let t = time();

        ret DateTime::from_utc(NaiveDateTime::new(d, t.0), t.1)
    }
}
