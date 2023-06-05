use std::fmt::Display;

use anyhow::{Context, Result};
use chrono::{Days, Duration, Months, NaiveDateTime};

#[derive(PartialEq, Eq, Default, Copy, Clone, Debug)]
pub struct HumanTime {
    pub years: u32,
    pub months: u32,
    pub weeks: u32,
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

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

    // guaranteed not to error, because s is at least 2 chars long
    #[allow(clippy::unwrap_used)]
    tok.push(*chars.last().unwrap());

    if !tok.is_empty() {
        vec.push(tok.clone());
    }
    vec
}

impl HumanTime {
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

pub trait CheckedAddHumanTime {
    fn checked_add(self, a: HumanTime) -> Result<Self>
    where
        Self: Sized;
}

impl CheckedAddHumanTime for NaiveDateTime {
    fn checked_add(self, rhs: HumanTime) -> Result<Self> {
        let mut a = self;

        a = a
            .checked_add_months(Months::new(rhs.months + rhs.years * 12))
            .context("checked add overflow")?;

        a = a
            .checked_add_days(Days::new((rhs.days + rhs.weeks * 7).into()))
            .context("checked add overflow")?;

        a = a
            .checked_add_signed(Duration::hours(rhs.hours.into()))
            .context("checked add overflow")?;

        a = a
            .checked_add_signed(Duration::minutes(rhs.minutes.into()))
            .context("checked add overflow")?;

        a = a
            .checked_add_signed(Duration::seconds(rhs.seconds.into()))
            .context("checked add overflow")?;

        Ok(a)
    }
}

impl Display for HumanTime {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        if self.years > 0 {
            res += &format!(
                "{}yr{} ",
                self.years.to_string(),
                if self.years > 1 { "s" } else { "" }
            );
        }

        if self.months > 0 {
            res += &format!(
                "{}mo{} ",
                self.months.to_string(),
                if self.months > 1 { "s" } else { "" }
            );
        }

        if self.weeks > 0 {
            res += &format!(
                "{}wk{} ",
                self.weeks.to_string(),
                if self.weeks > 1 { "s" } else { "" }
            );
        }

        if self.days > 0 {
            res += &format!(
                "{}day{} ",
                self.days.to_string(),
                if self.days > 1 { "s" } else { "" }
            );
        }

        if self.hours > 0 {
            res += &format!(
                "{}hr{} ",
                self.hours.to_string(),
                if self.hours > 1 { "s" } else { "" }
            );
        }

        if self.minutes > 0 {
            res += &format!(
                "{}min{} ",
                self.minutes.to_string(),
                if self.minutes > 1 { "s" } else { "" }
            );
        }

        if self.seconds > 0 {
            res += &format!(
                "{}sec{} ",
                self.seconds.to_string(),
                if self.seconds > 1 { "s" } else { "" }
            );
        }

        if res.ends_with(' ') {
            res.pop();
        }

        fmt.write_str(&res)?;

        Ok(())
    }
}

#[cfg(test)]
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
    fn parse_err_mixed() {
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
    fn parse_err_overflow() {
        assert!(HumanTime::parse("3 min 2131283123128397893782737 abc").is_err());
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
