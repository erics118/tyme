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
mod events;
mod handler;
mod interactions;
mod macros;
mod messages;
mod utils;

use anyhow::{Context, Result};
use dotenvy::dotenv;
use serenity::{client::Client, model::gateway::GatewayIntents};

use crate::{
    handler::Handler,
    utils::setup::{get_discord_token, setup_logger},
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

    let token = get_discord_token().context("Unable to get bot token")?;

    let mut client = Client::builder(
        token,
        GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT,
    )
    .event_handler(Handler)
    .await
    .context("Error creating client")?;

    client.start().await?;

    Ok(())
}
