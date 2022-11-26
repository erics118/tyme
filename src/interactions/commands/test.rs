use anyhow::Result;
use serenity::{
    builder::CreateApplicationCommand,
    model::application::interaction::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
    prelude::*,
};

pub async fn run<'a>(ctx: &'a Context, command: &'a ApplicationCommandInteraction) -> Result<()> {
    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("success"))
        })
        .await?;
    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("test").description("A test command")
}
