mod data;
mod events;
mod handler;
mod interactions;
mod messages;
mod utils;

use anyhow::{Context as AnyhowContext, Result};
use serenity::{client::Client, model::gateway::GatewayIntents};
use utils::setup::{get_token, setup_logger};

use crate::handler::Handler;

#[tokio::main]
async fn main() -> Result<()> {
    setup_logger();

    let token = get_token().context("Unable to get bot token")?;

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
