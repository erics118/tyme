use anyhow::{Context as _, Result};
use serenity::{
    client::Context,
    gateway::ActivityData,
    model::gateway::{ActivityType, Ready},
};
use tokio::sync::Mutex;
use tyme_db::reminders::event_loop::event_reminder_loop;

use crate::data::db::Database;

pub async fn run(ctx: Context, ready: Ready) -> Result<()> {
    log::info!("Bot connected as: {}", ready.user.name);

    ctx.set_activity(Some(ActivityData {
        name: "eirk".to_string(),
        kind: ActivityType::Listening,
        url: None,
    }));
    log::trace!("Set status");

    let data = ctx.data.read().await;

    let db = data
        .get::<Database>()
        .context("Expected `Database` in TypeMap")?;

    let pool = db.lock().await;

    let pool2 = pool.clone();

    tokio::spawn(async move {
        event_reminder_loop(Mutex::new(pool2), &ctx.http).await;
    });

    Ok(())
}
