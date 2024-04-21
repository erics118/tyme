//! Message create event handler.

use anyhow::{Context as _, Result};
use serenity::{
    client::Context,
    model::{mention::Mentionable, prelude::Message},
};

use crate::messages::commands;

/// Handle an message create event.
pub async fn run(ctx: Context, message: Message) -> Result<()> {
    let mention = ctx.cache.current_user().mention().to_string();

    let owner_id = ctx
        .http
        .get_current_application_info()
        .await
        .context("Couldn't get application info")?
        .owner
        .context("No owner")?
        .id;

    if message.author.id == owner_id && message.content.starts_with(&mention) {
        log::trace!("Message command invoked");

        let content = message.content.trim().to_string();

        if let Some(command) = content.split_whitespace().nth(1) {
            log::trace!("Received message command: {command}");

            commands::exec(command, ctx, message).await?;
        }
    }

    Ok(())
}
