use std::str::FromStr;

use anyhow::{Context as _, Result};
use serenity::{
    all::{CommandInteraction, ResolvedValue},
    client::Context,
};
use tyme_db::Reminder;

use crate::{
    create_message,
    data::database::Database,
    utils::timestamp::{DiscordTimestamp, TimestampFormat},
};

/// Delete a reminder.
pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    let o = command.data.options();

    let subcommand = o.get(0).context("missing option")?;

    let ResolvedValue::SubCommand(ref a) = subcommand.value else { panic!("f")};

    let ResolvedValue::String(id) = &a.get(0).context("missing option")?.value else {
        anyhow::bail!("incorrect resolved option type")
    };

    let Ok(id) = u32::from_str(id) else {
        command
        .create_response(
            &ctx.http,
            create_message!(/ "Invalid id format",),
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
        .create_response(&ctx.http, create_message!(/ res,))
        .await?;

    Ok(())
}
