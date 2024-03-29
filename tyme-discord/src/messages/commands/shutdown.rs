use std::process::exit;

use anyhow::Result;
use serenity::{client::Context, model::channel::Message};

/// Shut down the bot.
pub async fn run(ctx: Context, _message: Message) -> Result<()> {
    log::error!("Shutting down because shutdown message command was invoked");
    ctx.shard.shutdown_clean();
    exit(0);
}
