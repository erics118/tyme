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
mod events;
mod handler;
mod interactions;
mod macros;
mod messages;

use anyhow::{Context as _, Result};
use serenity::{client::Client, model::gateway::GatewayIntents};
use sqlx::PgPool;
use tokio::sync::Mutex;

use crate::{data::database::Database, handler::Handler};

pub async fn start(token: String, pool: Mutex<PgPool>) -> Result<()> {
    let mut client = Client::builder(
        token,
        GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT,
    )
    .event_handler(Handler)
    .await
    .context("Error creating client")?;

    {
        let mut data = client.data.write().await;

        data.insert::<Database>(pool);
    }

    client.start().await?;

    Ok(())
}
