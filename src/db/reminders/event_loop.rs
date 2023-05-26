use std::time::Duration;

use anyhow::Result;
use chrono::{Local, TimeZone};
use serenity::model::id::{ChannelId, GuildId, UserId};
use tokio::{sync::Mutex, time::sleep};

use super::reminder::Reminder;

pub async fn event_reminder_loop(pool: Mutex<sqlx::PgPool>) {
    loop {
        println!("checking");
        // Retrieve events from the database
        let events = retrieve_events(&pool).await.unwrap();

        let current_time = Local::now();
        println!("cur: {current_time}");
        for event in events {
            println!("Event reminder: It's time for {}!", event.message);
        }

        sleep(Duration::from_secs(60)).await;
    }
}

pub async fn retrieve_events(pool: &Mutex<sqlx::PgPool>) -> Result<Vec<Reminder>> {
    let pool = pool.lock().await;

    let query = sqlx::query!(
        r#"
SELECT *
FROM reminders
WHERE "time" <= CURRENT_TIMESTAMP;
        "#
    );

    let mut events = Vec::new();

    let rows = query.fetch_all(&*pool).await?;

    for row in rows {
        events.push(Reminder {
            id: row.id,
            created_at: row.created_at,
            time: row.time,
            message: row.message,
            creator_id: UserId::from(row.creator_id as u64),
            thread_id: row.thread_id.map(|a| ChannelId::from(a as u64)),
            channel_id: ChannelId::from(row.channel_id as u64),
            guild_id: row.guild_id.map(|a| GuildId::from(a as u64)),
        });
    }

    let query = sqlx::query!(
        r#"
SELECT *
FROM reminders
        "#
    );

    let rows = query.fetch_all(&*pool).await?;

    for row in rows {
        let message = row.message;
        let naive = row.time;
        let time = TimeZone::from_utc_datetime(&Local, &naive);
        println!("{message:?} - {time:?}");
    }

    Ok(events)
}
