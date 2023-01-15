use std::process::exit;

use anyhow::Result;
use serenity::{client::Context, model::channel::Message};
use tokio_postgres::Client as DbClient;

pub static NAME: &str = "shutdown";

pub async fn run(ctx: Context, _message: Message, _db: DbClient) -> Result<()> {
    log::error!("Shutting down because shutdown message command was invoked");
    ctx.shard.shutdown_clean();
    exit(0);
}
