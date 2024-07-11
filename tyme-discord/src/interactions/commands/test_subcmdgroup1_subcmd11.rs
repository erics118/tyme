use anyhow::Result;
use serenity::{
    all::{CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
    model::application::CommandInteraction,
};

use crate::get_options;

/// a
pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    let ((val,),) = get_options!(command, .[[String]]);

    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content(val),
            ),
        )
        .await?;

    Ok(())
}
