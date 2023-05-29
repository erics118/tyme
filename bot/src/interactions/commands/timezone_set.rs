use anyhow::{Context as _, Result};
use chrono_tz::Tz;
use serenity::{
    all::{CommandInteraction, ResolvedValue},
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
};

use crate::{data::db::Database, db::timezones::timezone::Timezone};

pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    let o = command.data.options();

    let subcommand = o.get(0).context("missing option")?;

    let ResolvedValue::SubCommand(ref a) = subcommand.value else { panic!("f")};

    let ResolvedValue::String(timezone) = &a.get(0)
        .context("missing option")?.value else {
        anyhow::bail!("incorrect resolved option type")
    };

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

    let data = ctx.data.read().await;

    let pool = data
        .get::<Database>()
        .context("Expected `Database` in TypeMap")?;

    let t = Timezone {
        user_id: command.user.id,
        timezone,
    };

    t.set(pool).await?;

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
