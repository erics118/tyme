use anyhow::Result;
use serenity::{client::Context, model::channel::Message};

pub async fn run(ctx: Context, _message: Message) -> Result<()> {
    log::error!("shutting down because shutdown message command was invoked");
    ctx.shard.shutdown_clean();
    Ok(())
}
