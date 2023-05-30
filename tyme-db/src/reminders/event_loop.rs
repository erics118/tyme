use std::time::Duration;

use chrono::Utc;
use serenity::{builder::CreateMessage, http::CacheHttp, prelude::Mentionable};
use tokio::{sync::Mutex, time::sleep};
use tyme_utils::timestamp::{DiscordTimestamp, TimestampFormat};

use super::reminder::Reminder;

pub async fn event_reminder_loop(pool: Mutex<sqlx::PgPool>, http: impl CacheHttp) {
    loop {
        // Retrieve events from the database
        #[allow(clippy::unwrap_used)]
        let reminders = Reminder::fetch_past_reminders(&pool).await.unwrap();

        let current_time = Utc::now();

        log::trace!("{current_time}");

        for r in reminders {
            log::info!("{:#?}!", r);

            let message = format!(
                "Reminder for {}: {}\nSet {}",
                r.user_id.mention(),
                r.message,
                r.created_at.discord_timestamp(TimestampFormat::Relative),
            );

            #[allow(clippy::unwrap_used)]
            r.channel_id
                .send_message(&http, CreateMessage::new().content(message))
                .await
                .unwrap();
        }

        sleep(Duration::from_secs(60)).await;
    }
}
