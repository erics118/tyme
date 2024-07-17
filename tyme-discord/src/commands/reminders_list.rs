use poise::{serenity_prelude::CreateEmbed, CreateReply};
use tyme_db::{chrono_tz::Tz, Reminder, Timezone};

use crate::{types::*, utils::pretty::Pretty};

/// Lists all reminders.
#[poise::command(slash_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let db = ctx.data().db.lock().await;

    let reminders = Reminder::get_all_by_user_id(&db, ctx.author().id.into()).await?;

    // get user's timezone
    let timezone: Tz = Timezone::get(&db, ctx.author().id.into())
        .await
        .map_or_else(|_| Tz::UTC, |t| t.timezone);

    let embed = CreateEmbed::new().title("Reminders").description(
        reminders
            .iter()
            .map(|r| {
                format!(
                    "{} - {} - {}",
                    r.id.unwrap_or(7057),
                    r.message,
                    r.time.pretty(timezone)
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
    );

    ctx.send(CreateReply {
        embeds: vec![embed],
        ..Default::default()
    })
    .await?;

    Ok(())
}
