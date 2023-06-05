use anyhow::Result;
use serenity::{
    all::CommandInteraction,
    builder::{CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
};

/// Create the test command.
pub fn register() -> CreateCommand {
    CreateCommand::new("test").description("A test command")
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
