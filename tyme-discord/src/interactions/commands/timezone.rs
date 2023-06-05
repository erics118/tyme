use anyhow::{Context as _, Result};
use serenity::{
    all::CommandInteraction,
    builder::{CreateCommand, CreateCommandOption},
    client::Context,
    model::application::CommandOptionType,
};

/// Create the timezone command.
pub fn register() -> CreateCommand {
    CreateCommand::new("timezone")
        .description("*")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "set",
                "Set your default timezone",
            )
            .add_sub_option(
                CreateCommandOption::new(CommandOptionType::String, "timezone", "Timezone to set")
                    .set_autocomplete(true)
                    .required(true),
            ),
        )
        .add_option(CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "get",
            "Get your default timezone",
        ))
        .add_option(CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "delete",
            "Delete your default timezone",
        ))
}

/// Handle the timezone command.
pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    let o = command.data.options();

    let subcommand = &o.get(0).context("missing option")?;

    match subcommand.name {
        "set" => super::timezone_set::run(ctx, command).await?,
        "get" => super::timezone_get::run(ctx, command).await?,
        "delete" => super::timezone_delete::run(ctx, command).await?,
        _ => unreachable!(),
    };

    Ok(())
}
