use anyhow::{Context as _, Result};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::Mutex;

use crate::setup::{get_database_url, get_discord_token, setup_logger};

pub mod setup;

#[tokio::main]
async fn main() -> Result<()> {
    let dotenv_state = dotenv().is_ok();

    setup_logger();

    if dotenv_state {
        log::info!("Using .env file");
    } else {
        log::info!("Not using .env file");
    }

    // start database
    let database_url = get_database_url().context("Unable to get database URL")?;

    log::info!("Connecting to database");

    let pool = PgPoolOptions::new().connect(&database_url).await?;

    log::info!("Database connection successful");

    // start discord bot
    let token = get_discord_token().context("Unable to get bot token")?;

    tokio::spawn(async move {
        let _ = tyme_discord::start(token, Mutex::new(pool.clone())).await;
    });

    loop {}
}
