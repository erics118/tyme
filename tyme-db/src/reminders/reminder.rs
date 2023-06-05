use anyhow::Result;
use chrono::NaiveDateTime;
use serenity::model::id::{ChannelId, GuildId, UserId};
use sqlx::MySqlPool;

#[derive(Debug, Clone)]
pub struct Reminder {
    pub id: Option<u64>,
    pub created_at: NaiveDateTime,
    pub time: NaiveDateTime,
    pub message: String,
    pub user_id: UserId,
    pub channel_id: ChannelId,
    pub guild_id: Option<GuildId>,
}

impl Reminder {
    pub async fn create(&self, db: &MySqlPool) -> Result<u64> {
        let id = sqlx::query!(
            r#"
            INSERT INTO reminders (created_at, time, message, user_id, channel_id, guild_id)
            VALUES (?, ?, ?, ?, ?, ?);
            "#,
            self.created_at,
            self.time,
            self.message,
            i64::from(self.user_id),
            i64::from(self.channel_id),
            self.guild_id.map(i64::from),
        )
        .execute(db)
        .await?
        .last_insert_id();

        Ok(id)
    }

    pub async fn get_all_past_reminders(db: &MySqlPool) -> Result<Vec<Self>> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM reminders
            WHERE time <= NOW();
            "#
        )
        .fetch_all(db)
        .await?;

        // TODO: breaks because NOW() is different
        sqlx::query!(
            r#"
            DELETE FROM reminders
            WHERE time <= NOW();
            "#
        )
        .execute(db)
        .await?;

        let mut reminders = Vec::new();

        for row in rows {
            reminders.push(Self {
                id: Some(row.id),
                created_at: row.created_at,
                time: row.time,
                message: row.message,
                user_id: UserId::from(row.user_id),
                channel_id: ChannelId::from(row.channel_id),
                guild_id: row.guild_id.map(GuildId::from),
            });
        }

        Ok(reminders)
    }

    pub async fn get_one_by_id(db: &MySqlPool, id: u64) -> Result<Self> {
        let row = sqlx::query!(
            r#"
            SELECT *
            FROM reminders
            WHERE id = ?;
            "#,
            id,
        )
        .fetch_one(db)
        .await?;

        Ok(Self {
            id: Some(row.id),
            created_at: row.created_at,
            time: row.time,
            message: row.message,
            user_id: UserId::from(row.user_id),
            channel_id: ChannelId::from(row.channel_id),
            guild_id: row.guild_id.map(GuildId::from),
        })
    }

    // get_one_by_user_id // gets latest

    pub async fn get_all_by_user_id(db: &MySqlPool, user_id: UserId) -> Result<Vec<Self>> {
        let rows = sqlx::query!(
            r#"
            SELECT *
            FROM reminders
            WHERE user_id = ?
            ORDER BY time;
            "#,
            i64::from(user_id),
        )
        .fetch_all(db)
        .await?;

        let mut reminders: Vec<Self> = Vec::new();

        for row in rows {
            reminders.push(Self {
                id: Some(row.id),
                created_at: row.created_at,
                time: row.time,
                message: row.message,
                user_id: UserId::from(row.user_id),
                channel_id: ChannelId::from(row.channel_id),
                guild_id: row.guild_id.map(GuildId::from),
            });
        }

        Ok(reminders)
    }

    pub async fn delete_one_by_id(db: &MySqlPool, id: u32) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM reminders
            WHERE id = ?;
            "#,
            id,
        )
        .fetch_one(db)
        .await?;

        Ok(())
    }

    pub async fn delete_all_by_user_id(db: &MySqlPool, user_id: UserId) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM reminders
            WHERE user_id = ?;
            "#,
            i64::from(user_id),
        )
        .execute(db)
        .await?;

        Ok(())
    }
}