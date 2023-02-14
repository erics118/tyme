use std::process::exit;

use color_eyre::eyre::Result;
use serenity::{client::Context, model::channel::Message};

pub static NAME: &str = "shutdown";

pub async fn run(ctx: Context, _message: Message) -> Result<()> {
    log::error!("Shutting down because shutdown message command was invoked");
    ctx.shard.shutdown_clean();
    exit(0);
}
