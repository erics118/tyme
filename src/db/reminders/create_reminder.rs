use anyhow::Result;
use sqlx::types::Uuid;

use super::reminder::Reminder;

pub async fn create_reminder(pool: &sqlx::PgPool, r: Reminder) -> Result<Uuid> {
    let rec = sqlx::query!(
        r#"
INSERT INTO reminders (id, created_at, time, message, creator_id, thread_id, channel_id, guild_id)
VALUES (gen_random_uuid(), $1::TIMESTAMP, $2::TIMESTAMP, $3::TEXT, $4::BIGINT, $5::BIGINT, $6::BIGINT, $7::BIGINT)
RETURNING id;
        "#,
        r.created_at,
        r.time,
        r.message,
        i64::from(r.creator_id),
        r.thread_id.map(|a| i64::from(a)),
        i64::from(r.channel_id),
        r.guild_id.map(|a| i64::from(a)),
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}
