#![forbid(unsafe_code)]
#![warn(
    explicit_outlives_requirements,
    elided_lifetimes_in_paths,
    unused_qualifications,
    clippy::all,
    clippy::nursery,
    clippy::expect_used,
    clippy::unwrap_used
)]

mod reminders;
mod timezones;

pub use reminders::reminder::Reminder;
pub use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
pub use timezones::timezone::Timezone;
