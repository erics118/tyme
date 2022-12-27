use anyhow::Result;
use serenity::{client::Context, model::channel::Message};

pub async fn run(ctx: Context, _message: Message) -> Result<()> {
    ctx.shard.shutdown_clean();
    Ok(())
}
