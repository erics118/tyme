use anyhow::Result;
use chrono::NaiveDateTime;
use serenity::model::id::{ChannelId, GuildId, UserId};
use sqlx::types::Uuid;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Reminder {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub time: NaiveDateTime,
    pub message: String,
    pub user_id: UserId,
    pub channel_id: ChannelId,
    pub guild_id: Option<GuildId>,
}

impl Reminder {
    pub async fn create(&self, pool: &Mutex<sqlx::PgPool>) -> Result<Uuid> {
        let pool = pool.lock().await;

        let rec = sqlx::query!(
            r#"
            INSERT INTO reminders (id, created_at, time, message, user_id, channel_id, guild_id)
            VALUES (gen_random_uuid(), $1::TIMESTAMP, $2::TIMESTAMP, $3::TEXT, $4::BIGINT, $5::BIGINT, $6::BIGINT)
            RETURNING id;
            "#,
            self.created_at,
            self.time,
            self.message,
            i64::from(self.user_id),
            i64::from(self.channel_id),
            self.guild_id.map(|a| i64::from(a)),
        )
        .fetch_one(&*pool)
        .await?;

        Ok(rec.id)
    }
}
