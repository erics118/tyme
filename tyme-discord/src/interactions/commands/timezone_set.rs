use anyhow::{Context as _, Result};
use chrono_tz::Tz;
use serenity::{
    all::{CommandInteraction, ResolvedValue},
    client::Context,
};
use tyme_db::Timezone;

use crate::{create_message, data::database::Database};

/// Set a user's timezone.
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
                    create_message!(
                        / "Invalid timezone. You can find them here: <https://en.wikipedia.org/wiki/List_of_tz_database_time_zones>",
                    ),
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

    let msg = format!("Set your timezone to `{}`", t.timezone.name());

    command
        .create_response(&ctx.http, create_message!(/ msg,))
        .await?;

    Ok(())
}
