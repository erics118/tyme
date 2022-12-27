mod data;
mod events;
mod handler;
mod interactions;
mod messages;

use std::env;

use anyhow::{Context as AnyhowContext, Result};
use serenity::{client::Client, model::gateway::GatewayIntents};

use crate::handler::Handler;

#[tokio::main]
async fn main() -> Result<()> {
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
