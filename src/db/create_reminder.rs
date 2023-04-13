use anyhow::Result;
use sqlx::{postgres::PgPool, types::uuid::Uuid};

pub async fn create_reminder(pool: &PgPool, days: i64, description: String) -> Result<Uuid> {
    let rec = sqlx::query!(
        r#"
INSERT INTO reminders (id, created_at, time, message)
VALUES ( $1::UUID, $2::TIMESTAMP, $3::TIMESTAMP, $4::TEXT )
RETURNING id;
        "#,
        Uuid::new_v4(),
        sqlx::types::chrono::Utc::now().naive_utc(),
        sqlx::types::chrono::Utc::now().naive_utc() + chrono::Duration::days(days.into()),
        description
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}
