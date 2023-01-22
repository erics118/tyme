mod data;
mod events;
mod handler;
mod interactions;
mod messages;
mod utils;

use std::collections::HashMap;

use color_eyre::eyre::{Result, WrapErr};
use data::{interaction_commands::InteractionCommands, message_commands::MessageCommands};
use dotenvy::dotenv;
use serenity::{client::Client, model::gateway::GatewayIntents};
use tokio_postgres::NoTls;
use utils::setup::{get_database_url, get_discord_token, setup_logger};

use crate::handler::Handler;

#[tokio::main]
async fn main() -> Result<()> {
    let dotenv_state = dotenv().is_ok();

    setup_logger();

    if dotenv_state {
        log::info!("Using .env file");
    } else {
        log::info!("Not using .env file");
    }

    let db_url = get_database_url().context("Unable to get database url")?;

    let (db_client, connection) = tokio_postgres::connect(&db_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            log::error!("connection error: {}", e);
        }
    });

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

        data.insert::<InteractionCommands>(HashMap::default());
        data.insert::<MessageCommands>(HashMap::default());
    }

    client.start().await?;

    Ok(())
}
