use anyhow::{Context as AnyhowContext, Result};
use serenity::{
    builder::CreateApplicationCommand,
    client::Context,
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        prelude::{
            command::CommandOptionType, interaction::application_command::CommandDataOptionValue,
        },
    },
};

use crate::{data::db::Database, db::create_reminder::create_reminder};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("remind")
        .description("Remind you about something")
        .create_option(|option| {
            option
                .name("days")
                .description("In how many days to remind you")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("description")
                .description("What to remind you about")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

pub async fn run(ctx: Context, command: ApplicationCommandInteraction) -> Result<()> {
    let CommandDataOptionValue::Integer(days) = command.data.options
        .get(0)
        .expect("Expected days option")
        .resolved
        .as_ref()
        .expect("Expected days object") else {
            panic!("Expected days object");
        };

    let CommandDataOptionValue::String(description) = command.data.options
        .get(1)
        .expect("Expected description option")
        .resolved
        .as_ref()
        .expect("Expected description object") else {
            panic!("Expected description object");
        };

    let data = ctx.data.read().await;

    let db = data
        .get::<Database>()
        .context("Expected `Database` in TypeMap")?;

    let pool = db.lock().await;

    create_reminder(&pool, *days, description.to_string()).await?;

    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(format!("{:?}", "fds")))
        })
        .await?;

    Ok(())
}