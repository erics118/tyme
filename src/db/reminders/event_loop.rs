use std::time::Duration;

use anyhow::Result;
use chrono::Utc;
use serenity::{
    builder::CreateMessage,
    http::CacheHttp,
    model::id::{ChannelId, GuildId, UserId},
};
use tokio::{sync::Mutex, time::sleep};

use super::reminder::Reminder;

pub async fn event_reminder_loop(pool: Mutex<sqlx::PgPool>, http: impl CacheHttp) {
    loop {
        // Retrieve events from the database
        #[allow(clippy::unwrap_used)]
        let reminders = fetch_past_reminders(&pool).await.unwrap();

        let current_time = Utc::now();

        log::trace!("{current_time}");

        for r in reminders {
            log::info!("Reminder: {} | {}!", r.message, r.id);

            let message = format!("Reminder: ```rs\n{:#?}\n```", r);

            #[allow(clippy::unwrap_used)]
            r.channel_id
                .send_message(&http, CreateMessage::new().content(message))
                .await
                .unwrap();
        }

        sleep(Duration::from_secs(60)).await;
    }
}

pub async fn fetch_past_reminders(pool: &Mutex<sqlx::PgPool>) -> Result<Vec<Reminder>> {
    let pool = pool.lock().await;

    let query = sqlx::query!(
        r#"
DELETE FROM reminders
WHERE "time" <= CURRENT_TIMESTAMP
RETURNING *
        "#
    );

    let mut reminders = Vec::new();

    let rows = query.fetch_all(&*pool).await?;

    for row in rows {
        reminders.push(Reminder {
            id: row.id,
            created_at: row.created_at,
            time: row.time,
            message: row.message,
            creator_id: UserId::from(row.creator_id as u64),
            channel_id: ChannelId::from(row.channel_id as u64),
            guild_id: row.guild_id.map(|a| GuildId::from(a as u64)),
        });
    }

    Ok(reminders)
}