use anyhow::Result;
use serenity::{
    all::{CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
    model::application::CommandInteraction,
};

/// a
pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content("ok"),
            ),
        )
        .await?;

    Ok(())
}
