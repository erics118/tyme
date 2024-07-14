use anyhow::Result;
use serenity::{
    all::{
        ChannelType, CommandOptionType, CreateCommand, CreateCommandOption,
        CreateInteractionResponse, CreateInteractionResponseMessage,
    },
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

pub fn a() {
    let c = CreateCommand::new("command")
        .description("description")
        .add_option(
            CreateCommandOption::new(CommandOptionType::Boolean, "name", "description")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::Channel, "name", "description")
                .channel_types(vec![ChannelType::Forum, ChannelType::PrivateThread]),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "name", "description")
                .add_int_choice("name", 3 /* value */)
                .add_int_choice("name2", 5 /* value */)
                .min_int_value(1)
                .max_int_value(10),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::Mentionable, "name", "description")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::Number, "name", "description")
                .add_number_choice("name", 3.0)
                .max_number_value(10.0)
                .min_number_value(1.0),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::Role, "name", "description")
                .set_autocomplete(false),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "name", "description")
                .max_length(10)
                .min_length(1)
                .add_string_choice("hi", "aaa")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "name", "description")
                .set_autocomplete(true),
        );
}
