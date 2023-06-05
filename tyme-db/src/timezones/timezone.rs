use anyhow::{Context, Result};
use chrono_tz::Tz;
use serenity::model::id::UserId;
use sqlx::MySqlPool;

#[derive(Debug, Clone)]
pub struct Timezone {
    pub user_id: UserId,
    pub timezone: Tz,
}

impl Timezone {
    pub async fn get(db: &MySqlPool, user_id: UserId) -> Result<Self> {
        let row = sqlx::query!(
            r#"
            SELECT timezone
            FROM timezones
            WHERE user_id = ?;
            "#,
            i64::from(user_id),
        )
        .fetch_optional(db)
        .await?
        .context("does not exist")?;

        let timezone = Tz::from_str_insensitive(&row.timezone)
            .map_err(|_| anyhow::anyhow!("database corrupted, timezone invalid"))?;

        Ok(Self { user_id, timezone })
    }

    pub async fn set(&self, db: &MySqlPool) -> Result<()> {
        // either update row or create new row
        sqlx::query!(
            r#"
            INSERT INTO timezones (user_id, timezone)
            VALUES (?, ?)
            ON DUPLICATE KEY UPDATE timezone = VALUES(timezone);
            "#,
            i64::from(self.user_id),
            self.timezone.name()
        )
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn delete(db: &MySqlPool, user_id: UserId) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM timezones
            WHERE user_id = ?;
            "#,
            i64::from(user_id),
        )
        .execute(db)
        .await?;
        Ok(())
    }
}