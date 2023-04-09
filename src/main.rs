#![warn(
    explicit_outlives_requirements,
    elided_lifetimes_in_paths,
    let_underscore_drop,
    missing_debug_implementations,
    noop_method_call,
    unsafe_code,
    unused_qualifications,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

mod data;
mod events;
mod handler;
mod interactions;
mod macros;
mod messages;
mod utils;

use color_eyre::eyre::{Result, WrapErr};
use dotenvy::dotenv;
use serenity::{client::Client, model::gateway::GatewayIntents};

// use tokio_postgres::NoTls;
use crate::{
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

    let _db_url = get_database_url().context("Unable to get database url")?;

    // let config = rustls::ClientConfig::builder()
    //     .with_safe_defaults()
    //     .with_no_client_auth();
    // let tls = tokio_postgres_rustls::MakeRustlsConnect::new(config);
    //
    // let (_db_client, connection) = tokio_postgres::connect(&db_url, tls).await?;
    //
    // tokio::spawn(async move {
    //     if let Err(e) = connection.await {
    //         log::error!("connection error: {}", e);
    //     }
    // });

    let token = get_discord_token().context("Unable to get bot token")?;

    let mut client = Client::builder(
        token,
        GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT,
    )
    .event_handler(Handler)
    .await
    .context("Error creating client")?;

    {
        let _data = client.data.write().await;

        // data.insert::<Database>(Mutex::new(db_client));
    }

    client.start().await?;

    Ok(())
}
