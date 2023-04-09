use color_eyre::eyre::{ContextCompat, Result};
use serenity::{client::Context, model::channel::Message};

use crate::data::database::Database;

pub async fn _run(ctx: Context, _message: Message) -> Result<()> {
    {
        let data = ctx.data.read().await;

        let db = data
            .get::<Database>()
            .context("Expected `Database` in TypeMap")?;

        let adb = db.lock().await;
        let bdb = adb.write().await;

        let _rows = bdb.query("SELECT $1::TEXT", &[&"hello world"]).await?;
    }
    Ok(())
}
