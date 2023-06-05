#![forbid(unsafe_code)]
#![warn(
    explicit_outlives_requirements,
    elided_lifetimes_in_paths,
    unused_qualifications,
    clippy::all,
    clippy::expect_used,
    clippy::unwrap_used
)]

pub mod data;
pub mod events;
pub mod handler;
pub mod interactions;
pub mod macros;
pub mod messages;
pub mod setup;
pub mod utils;

use anyhow::{Context as _, Result};
use dotenvy::dotenv;
use serenity::{client::Client, model::gateway::GatewayIntents};
use tyme_db::MySqlPoolOptions;

use crate::{
    data::database::Database,
    handler::Handler,
    setup::{get_database_url, get_discord_token, setup_logger},
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

    let pool = MySqlPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await?;

    // pool.connect_options().pipes_as_concat(false);

    log::info!("Database connection successful");

    // start discord bot
    let token = get_discord_token().context("Unable to get bot token")?;

    let mut client = Client::builder(
        token,
        GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT,
    )
    .event_handler(Handler)
    .type_map_insert::<Database>(pool)
    .await
    .context("Error creating client")?;

    client.start().await?;

    Ok(())
}