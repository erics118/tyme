use anyhow::Result;
use serenity::{
    builder::CreateApplicationCommand,
    client::Context,
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        prelude::{
            command::CommandOptionType,
        },
    },
};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("remind")
        .description("Remind you about something")
        .create_option(|option| {
            option
                .name("what")
                .description("What to remind you about")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("when")
                .description("When to remind you")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

pub async fn run(ctx: Context, command: ApplicationCommandInteraction) -> Result<()> {
    command
    .create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content(format!("{:?}", "fds")))
    })
    .await?;

    Ok(())
}
