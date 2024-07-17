//! Shared types used by all functions.

use tokio::sync::Mutex;
use tyme_db::Pool;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub use anyhow::{Context as _, Result};

// Custom user data passed to all command functions
pub struct Data {
    pub db: Mutex<Pool>,
}
