use anyhow::{Context as _, Result};
use serenity::{
    all::CommandInteraction,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
};

use crate::{data::db::Database, db::timezones::timezone::Timezone};

pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    let data = ctx.data.read().await;

    let pool = data
        .get::<Database>()
        .context("Expected `Database` in TypeMap")?;

    let res = match Timezone::get(command.user.id, pool).await {
        Ok(t) => format!("Your timezone is `{}`", t.timezone.name()),
        Err(_) => "No timezone is set".to_string(),
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