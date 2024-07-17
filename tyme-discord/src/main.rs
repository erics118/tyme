//! # Tyme Discord Bot
//! This is the main file for the Tyme Discord bot. It is responsible for
//! starting the bot and connecting to the database.

#![forbid(unsafe_code)]
#![warn(
    absolute_paths_not_starting_with_crate,
    unused_qualifications,
    dead_code,
    clippy::all,
    clippy::expect_used,
    clippy::unwrap_used
)]

pub mod setup;
pub mod utils;

use std::time::Duration;

use anyhow::{Context as _, Result};
use dotenvy::dotenv;
use event_loop::notify_past_reminders;
use poise::{
    serenity_prelude::{ActivityData, ActivityType, Client, GatewayIntents},
    FrameworkOptions, PrefixFrameworkOptions,
};
use tokio::{sync::Mutex, time::interval};
use tyme_db::PoolOptions;
use types::*;

use crate::setup::{get_database_url, get_discord_token, setup_logger};

pub mod types;

pub mod commands;
pub mod event_loop;
pub mod macros;

#[tokio::main]
async fn main() -> Result<()> {
    let dotenv_state = dotenv().is_ok();

    setup_logger();

    if dotenv_state {
        log::info!("Using .env file");
    } else {
        log::info!("Not using .env file");
    }

    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options = FrameworkOptions {
        commands: commands::all(),
        prefix_options: PrefixFrameworkOptions {
            mention_as_prefix: true,
            ..Default::default()
        },
        skip_checks_for_owners: true,
        pre_command: move |ctx| {
            Box::pin(async move {
                log::trace!(
                    "Received interaction command: {}",
                    ctx.command().qualified_name
                );
            })
        },
        ..Default::default()
    };

    // start database
    let database_url = get_database_url().context("Unable to get database URL")?;

    log::info!("Connecting to database");

    let db = PoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await?;

    log::info!("Database connection successful");

    let framework = poise::Framework::builder()
        .setup(move |ctx, ready, _framework| {
            Box::pin(async move {
                log::info!("Bot connected as: {}", ready.user.name);

                ctx.set_activity(Some(ActivityData {
                    name: "eirk".to_string(),
                    kind: ActivityType::Listening,
                    state: None,
                    url: None,
                }));

                log::trace!("Set status");

                let http = ctx.http.clone();
                let db2 = db.clone();

                // Start a task to notify users of past reminders every minute
                tokio::spawn(async move {
                    let mut interval = interval(Duration::from_secs(60));

                    loop {
                        interval.tick().await;

                        if let Err(e) = notify_past_reminders(&db2, &http).await {
                            log::error!("Failed to notify past reminders: {:#?}", e);
                        }
                    }
                });

                Ok(Data { db: Mutex::new(db) })
            })
        })
        .options(options)
        .build();

    let token = get_discord_token().context("Unable to get bot token")?;

    let mut client = Client::builder(token, GatewayIntents::non_privileged())
        .framework(framework)
        .await?;

    client.start().await?;

    Ok(())
}
