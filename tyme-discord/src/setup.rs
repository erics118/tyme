//! Utility functions to set up the bot.

use std::env;

use anyhow::{Context as _, Result};

/// Setup the logger.
pub fn setup_logger() {
    env_logger::Builder::new()
        .parse_default_env()
        .default_format()
        .format_indent(Some(4))
        .format_level(true)
        .format_module_path(true)
        .format_target(false)
        .format_timestamp_millis()
        .init();
}

/// Get the `DISCORD_TOKEN` env var.
///
/// # Errors
///
/// Returns an error if the `DISCORD_TOKEN` env var is not set.
pub fn get_discord_token() -> Result<String> {
    env::var("DISCORD_TOKEN").context("Missing `DISCORD_TOKEN` env var")
}

/// Get the `DATABASE_URL` env var.
///
/// # Errors
///
/// Returns an error if the `DATABASE_URL` env var is not set.
pub fn get_database_url() -> Result<String> {
    env::var("DATABASE_URL").context("Missing `DATABASE_URL` env var")
}
