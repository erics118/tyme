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

pub mod reminders;
pub mod timezones;

pub use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
