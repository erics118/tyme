use std::env;

use anyhow::{Context, Result};
use log::LevelFilter;

#[allow(dead_code)]
pub fn setup_logger() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Off)
        .filter_module("tyme", LevelFilter::Trace)
        .parse_default_env()
        .default_format()
        .format_indent(Some(4))
        .format_level(true)
        .format_module_path(true)
        .format_target(false)
        .format_timestamp_millis()
        .init();
}

#[allow(dead_code)]
pub fn get_discord_token() -> Result<String> {
    env::var("DISCORD_TOKEN").context("Missing `DISCORD_TOKEN` env var")
}

#[allow(dead_code)]
pub fn get_database_url() -> Result<String> {
    env::var("DATABASE_URL").context("Missing `DATABASE_URL` env var")
}
