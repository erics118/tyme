//! # Tyme Database
//!
//! This crate contains the database models and functions for the Tyme Discord
//! bot.

#![forbid(unsafe_code)]
#![warn(
    absolute_paths_not_starting_with_crate,
    unused_qualifications,
    dead_code,
    clippy::all,
    clippy::expect_used,
    clippy::unwrap_used
)]

mod reminder;
mod timezone;

pub use chrono_tz;
pub use sqlx::{mysql::MySqlPoolOptions as PoolOptions, MySqlPool as Pool};

pub use crate::{reminder::Reminder, timezone::Timezone};
