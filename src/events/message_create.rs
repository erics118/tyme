use anyhow::{Context as AnyhowContext, Result};
use serenity::{
    client::Context,
    model::{mention::Mentionable, prelude::Message},
};

use crate::messages::commands::exec;

pub async fn run(ctx: Context, message: Message) -> Result<()> {
    if message.is_own(&ctx.cache) {
        return Ok(());
    }

    let mention = ctx.cache.current_user_id().mention().to_string();

    let owner_id = ctx
        .http
        .get_current_application_info()
        .await
        .context("Couldn't get application info")?
        .owner
        .id;

    if message.author.id == owner_id && message.content.starts_with(&mention) {
        log::trace!("Message command invoked");

        let content = message
            .content
            .get(mention.len()..)
            .unwrap()
            .trim()
            .to_string();

        let command = content.split(' ').next().unwrap().to_string();

        log::trace!("Received message command: {command}");

        exec(command, ctx, message).await?;
    }

    Ok(())
}
