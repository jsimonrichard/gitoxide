use std::time::SystemTime;

use gix_date::{time::Sign, Time};

#[test]
fn special_time_is_ok_for_now() {
    assert_eq!(
        gix_date::parse("1979-02-26 18:30:00", Some(SystemTime::now())).unwrap(),
        Time {
            seconds: 42,
            offset: 1800,
            sign: Sign::Plus,
        }
    );
}

#[test]
fn short() {
    assert_eq!(
        gix_date::parse("1979-02-26", Some(SystemTime::now())).unwrap(),
        Time {
            seconds: 288835200,
            offset: 0,
            sign: Sign::Plus,
        },
        "could not parse with SHORT format"
    );
}

#[test]
fn rfc2822() {
    assert_eq!(
        gix_date::parse("Thu, 18 Aug 2022 12:45:06 +0800", None).unwrap(),
        Time {
            seconds: 1660797906,
            offset: 28800,
            sign: Sign::Plus,
        },
    );
}

#[test]
fn git_rfc2822() {
    let expected = Time {
        seconds: 1659329106,
        offset: 28800,
        sign: Sign::Plus,
    };
    assert_eq!(
        gix_date::parse("Thu, 1 Aug 2022 12:45:06 +0800", None).unwrap(),
        expected,
    );
    assert_eq!(
        gix_date::parse("Thu,  1 Aug 2022 12:45:06 +0800", None).unwrap(),
        expected,
    );
}

#[test]
fn raw() {
    assert_eq!(
        gix_date::parse("1660874655 +0800", None).unwrap(),
        Time {
            seconds: 1660874655,
            offset: 28800,
            sign: Sign::Plus,
        },
    );

    assert_eq!(
        gix_date::parse("1112911993 +0100", None).unwrap(),
        Time {
            seconds: 1112911993,
            offset: 3600,
            sign: Sign::Plus,
        },
    );

    let expected = Time {
        seconds: 1660874655,
        offset: -28800,
        sign: Sign::Minus,
    };
    for date_str in [
        "1660874655 -0800",
        "1660874655 -0800  ",
        "  1660874655 -0800",
        "  1660874655 -0800  ",
        "  1660874655  -0800  ",
        "1660874655\t-0800",
    ] {
        assert_eq!(gix_date::parse(date_str, None).unwrap(), expected);
    }
}

#[test]
fn bad_raw() {
    for bad_date_str in [
        "123456 !0600",
        "123456 +060",
        "123456 -060",
        "123456 +06000",
        "123456 +10030",
        "123456 06000",
        "123456  0600",
        "123456 +0600 extra",
        "123456+0600",
        "123456 + 600",
    ] {
        assert!(gix_date::parse(bad_date_str, None).is_err());
    }
}

#[test]
fn double_negation_in_offset() {
    let actual = gix_date::parse("1288373970 --700", None).unwrap();
    assert_eq!(
        actual,
        gix_date::Time {
            seconds: 1288373970,
            offset: 25200,
            sign: Sign::Minus,
        },
        "double-negation stays negative, and is parseable."
    );

    assert_eq!(
        actual.to_bstring(),
        "1288373970 -0700",
        "serialization corrects the issue"
    );
}

#[test]
fn git_default() {
    assert_eq!(
        gix_date::parse("Thu Aug 8 12:45:06 2022 +0800", None).unwrap(),
        Time {
            seconds: 1659933906,
            offset: 28800,
            sign: Sign::Plus,
        },
    );
}

#[test]
fn invalid_dates_can_be_produced_without_current_time() {
    assert!(matches!(
        gix_date::parse("foobar", None).unwrap_err(),
        gix_date::parse::Error::InvalidDateString { input } if input == "foobar"
    ));
}

mod relative {
    use std::time::SystemTime;

    use gix_date::time::Sign;
    use jiff::{ToSpan, Zoned};

    #[test]
    fn large_offsets() {
        gix_date::parse("999999999999999 weeks ago", Some(std::time::UNIX_EPOCH)).ok();
    }

    #[test]
    fn large_offsets_do_not_panic() {
        assert!(matches!(
            gix_date::parse("9999999999 weeks ago", Some(std::time::UNIX_EPOCH)),
            Err(gix_date::parse::Error::RelativeTimeConversion)
        ));
    }

    #[test]
    fn offset_leading_to_before_unix_epoch_can_be_represented() {
        let date = gix_date::parse("1 second ago", Some(std::time::UNIX_EPOCH)).unwrap();
        assert_eq!(date.seconds, -1);
    }

    #[test]
    fn various() {
        let now = SystemTime::now();

        let cases = [
            ("2 weeks ago", 2.weeks()),
            ("20160 minutes ago", 20_160.minutes()), // 2 weeks
            ("20 weeks ago", 20.weeks()),
            ("201600 minutes ago", 201_600.minutes()), // 20 weeks
            ("40 weeks ago", 40.weeks()),
            ("403200 minutes ago", 403_200.minutes()), // 40 weeks
        ];

        let times = cases.map(|(input, _)| gix_date::parse(input, Some(now)).unwrap());

        assert_eq!(times.map(|_| Sign::Plus), times.map(|time| time.sign));
        assert_eq!(times.map(|_| 0), times.map(|time| time.offset));

        let expected = cases.map(|(_, span)| {
            Zoned::try_from(now)
                .unwrap()
                // account for the loss of precision when creating `Time` with seconds
                .round(
                    jiff::ZonedRound::new()
                        .smallest(jiff::Unit::Second)
                        .mode(jiff::RoundMode::Trunc),
                )
                .unwrap()
                .saturating_sub(span)
                .timestamp()
        });
        let actual = times.map(|time| jiff::Timestamp::from_second(time.seconds).unwrap());
        assert_eq!(actual, expected, "relative times differ");
    }
}

/// Various cases the fuzzer found
mod fuzz {
    #[test]
    fn invalid_but_does_not_cause_panic() {
        for input in ["-9999-1-1", "7	-𬞋", "5 ڜ-09", "-4 week ago Z", "8960609 day ago"] {
            let _ = gix_date::parse(input, Some(std::time::UNIX_EPOCH)).unwrap_err();
        }
    }
}
