use anyhow::{Context as _, Result};
use chrono_tz::Tz;
use serenity::{
    all::CommandInteraction,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
};
use tyme_db::Timezone;

use crate::{data::database::Database, get_options};

/// Set a user's timezone.
pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    let (timezone,) = get_options!(command, .String);

    let timezone = match Tz::from_str_insensitive(timezone) {
        Ok(t) => t,
        Err(_) => {
            command
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                        format!(
                            "Invalid timezone. You can find them here: <https://en.wikipedia.org/wiki/List_of_tz_database_time_zones>",
                        ),
                    )),
                )
                .await?;
            return Ok(());
        },
    };

    let db = {
        let data = ctx.data.read().await;
        data.get::<Database>()
            .context("Expected `Database` in TypeMap")?
            .clone()
    };

    let t = Timezone {
        user_id: command.user.id,
        timezone,
    };

    t.set(&db).await?;

    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(format!("Set your timezone to `{}`", t.timezone.name())),
            ),
        )
        .await?;

    Ok(())
}
