use anyhow::{Context, Result};
use chrono_tz::Tz;
use serenity::model::id::UserId;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Timezone {
    pub user_id: UserId,
    pub timezone: Tz,
}

impl Timezone {
    pub async fn get(user_id: UserId, pool: &Mutex<sqlx::PgPool>) -> Result<Self> {
        let pool = pool.lock().await;

        let record = sqlx::query!(
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

        let timezone = match Tz::from_str_insensitive(&record.timezone) {
            Ok(t) => t,
            Err(_) => anyhow::bail!("database corrupted, timezone invalid"),
        };
        Ok(Self { user_id, timezone })
    }

    pub async fn set(&self, pool: &Mutex<sqlx::PgPool>) -> Result<()> {
        let pool = pool.lock().await;

        // either update row or create new row
        sqlx::query!(
            r#"
            INSERT INTO timezones (user_id, timezone)
            VALUES ($1::BIGINT, $2::TEXT)
            ON CONFLICT (user_id)
            DO UPDATE SET timezone = EXCLUDED.timezone;
            "#,
            i64::from(self.user_id),
            self.timezone.name()
        )
        .fetch_optional(&*pool)
        .await?;

        Ok(())
    }

    pub async fn remove(user_id: UserId, pool: &Mutex<sqlx::PgPool>) -> Result<Self> {
        let pool = pool.lock().await;

        let record = sqlx::query!(
            r#"
            DELETE FROM timezones
            WHERE user_id = $1::BIGINT
            RETURNING *
            "#,
            i64::from(user_id),
        )
        .fetch_optional(&*pool)
        .await?
        .context("does not exist")?;

        let timezone =
            Tz::from_str_insensitive(&record.timezone).unwrap_or_else(|_| panic!("uh oh"));

        Ok(Self { user_id, timezone })
    }
}