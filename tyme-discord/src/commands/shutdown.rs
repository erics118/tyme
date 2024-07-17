use std::process::exit;

use crate::types::*;

/// Shuts down the bot.
#[poise::command(prefix_command, owners_only)]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("Shutting down.").await?;

    log::error!("Shutting down because shutdown message command was invoked");
    ctx.serenity_context().shard.shutdown_clean();
    exit(0);
}
