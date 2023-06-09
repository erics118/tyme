//! # Tyme Discord Bot
//! This is the main file for the Tyme Discord bot. It is responsible for
//! starting the bot and connecting to the database.

#![forbid(unsafe_code)]
#![warn(
    absolute_paths_not_starting_with_crate,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    ffi_unwind_calls,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    non_ascii_idents,
    noop_method_call,
    pointer_structural_match,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    unused_tuple_struct_fields,
    variant_size_differences,
    explicit_outlives_requirements,
    elided_lifetimes_in_paths,
    unused_qualifications,
    clippy::all,
    clippy::nursery,
    clippy::expect_used,
    clippy::unwrap_used
)]

pub mod data;
pub mod events;
pub mod handler;
pub mod interactions;
pub mod macros;
pub mod messages;
pub mod setup;
pub mod utils;

use anyhow::{Context as _, Result};
use dotenvy::dotenv;
use serenity::{client::Client, model::gateway::GatewayIntents};
use tyme_db::MysqlConnection;

use crate::{
    data::database::Database,
    handler::Handler,
    setup::{get_database_url, get_discord_token, setup_logger},
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

    MysqlConnection::establish(&database_url).context("Unable to connect to database")?;

    log::info!("Database connection successful");

    // start discord bot
    let token = get_discord_token().context("Unable to get bot token")?;

    let mut client = Client::builder(
        token,
        GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT,
    )
    .event_handler(Handler)
    .type_map_insert::<Database>(db)
    .await
    .context("Error creating client")?;

    client.start().await?;

    Ok(())
}
