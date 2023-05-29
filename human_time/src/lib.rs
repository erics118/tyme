use anyhow::Result;
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
                num if num.chars().all(char::is_numeric) => n = num.parse::<u32>().unwrap(),
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

    pub fn to_string(&self) -> String {
        let mut res = String::new();

        if self.years > 0 {
            res += &format!(
                "{} year{}",
                self.years.to_string(),
                if self.years > 1 { "s" } else { "" }
            );
        }

        if self.months > 0 {
            res += &format!(
                "{} month{}",
                self.months.to_string(),
                if self.months > 1 { "s" } else { "" }
            );
        }

        if self.weeks > 0 {
            res += &format!(
                "{} week{}",
                self.weeks.to_string(),
                if self.weeks > 1 { "s" } else { "" }
            );
        }

        if self.days > 0 {
            res += &format!(
                "{} day{}",
                self.days.to_string(),
                if self.days > 1 { "s" } else { "" }
            );
        }

        if self.hours > 0 {
            res += &format!(
                "{} hour{}",
                self.hours.to_string(),
                if self.hours > 1 { "s" } else { "" }
            );
        }

        if self.minutes > 0 {
            res += &format!(
                "{} minute{}",
                self.minutes.to_string(),
                if self.minutes > 1 { "s" } else { "" }
            );
        }

        if self.seconds > 0 {
            res += &format!(
                "{} second{}",
                self.seconds.to_string(),
                if self.seconds > 1 { "s" } else { "" }
            );
        }

        res
    }

    pub fn add_to(&self, d: &mut NaiveDateTime) {
        *d = d
            .checked_add_days(Days::new((self.days + self.weeks * 7).into()))
            .unwrap();

        *d = d
            .checked_add_months(Months::new(self.months + self.years * 12))
            .unwrap();
        *d = d
            .checked_add_signed(Duration::seconds(self.seconds.into()))
            .unwrap();
        *d = d
            .checked_add_signed(Duration::hours(self.hours.into()))
            .unwrap();
        *d = d
            .checked_add_signed(Duration::minutes(self.minutes.into()))
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn aaa() {
        let a = super::HumanTime::parse("30 min").unwrap();
        let mut b = Utc::now().naive_utc();
        a.add_to(&mut b);
        println!("{:#?}", b);
        assert_eq!(
            format!("{:?}", super::HumanTime::parse("3 min 2 sec 50 hr")),
            "Ok(HumanTime { years: 0, months: 0, weeks: 0, days: 0, hours: 50, minutes: 3, seconds: 2 })"
        );
    }
}

// fn get_tokens(str: String) -> Vec<String> {
// let s = (str + " ").as_str();
// let mut res: Vec<String> = Vec::new();
// let mut start: usize = 0;
//
// for i in s.as_bytes().enum
// for i in 0..(s.len() - 1) {
// if (((s.as_bytes()[i] as char) as char).is_alphanumeric()  &&
// (s.as_bytes()[i+1] as char).is_whitespace())   // `a ` or `1 `
// ||  (((s.as_bytes()[i] as char) as char).is_numeric() && (s.as_bytes()[i + 1]
// as char).is_alphanumeric())     // `a1` || ((s.as_bytes()[i] as
// char).is_alphanumeric() && (s.as_bytes()[i + 1] as char).is_numeric()) `1a`
// {
// res.push(s[start..i].to_string());
// start = i + 1;
// }
//
// if ((s.as_bytes()[i] as char).is_whitespace()  && (s.as_bytes()[i+1] as
// char).is_alphanumeric())   // `a ` or `1 ` ||  ((s.as_bytes()[i] as
// char).is_alphanumeric() && (s.as_bytes()[i + 1] as char).is_numeric())     //
// `a1` || ((s.as_bytes()[i] as char).is_numeric() && (s.as_bytes()[i + 1]as
// char).is_alphanumeric()) `1a`
// {
// start = i + 1;
// }
// }
// println!("{res:#?}");
//
// return res;
// }