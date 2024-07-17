//! Pretty formatting for dates and times.

use chrono::NaiveDateTime;
use tyme_db::chrono_tz::Tz;

/// Trait for pretty formatting dates and times.
pub trait Pretty {
    /// Get a pretty formatted date and time for a given timezone.
    fn pretty(&self, timezone: Tz) -> String;

    /// Utility function to format a date and time with a given format and
    /// timezone.
    fn format(&self, format: &str, timezone: Tz) -> String;
}

impl Pretty for NaiveDateTime {
    fn pretty(&self, timezone: Tz) -> String {
        self.and_utc()
            .with_timezone(&timezone)
            .format("%h %e, %Y at %l:%M %p %Z")
            .to_string()
    }

    fn format(&self, format: &str, timezone: Tz) -> String {
        self.and_utc()
            .with_timezone(&timezone)
            .format(format)
            .to_string()
    }
}
