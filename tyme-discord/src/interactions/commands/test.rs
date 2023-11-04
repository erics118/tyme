use anyhow::Result;
use serenity::{
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
    model::application::CommandInteraction,
};

use crate::create_command;

create_command! {
    / test
    | "A test command"
}

/// Handle the test command.
pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content("success"),
            ),
        )
        .await?;

    Ok(())
}
