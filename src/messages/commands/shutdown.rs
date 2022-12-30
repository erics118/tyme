use std::process::exit;

use anyhow::Result;
use serenity::{client::Context, model::channel::Message};

pub static NAME: &str = "test";

pub async fn run(ctx: Context, _message: Message) -> Result<()> {
    log::error!("Shutting down because shutdown message command was invoked");
    ctx.shard.shutdown_clean();
    exit(0);
}
