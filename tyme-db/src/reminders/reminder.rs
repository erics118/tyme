use anyhow::Result;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serenity::model::id::{ChannelId, GuildId, UserId};
use sqlx::types::Uuid;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

        let record = sqlx::query!(
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

        Ok(record.id)
    }

    pub async fn fetch_past_reminders(pool: &Mutex<sqlx::PgPool>) -> Result<Vec<Self>> {
        let pool = pool.lock().await;

        let rows = sqlx::query!(
            r#"
            DELETE FROM reminders
            WHERE time <= CURRENT_TIMESTAMP
            RETURNING *;
            "#
        )
        .fetch_all(&*pool)
        .await?;

        let mut reminders = Vec::new();

        for row in rows {
            reminders.push(Self {
                id: row.id,
                created_at: row.created_at,
                time: row.time,
                message: row.message,
                user_id: UserId::from(row.user_id as u64),
                channel_id: ChannelId::from(row.channel_id as u64),
                guild_id: row.guild_id.map(|a| GuildId::from(a as u64)),
            });
        }

        Ok(reminders)
    }

    pub async fn get_by_user_id(pool: &Mutex<sqlx::PgPool>, user_id: UserId) -> Result<Vec<Self>> {
        let pool = pool.lock().await;

        let rows = sqlx::query!(
            r#"
            SELECT *
            FROM reminders
            WHERE user_id = $1;
            "#,
            i64::from(user_id),
        )
        .fetch_all(&*pool)
        .await?;

        let mut reminders: Vec<Self> = Vec::new();

        for row in rows {
            reminders.push(Self {
                id: row.id,
                created_at: row.created_at,
                time: row.time,
                message: row.message,
                user_id: UserId::from(row.user_id as u64),
                channel_id: ChannelId::from(row.channel_id as u64),
                guild_id: row.guild_id.map(|a| GuildId::from(a as u64)),
            });
        }

        Ok(reminders)
    }
}
