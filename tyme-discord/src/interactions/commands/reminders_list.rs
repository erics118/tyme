use anyhow::{Context as _, Result};
use chrono_tz::Tz;
use serenity::{
    all::CommandInteraction,
    builder::{CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
};
use tyme_db::{Reminder, Timezone};

use crate::{data::database::Database, utils::pretty::Pretty};

/// List all reminders of a user.
pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    let db = {
        let data = ctx.data.read().await;
        data.get::<Database>()
            .context("Expected `Database` in TypeMap")?
            .clone()
    };

    let reminders = Reminder::get_all_by_user_id(&db, command.user.id).await?;

    // get user's timezone
    let timezone: Tz = Timezone::get(&db, command.user.id)
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
