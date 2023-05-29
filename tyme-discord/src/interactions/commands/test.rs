use anyhow::Result;
use serenity::{
    all::CommandInteraction,
    builder::{CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
};

pub fn register() -> CreateCommand {
    CreateCommand::new("test").description("A test command")
}

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
