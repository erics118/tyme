//! Timezone database model.

use anyhow::{Context, Result};
use chrono_tz::Tz;
use serenity::model::id::UserId;
use sqlx::MySqlPool;

/// Timezone struct.
#[derive(Debug, Clone, Copy)]
pub struct Timezone {
    /// The user's id.
    pub user_id: UserId,

    /// The user's timezone.
    pub timezone: Tz,
}

impl Timezone {
    /// Get a user's timezone, given their user id.
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

    /// Get a user's timezone, given their user id.
    pub async fn set(&self, db: &MySqlPool) -> Result<()> {
        // either update row or create new row
        sqlx::query!(
            r#"
            INSERT INTO timezones (user_id, timezone)
            VALUES (?, ?)
            ON DUPLICATE KEY UPDATE timezone = VALUES(timezone);
            "#,
            i64::from(self.user_id),
            self.timezone.name(),
        )
        .execute(db)
        .await?;

        Ok(())
    }

    /// Delete a user's timezone, given their user id.
    pub async fn delete(db: &MySqlPool, user_id: UserId) -> Result<Self> {
        let row = sqlx::query!(
            r#"
            SELECT * FROM timezones
            WHERE user_id = ?;
            "#,
            i64::from(user_id),
        )
        .fetch_one(db)
        .await?;

        sqlx::query!(
            r#"
            DELETE FROM timezones
            WHERE user_id = ?;
            "#,
            i64::from(user_id),
        )
        .execute(db)
        .await?;

        let timezone = Tz::from_str_insensitive(&row.timezone)
            .map_err(|_| anyhow::anyhow!("database corrupted, timezone invalid"))?;
        Ok(Self {
            user_id: row.user_id.into(),
            timezone,
        })
    }
}
