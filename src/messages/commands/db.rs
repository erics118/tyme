use anyhow::{Context as AnyhowContext, Result};
use serenity::{client::Context, model::channel::Message};

// use crate::data::database::Database;

pub static NAME: &str = "db";

// pub async fn run(ctx: Context, _message: Message) -> Result<()> {
//     let data = ctx.data.read().await;
//     {
//         let db = data
//             .get::<Database>()
//             .context("Expected `Database` in TypeMap")?;
//         let rows = db.query("SELECT $1::TEXT", &[&"hello world"]).await?;
//     }
//     Ok(())
// }
