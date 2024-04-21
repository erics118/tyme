//! Nice, humanized duration parsing and formatting.
//!
//! It is used to parse a string into a human readable time, store it in a nice
//! struct, and also to convert it back into a string.

use std::fmt::Display;

use anyhow::{Context, Result};
use chrono::{Days, Duration, Months, NaiveDateTime};

/// Struct that represents a human understandable time.
#[derive(PartialEq, Eq, Default, Copy, Clone, Debug)]
pub struct HumanTime {
    /// The number of years.
    pub years: u32,

    /// The number of months.
    pub months: u32,

    /// The number of weeks.
    pub weeks: u32,

    /// The number of days.
    pub days: u32,

    /// The number of hours.
    pub hours: u32,

    /// The number of minutes.
    pub minutes: u32,

    /// The number of seconds.
    pub seconds: u32,
}

/// Tokenize a string into parsable tokens.
fn get_tokens(s: &str) -> Vec<String> {
    if s.len() < 2 {
        return Vec::new();
    }

    let chars = s
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<char>>();

    let windows = chars.windows(2);

    let mut tok = String::new();

    let mut vec: Vec<String> = Vec::new();

    for w in windows {
        let a = w[0];
        let b = w[1];
        tok.push(a);
        if ((a.is_alphabetic() && b.is_numeric()) || (a.is_numeric() && b.is_alphabetic()))
            && !tok.is_empty()
        {
            vec.push(tok.clone());
            tok.clear();
        }
    }

    if let Some(last) = chars.last() {
        tok.push(*last);
    }

    if !tok.is_empty() {
        vec.push(tok.clone());
    }
    vec
}

impl HumanTime {
    /// Parse a string into a `HumanTime`.
    pub fn parse(s: &str) -> Result<Self> {
        let mut res = Self::default();

        let tokens = get_tokens(s);

        if tokens.len() % 2 != 0 || tokens.len() < 2 {
            anyhow::bail!("incorrect number of tokens");
        }

        let mut n: u32 = 0;

        for t in tokens {
            match t.to_lowercase().as_str() {
                num if num.chars().all(char::is_numeric) => n = num.parse::<u32>()?,
                "y" | "yr" | "yrs" | "year" | "years" => {
                    res.years += n;
                    n = 0;
                },
                "mo" | "mos" | "mon" | "month" | "months" => {
                    res.months += n;
                    n = 0;
                },
                "w" | "wk" | "week" | "weeks" => {
                    res.weeks += n;
                    n = 0;
                },
                "d" | "day" | "days" => {
                    res.days += n;
                    n = 0;
                },
                "h" | "hr" | "hrs" | "hour" | "hours" => {
                    res.hours += n;
                    n = 0;
                },
                "m" | "min" | "mins" | "minute" | "minutes" => {
                    res.minutes += n;
                    n = 0;
                },
                "s" | "sec" | "secs" | "second" | "seconds" => {
                    res.seconds += n;
                    n = 0;
                },
                _ => (),
            }
        }

        Ok(res)
    }

    /// Make each field have the lowest possible value, to be easily understood.
    pub fn cleanup(&mut self) {
        if self.seconds >= 60 {
            self.minutes += self.seconds / 60;
            self.seconds %= 60;
        }

        if self.minutes >= 60 {
            self.hours += self.minutes / 60;
            self.minutes %= 60;
        }

        if self.hours >= 24 {
            self.days += self.hours / 24;
            self.hours %= 24;
        }

        if self.days >= 7 {
            self.weeks += self.days / 7;
            self.days %= 7;
        }

        if self.months >= 12 {
            self.years += self.months / 12;
            self.months %= 12;
        }
    }
}

/// Trait to add a `HumanTime` to a `NaiveDateTime`.
pub trait CheckedAddHumanTime {
    /// Add a `HumanTime` to a `NaiveDateTime`.
    ///
    /// # Errors
    ///
    /// Returns an error if the addition overflows.
    fn checked_add(self, a: HumanTime) -> Result<Self>
    where
        Self: Sized;
}

impl CheckedAddHumanTime for NaiveDateTime {
    fn checked_add(self, rhs: HumanTime) -> Result<Self> {
        let mut a = self;

        a = a
            .checked_add_months(Months::new(rhs.months + rhs.years * 12))
            .context("checked add overflow for months and years")?;

        a = a
            .checked_add_days(Days::new((rhs.days + rhs.weeks * 7).into()))
            .context("checked add overflow for days and weeks")?;

        a = a
            .checked_add_signed(Duration::hours(rhs.hours.into()))
            .context("checked add overflow for hours")?;

        a = a
            .checked_add_signed(Duration::minutes(rhs.minutes.into()))
            .context("checked add overflow for minutes")?;

        a = a
            .checked_add_signed(Duration::seconds(rhs.seconds.into()))
            .context("checked add overflow for seconds")?;

        if a == self {
            anyhow::bail!("zero time added");
        }

        Ok(a)
    }
}

fn pluralized(n: u32, s: &str) -> String {
    format!("{n}{s}{}", if n > 1 { "s" } else { "" })
}

impl Display for HumanTime {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let units = [
            (self.years, "yr"),
            (self.months, "mo"),
            (self.weeks, "wk"),
            (self.days, "day"),
            (self.hours, "hr"),
            (self.minutes, "min"),
            (self.seconds, "sec"),
        ];

        // Filter out zero values, and pluralize the unit if necessary.
        let res = units
            .iter()
            .map(|(value, unit)| pluralized(*value, unit))
            .collect::<Vec<_>>()
            .join(" ");

        fmt.write_str(&res)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::HumanTime;

    #[test]
    fn parse_with_spaces() {
        assert_eq!(
            HumanTime::parse("3 min 2 sec 50 hr").unwrap(),
            HumanTime {
                years: 0,
                months: 0,
                weeks: 0,
                days: 0,
                hours: 50,
                minutes: 3,
                seconds: 2,
            }
        );
    }

    #[test]
    fn parse_no_spaces() {
        assert_eq!(
            HumanTime::parse("3min2sec50hr").unwrap(),
            HumanTime {
                years: 0,
                months: 0,
                weeks: 0,
                days: 0,
                hours: 50,
                minutes: 3,
                seconds: 2,
            }
        );
    }

    #[test]
    fn parse_all_short_no_spaces() {
        assert_eq!(
            HumanTime::parse("30s8m7h3d2w9mo3y").unwrap(),
            HumanTime {
                years: 3,
                months: 9,
                weeks: 2,
                days: 3,
                hours: 7,
                minutes: 8,
                seconds: 30,
            }
        );
    }

    #[test]
    fn parse_all_long_no_spaces() {
        assert_eq!(
            HumanTime::parse("30seconds8minutes7hours3days2weeks9months3years").unwrap(),
            HumanTime {
                years: 3,
                months: 9,
                weeks: 2,
                days: 3,
                hours: 7,
                minutes: 8,
                seconds: 30,
            }
        );
    }

    #[test]
    fn parse_all_short_with_spaces() {
        assert_eq!(
            HumanTime::parse("30 s 8 m 7 h 3 d 2 w 9 mo 3 y").unwrap(),
            HumanTime {
                years: 3,
                months: 9,
                weeks: 2,
                days: 3,
                hours: 7,
                minutes: 8,
                seconds: 30,
            }
        );
    }

    #[test]
    fn parse_all_long_with_spaces() {
        assert_eq!(
            HumanTime::parse(" 30seconds 8 minutes7hours 3days 2weeks  9months3    years").unwrap(),
            HumanTime {
                years: 3,
                months: 9,
                weeks: 2,
                days: 3,
                hours: 7,
                minutes: 8,
                seconds: 30,
            }
        );
    }

    #[test]
    fn parse_all_short_no_spaces_mixed() {
        assert_eq!(
            HumanTime::parse("7h3y3d9mo8m30s2w").unwrap(),
            HumanTime {
                years: 3,
                months: 9,
                weeks: 2,
                days: 3,
                hours: 7,
                minutes: 8,
                seconds: 30,
            }
        );
    }

    #[test]
    fn parse_zero_and_normal() {
        assert_eq!(
            HumanTime::parse("0day 1min").unwrap(),
            HumanTime {
                years: 0,
                months: 0,
                weeks: 0,
                days: 0,
                hours: 0,
                minutes: 1,
                seconds: 0,
            }
        );
    }

    #[test]
    fn parse_err_zero() {
        assert!(HumanTime::parse("0").is_err());
    }

    #[test]
    fn parse_err_simple() {
        assert!(HumanTime::parse("abc").is_err());
    }

    #[test]
    fn parse_err_one_char() {
        assert!(HumanTime::parse("a").is_err());
    }

    #[test]
    fn parse_err_one_digit() {
        assert!(HumanTime::parse("1").is_err());
    }

    #[test]
    fn parse_err_empty() {
        assert!(HumanTime::parse("").is_err());
    }

    #[test]
    fn parse_err_missing_token1() {
        assert!(HumanTime::parse("3 min 7").is_err());
    }

    #[test]
    fn parse_err_missing_token2() {
        assert!(HumanTime::parse("min 7 sec").is_err());
    }

    #[test]
    fn parse_err_overflow() {
        assert!(HumanTime::parse("3 min 2131283123128397893782737 abc").is_err());
    }

    #[test]
    fn parse_mixed() {
        assert_eq!(
            HumanTime::parse("3 min 7 abc").unwrap(),
            HumanTime {
                years: 0,
                months: 0,
                weeks: 0,
                days: 0,
                hours: 0,
                minutes: 3,
                seconds: 0,
            }
        );
    }

    #[test]
    fn display_normal() {
        assert_eq!(
            HumanTime::parse("30s8m7h3d2w9mo3y").unwrap().to_string(),
            "3yrs 9mos 2wks 3days 7hrs 8mins 30secs"
        );
    }

    #[test]
    fn cleanup_months() {
        let mut a = HumanTime::parse("100months").unwrap();
        a.cleanup();

        assert_eq!(
            a,
            HumanTime {
                years: 8,
                months: 4,
                weeks: 0,
                days: 0,
                hours: 0,
                minutes: 0,
                seconds: 0,
            }
        );
    }

    #[test]
    fn cleanup_days() {
        let mut a = HumanTime::parse("100days").unwrap();
        a.cleanup();

        assert_eq!(
            a,
            HumanTime {
                years: 0,
                months: 0,
                weeks: 14,
                days: 2,
                hours: 0,
                minutes: 0,
                seconds: 0,
            }
        );
    }

    #[test]
    fn cleanup_hours() {
        let mut a = HumanTime::parse("100hours").unwrap();
        a.cleanup();

        assert_eq!(
            a,
            HumanTime {
                years: 0,
                months: 0,
                weeks: 0,
                days: 4,
                hours: 4,
                minutes: 0,
                seconds: 0,
            }
        );
    }

    #[test]
    fn cleanup_minutes() {
        let mut a = HumanTime::parse("100minutes").unwrap();
        a.cleanup();

        assert_eq!(
            a,
            HumanTime {
                years: 0,
                months: 0,
                weeks: 0,
                days: 0,
                hours: 1,
                minutes: 40,
                seconds: 0
            }
        );
    }

    #[test]
    fn cleanup_seconds() {
        let mut a = HumanTime::parse("100seconds").unwrap();
        a.cleanup();

        assert_eq!(
            a,
            HumanTime {
                years: 0,
                months: 0,
                weeks: 0,
                days: 0,
                hours: 0,
                minutes: 1,
                seconds: 40,
            }
        );
    }
}
