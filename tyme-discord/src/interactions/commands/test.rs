use anyhow::Result;
use serenity::{
    all::CommandInteraction,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
};

use crate::create_interaction_command;

create_interaction_command! {
    test
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
