//! Ready event handler.

use anyhow::{Context as _, Result};
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
use tokio::time::{interval, Duration};
use tyme_db::{MySqlPool, Reminder};

use crate::{
    data::database::Database,
    utils::timestamp::{DiscordTimestamp, TimestampFormat},
};

/// Notify users of past reminders.
pub async fn notify_past_reminders(db: &MySqlPool, http: impl CacheHttp) -> Result<()> {
    let reminders = Reminder::get_all_past_reminders(db).await?;

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

/// Handle the ready event.
pub async fn run(ctx: Context, ready: Ready) -> Result<()> {
    log::info!("Bot connected as: {}", ready.user.name);

    ctx.set_activity(Some(ActivityData {
        name: "eirk".to_string(),
        kind: ActivityType::Listening,
        url: None,
    }));

    log::trace!("Set status");

    let db = {
        let data = ctx.data.read().await;
        data.get::<Database>()
            .context("Expected `Database` in TypeMap")?
            .clone()
    };

    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(60));

        loop {
            interval.tick().await;

            notify_past_reminders(&db, &ctx.http)
                .await
                .unwrap_or_else(|e| log::error!("Failed to notify past reminders: {e:#?}"));
        }
    });

    Ok(())
}
