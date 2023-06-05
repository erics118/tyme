use anyhow::{Context as _, Result};
use serenity::{
    all::CommandInteraction,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
};
use tyme_db::Timezone;

use crate::data::database::Database;

pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    let data = ctx.data.read().await;

    let db = data
        .get::<Database>()
        .context("Expected `Database` in TypeMap")?;

    let res = match Timezone::delete(db, command.user.id).await {
        // TODO: fetch the deleted timezone and show what was deleted
        Ok(_) => format!("deleted"),
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
