use anyhow::Result;
use serenity::{builder::CreateMessage, client::Context, model::channel::Message};

use crate::interactions::commands::register_all;

/// Register all application commands.
pub async fn run(ctx: Context, message: Message) -> Result<()> {
    register_all(&ctx.http).await?;

    message
        .channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new()
                .content("Registered all application commands.")
                .reference_message(&message),
        )
        .await?;

    Ok(())
}
