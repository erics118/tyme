use anyhow::Result;
use serenity::{client::Context, model::channel::Message};

use crate::interactions::commands::register_all;

pub async fn run(ctx: Context, _message: Message) -> Result<()> {
    register_all(ctx).await
}
