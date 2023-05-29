#![forbid(unsafe_code)]
#![warn(
    explicit_outlives_requirements,
    elided_lifetimes_in_paths,
    missing_debug_implementations,
    noop_method_call,
    unused_qualifications,
    clippy::all,
    clippy::nursery,
    clippy::expect_used,
    clippy::unwrap_used
)]
#![allow(clippy::significant_drop_tightening)]

mod data;
mod db;
mod events;
mod handler;
mod interactions;
mod macros;
mod messages;
mod utils;

use anyhow::{Context as _, Result};
use dotenvy::dotenv;
use serenity::{client::Client, model::gateway::GatewayIntents};
use sqlx::postgres::PgPoolOptions;
use tokio::sync::Mutex;

use crate::{
    data::db::Database,
    handler::Handler,
    utils::setup::{get_database_url, get_discord_token, setup_logger},
};

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

    let mut client = Client::builder(
        token,
        GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT,
    )
    .event_handler(Handler)
    .await
    .context("Error creating client")?;

    {
        let mut data = client.data.write().await;

        data.insert::<Database>(Mutex::new(pool));
    }

    client.start().await?;

    Ok(())
}
