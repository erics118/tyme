mod data;
mod events;
mod handler;
mod interactions;
mod messages;
mod utils;

use std::env;

use anyhow::{Context as AnyhowContext, Result};
use log::LevelFilter;
use serenity::{client::Client, model::gateway::GatewayIntents};

use crate::handler::Handler;

#[tokio::main]
async fn main() -> Result<()> {
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

    let token = env::var("DISCORD_TOKEN").context("Missing `DISCORD_TOKEN` env var")?;

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
