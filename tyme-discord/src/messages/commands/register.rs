use anyhow::Result;
use serenity::{client::Context, model::channel::Message};

use crate::{create_message, interactions::commands::register_all};

/// Register all application commands.
pub async fn run(ctx: Context, message: Message) -> Result<()> {
    register_all(&ctx.http).await?;

    message
        .channel_id
        .send_message(
            &ctx.http,
            create_message!(
                    "Registered all application commands.",
                    @ message
            ),
        )
        .await?;

    Ok(())
}
