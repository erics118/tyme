#![forbid(unsafe_code)]
#![warn(
    explicit_outlives_requirements,
    elided_lifetimes_in_paths,
    let_underscore_drop,
    missing_debug_implementations,
    noop_method_call,
    unused_qualifications,
    clippy::all,
    clippy::nursery
)]

mod data;
mod db;
mod events;
mod handler;
mod interactions;
mod macros;
mod messages;
mod utils;

use anyhow::{Context, Result};
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

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // test query
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);

    println!("{}", row.0);

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
