use std::str::FromStr;

use anyhow::{Context as _, Result};
use serenity::{
    all::{CommandInteraction, ResolvedValue},
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
};
use tyme_db::Reminder;

use crate::data::database::Database;

pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    let o = command.data.options();

    let subcommand = o.get(0).context("missing option")?;

    let ResolvedValue::SubCommand(ref a) = subcommand.value else { panic!("f")};

    let ResolvedValue::String(id) = &a.get(0).context("missing option")?.value else {
        anyhow::bail!("incorrect resolved option type")
    };

    let Ok (id) = u32::from_str(id) else {
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

    let data = ctx.data.read().await;

    let pool = data
        .get::<Database>()
        .context("Expected `Database` in TypeMap")?;

    let res = match Reminder::delete_one_by_id(pool, id).await {
        // TODO: fetch the deleted reminder and show what was deleted
        Ok(_) => format!("deleted"),
        Err(_) => format!("Reminder with id `{}` does not exist", id),
    };

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
