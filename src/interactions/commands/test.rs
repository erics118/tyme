use color_eyre::eyre::Result;
use serenity::{
    builder::CreateApplicationCommand,
    client::Context,
    model::application::interaction::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("test").description("A test command")
}

pub async fn run(ctx: Context, command: ApplicationCommandInteraction) -> Result<()> {
    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("success"))
        })
        .await?;

    Ok(())
}
