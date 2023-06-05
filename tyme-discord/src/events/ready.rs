use std::time::Duration;

use anyhow::{Context as _, Result};
use chrono::Utc;
use serenity::{
    builder::CreateMessage,
    client::Context,
    gateway::ActivityData,
    http::CacheHttp,
    model::{
        gateway::{ActivityType, Ready},
        mention::Mentionable,
    },
};
use tokio::{sync::Mutex, time::sleep};
use tyme_db::{MySqlPool, Reminder};

use crate::{
    data::database::Database,
    utils::timestamp::{DiscordTimestamp, TimestampFormat},
};

pub async fn notify_past_reminders(pool: &Mutex<MySqlPool>, http: impl CacheHttp) -> Result<()> {
    // Retrieve events from the database
    let reminders = Reminder::get_all_past_reminders(pool).await?;

    let current_time = Utc::now();

    log::trace!("{current_time}");

    for r in reminders {
        log::info!("{r:#?}");

        let message = format!(
            "Reminder for {}: {}\nSet {}",
            r.user_id.mention(),
            r.message,
            r.created_at.discord_timestamp(TimestampFormat::Relative),
        );

        r.channel_id
            .send_message(&http, CreateMessage::new().content(message))
            .await?;
    }

    Ok(())
}

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

    let pool2 = Mutex::new(pool.clone());

    tokio::spawn(async move {
        loop {
            #[allow(clippy::unwrap_used)]
            notify_past_reminders(&pool2, &ctx.http).await.unwrap();

            sleep(Duration::from_secs(60)).await;
        }
    });

    Ok(())
}
