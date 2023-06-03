use anyhow::{Context, Result};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use serenity::model::id::UserId;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timezone {
    pub user_id: UserId,
    pub timezone: Tz,
}

impl Timezone {
    pub async fn get(pool: &Mutex<sqlx::PgPool>, user_id: UserId) -> Result<Self> {
        let pool = pool.lock().await;

        let row = sqlx::query!(
            r#"
            SELECT timezone
            FROM timezones
            WHERE user_id = $1::BIGINT;
            "#,
            i64::from(user_id),
        )
        .fetch_optional(&*pool)
        .await?
        .context("does not exist")?;

        let timezone = Tz::from_str_insensitive(&row.timezone)
            .map_err(|_| anyhow::anyhow!("database corrupted, timezone invalid"))?;

        Ok(Self { user_id, timezone })
    }

    pub async fn set(&self, pool: &Mutex<sqlx::PgPool>) -> Result<Self> {
        let pool = pool.lock().await;

        // either update row or create new row
        let row = sqlx::query!(
            r#"
            INSERT INTO timezones (user_id, timezone)
            VALUES ($1::BIGINT, $2::TEXT)
            ON CONFLICT (user_id)
            DO UPDATE SET timezone = EXCLUDED.timezone
            RETURNING *;
            "#,
            i64::from(self.user_id),
            self.timezone.name()
        )
        .fetch_one(&*pool)
        .await?;

        let timezone = Tz::from_str_insensitive(&row.timezone)
            .map_err(|_| anyhow::anyhow!("database corrupted, timezone invalid"))?;

        let user_id = UserId::from(row.user_id as u64);

        Ok(Self { user_id, timezone })
    }

    pub async fn delete(pool: &Mutex<sqlx::PgPool>, user_id: UserId) -> Result<Self> {
        let pool = pool.lock().await;

        let row = sqlx::query!(
            r#"
            DELETE FROM timezones
            WHERE user_id = $1::BIGINT
            RETURNING *;
            "#,
            i64::from(user_id),
        )
        .fetch_optional(&*pool)
        .await?
        .context("does not exist")?;

        let timezone = Tz::from_str_insensitive(&row.timezone)
            .map_err(|_| anyhow::anyhow!("database corrupted, timezone invalid"))?;

        Ok(Self { user_id, timezone })
    }
}
