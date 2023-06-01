use chrono::NaiveDateTime;
use chrono_tz::Tz;
pub trait Pretty {
    fn pretty(&self, timezone: Tz) -> String;
}

impl Pretty for NaiveDateTime {
    fn pretty(&self, timezone: Tz) -> String {
        self.and_utc()
            .with_timezone(&timezone)
            .format("%h %e, %Y at %l:%M %p %Z")
            .to_string()
    }
}
