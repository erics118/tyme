use std::str::FromStr;

use anyhow::{Context as _, Result};
use serenity::{
    all::CommandInteraction,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
};
use tyme_db::Reminder;

use crate::{
    data::database::Database,
    get_options,
    utils::timestamp::{DiscordTimestamp, TimestampFormat},
};

/// Delete a reminder.
pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    let (id,) = get_options!(command, .String);

    let Ok(id) = u32::from_str(id) else {
        command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content("Invalid id format"),
            ),
        )
        .await?;

        return Ok(());
    };

    let db = {
        let data = ctx.data.read().await;
        data.get::<Database>()
            .context("Expected `Database` in TypeMap")?
            .clone()
    };

    let res = Reminder::delete_one_by_id(&db, id).await.map_or_else(
        |_| format!("Reminder with id `{}` does not exist", id),
        |r| {
            format!(
                "Deleted reminder of {}\nSet {}",
                r.message,
                r.created_at.discord_timestamp(TimestampFormat::Relative),
            )
        },
    );

    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content(res),
            ),
        )
        .await?;

    Ok(())
}
