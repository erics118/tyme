use anyhow::{Context as AnyhowContext, Result};
use serenity::{
    all::{CommandInteraction, ResolvedOption, ResolvedValue},
    builder::{
        CreateCommand, CreateCommandOption, CreateInteractionResponse,
        CreateInteractionResponseMessage,
    },
    client::Context,
    model::application::CommandOptionType,
};

use crate::{data::db::Database, db::reminders::create_reminder::create_reminder};

pub fn register() -> CreateCommand {
    CreateCommand::new("remind")
        .description("Remind you about something")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "days",
                "In how many days to remind you",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "message",
                "What to remind you about",
            )
            .required(true),
        )
}

pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    let o = command.data.options();
    let Some(ResolvedOption {
        value: ResolvedValue::Integer(days),
        ..
    }) = o.get(0)
     else {
        panic!("f")
    };

    let Some(ResolvedOption {
        value: ResolvedValue::String(description),
        ..
    }) = o.get(1)
     else {
        panic!("f")
    };

    let data = ctx.data.read().await;

    let db = data
        .get::<Database>()
        .context("Expected `Database` in TypeMap")?;

    let pool = db.lock().await;

    create_reminder(&pool, *days, description.to_string()).await?;

    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content(format!("{:?}", "fds")),
            ),
        )
        .await?;

    Ok(())
}
