//! Utilities for Discord timestamps

use chrono::NaiveDateTime;

/// Enum of all valid Discord timestamp formats.

#[derive(Debug, Copy, Clone)]
pub enum TimestampFormat {
    /// Short time.
    ///
    /// e.g 9:41 PM
    ShortTime,

    /// Long time.
    ///
    /// e.g. 9:41:30 PM
    LongTime,

    /// Short date.
    ///
    /// e.g. 30/06/2021
    ShortDate,

    /// Long date
    ///
    /// e.g. 30 June 2021.
    LongDate,

    /// Short date and time.
    ///
    /// e.g. 30 June 2021 9:41 PM
    ShortDateTime,

    /// Long date and time.
    ///
    /// e.g. Wednesday, June, 30, 2021 9:41 PM
    LongDateTime,

    /// Relative time.
    /// e.g. 7 months ago
    Relative,
}

/// Trait for to convert into a formatted Discord timestamp string.
pub trait DiscordTimestamp {
    /// Get a formatted Discord timestamp string for a given time.
    fn discord_timestamp(&self, t: TimestampFormat) -> String;
}

impl DiscordTimestamp for NaiveDateTime {
    fn discord_timestamp(&self, t: TimestampFormat) -> String {
        use TimestampFormat::*;
        format!(
            "<t:{}:{}>",
            self.timestamp(),
            match t {
                ShortTime => "t",
                LongTime => "T",
                ShortDate => "d",
                LongDate => "D",
                ShortDateTime => "f",
                LongDateTime => "F",
                Relative => "R",
            }
        )
    }
}
