use anyhow::Result;
use sqlx::types::Uuid;

pub async fn create_reminder(pool: &sqlx::PgPool, days: i64, description: String) -> Result<Uuid> {
    let rec = sqlx::query!(
        r#"
INSERT INTO reminders (id, created_at, time, message)
VALUES (gen_random_uuid(), $1::TIMESTAMP, $2::TIMESTAMP, $3::TEXT)
RETURNING id;
        "#,
        sqlx::types::chrono::Utc::now().naive_utc(),
        sqlx::types::chrono::Utc::now().naive_utc() + chrono::Duration::days(days.into()),
        description
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}
