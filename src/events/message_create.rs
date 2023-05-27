use anyhow::{Context as _, Result};
use serenity::{
    client::Context,
    model::{mention::Mentionable, prelude::Message},
};

use crate::messages::commands::exec;

pub async fn run(ctx: Context, message: Message) -> Result<()> {
    if message.is_own(&ctx.cache) {
        return Ok(());
    }

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

        if let Some(command) = content.split(' ').next() {
            let command = command.to_string();

            log::trace!("Received message command: {command}");

            exec(command, ctx, message).await?;
        }
    }

    Ok(())
}
