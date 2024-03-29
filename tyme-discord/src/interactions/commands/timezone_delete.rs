use anyhow::{Context as _, Result};
use serenity::{
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
    model::application::CommandInteraction,
};
use tyme_db::Timezone;

use crate::data::database::Database;

/// Delete a user's timezone.
pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    let db = {
        let data = ctx.data.read().await;
        data.get::<Database>()
            .context("Expected `Database` in TypeMap")?
            .clone()
    };

    let res = Timezone::delete(&db, command.user.id.into())
        .await
        .map_or_else(
            |_| "No timezone is set.".to_string(),
            |t| format!("Your timezone of `{}` has been deleted.", t.timezone),
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
