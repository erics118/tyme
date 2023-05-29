use chrono::NaiveDateTime;

#[allow(dead_code)]
pub enum TimestampFormat {
    ShortTime,
    LongTime,
    ShortDate,
    LongDate,
    ShortDateTime,
    LongDateTime,
    Relative,
}
pub trait DiscordTimestamp {
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
