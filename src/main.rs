mod events;
mod handler;
mod interactions;

use std::{collections::HashMap, env};

use anyhow::{Context as AnyhowContext, Result};
use serenity::{model::prelude::*, prelude::*};

use crate::{handler::Handler, interactions::commands::InteractionCommands};

#[tokio::main]
async fn main() -> Result<()> {
    let token = env::var("DISCORD_TOKEN").context("Missing `DISCORD_TOKEN` env var")?;

    let mut client = Client::builder(token, GatewayIntents::GUILD_MESSAGES)
        .event_handler(Handler)
        .await
        .context("Error creating client")?;
    {
        let mut data = client.data.write().await;
        data.insert::<InteractionCommands>(HashMap::default());
    }
    client.start().await?;
    Ok(())
}
