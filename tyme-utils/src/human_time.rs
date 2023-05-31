use std::fmt::Display;

use anyhow::{Context, Result};
use chrono::{Days, Duration, Months, NaiveDateTime};
use slice_group_by::StrGroupBy;

#[derive(Default, Copy, Clone, Debug)]
pub struct HumanTime {
    pub years: u32,
    pub months: u32,
    pub weeks: u32,
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

fn get_tokens(s: &str) -> Vec<&str> {
    s.linear_group_by_key(|c: char| (c.is_alphabetic() as u8) * 2 + c.is_numeric() as u8)
        .filter(|s| !s.trim().is_empty())
        .collect()
}

impl HumanTime {
    pub fn parse(s: &str) -> Result<Self> {
        let mut res = Self::default();

        let tokens = get_tokens(s);

        if tokens.len() % 2 != 0 {
            anyhow::bail!("f")
        }

        let mut n: u32 = 0;

        for t in tokens {
            match t {
                num if num.chars().all(char::is_numeric) => n = num.parse::<u32>()?,
                "y" | "yr" | "yrs" | "year" | "years" => {
                    res.years += n;
                    n = 0
                },
                "mo" | "mos" | "mon" | "month" | "months" => {
                    res.months += n;
                    n = 0
                },
                "w" | "wk" | "week" | "weeks" => {
                    res.weeks += n;
                    n = 0
                },
                "d" | "day" | "days" => {
                    res.days += n;
                    n = 0
                },
                "h" | "hr" | "hrs" | "hour" | "hours" => {
                    res.hours += n;
                    n = 0
                },
                "m" | "min" | "mins" | "minute" | "minutes" => {
                    res.minutes += n;
                    n = 0
                },
                "s" | "sec" | "secs" | "second" | "seconds" => {
                    res.seconds += n;
                    n = 0
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
        if self.years > 0 {
            fmt.write_str(&format!(
                "{} year{}",
                self.years.to_string(),
                if self.years > 1 { "s" } else { "" }
            ))?;
        }

        if self.months > 0 {
            fmt.write_str(&format!(
                "{} month{}",
                self.months.to_string(),
                if self.months > 1 { "s" } else { "" }
            ))?;
        }

        if self.weeks > 0 {
            fmt.write_str(&format!(
                "{} week{}",
                self.weeks.to_string(),
                if self.weeks > 1 { "s" } else { "" }
            ))?;
        }

        if self.days > 0 {
            fmt.write_str(&format!(
                "{} day{}",
                self.days.to_string(),
                if self.days > 1 { "s" } else { "" }
            ))?;
        }

        if self.hours > 0 {
            fmt.write_str(&format!(
                "{} hour{}",
                self.hours.to_string(),
                if self.hours > 1 { "s" } else { "" }
            ))?;
        }

        if self.minutes > 0 {
            fmt.write_str(&format!(
                "{} minute{}",
                self.minutes.to_string(),
                if self.minutes > 1 { "s" } else { "" }
            ))?;
        }

        if self.seconds > 0 {
            fmt.write_str(&format!(
                "{} second{}",
                self.seconds.to_string(),
                if self.seconds > 1 { "s" } else { "" }
            ))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn debug_normal() {
        assert_eq!(
            format!("{:?}", super::HumanTime::parse("3 min 2 sec 50 hr")),
            "Ok(HumanTime { years: 0, months: 0, weeks: 0, days: 0, hours: 50, minutes: 3, seconds: 2 })"
        );
    }

    #[test]
    fn debug_no() {
        assert_eq!(
            format!("{:?}", super::HumanTime::parse("3 min 2 sec 50 hr")),
            "Ok(HumanTime { years: 0, months: 0, weeks: 0, days: 0, hours: 50, minutes: 3, seconds: 2 })"
        );
    }
}
