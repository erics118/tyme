use anyhow::{Context as _, Result};
use chrono_tz::Tz;
use serenity::{
    all::CommandInteraction,
    builder::{CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
};
use tyme_db::{reminders::reminder::Reminder, timezones::timezone::Timezone};
use tyme_utils::pretty::Pretty;

use crate::data::database::Database;

pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    let data = ctx.data.read().await;

    let pool = data
        .get::<Database>()
        .context("Expected `Database` in TypeMap")?;

    let reminders = Reminder::get_all_by_user_id(pool, command.user.id).await?;

    // get user's timezone
    let timezone: Tz = Timezone::get(pool, command.user.id)
        .await
        .map_or_else(|_| Tz::UTC, |t| t.timezone);

    let embed = CreateEmbed::new().title("Reminders").description(
        reminders
            .iter()
            .map(|r| format!("{} - {} - {}", r.id, r.message, r.time.pretty(timezone)))
            .collect::<Vec<_>>()
            .join("\n"),
    );

    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(embed),
            ),
        )
        .await?;

    Ok(())
}
