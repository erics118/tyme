use anyhow::{Context as _, Result};
use serenity::{
    all::{CommandInteraction, ResolvedOption, ResolvedValue},
    builder::{
        CreateCommand, CreateCommandOption, CreateInteractionResponse,
        CreateInteractionResponseMessage,
    },
    client::Context,
    model::application::CommandOptionType,
};

use crate::{
    data::db::Database,
    db::reminders::{create_reminder::create_reminder, reminder::Reminder},
};

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

#[allow(clippy::significant_drop_tightening)]
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

    let r = Reminder {
        id: sqlx::types::Uuid::new_v4(),
        created_at: sqlx::types::chrono::Utc::now().naive_utc(),
        time: sqlx::types::chrono::Utc::now().naive_utc() + chrono::Duration::days(*days),
        message: description.to_string(),
        creator_id: command.user.id,
        channel_id: command.channel_id,
        // if guild_id exists, use the guild. otherwise, DMs
        guild_id: command.guild_id,
    };

    create_reminder(&pool, r).await?;

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