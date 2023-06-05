use anyhow::{Context as _, Result};
use serenity::{
    all::CommandInteraction,
    builder::{CreateCommand, CreateCommandOption},
    client::Context,
    model::application::CommandOptionType,
};

/// Create the reminders command.
pub fn register() -> CreateCommand {
    CreateCommand::new("reminders")
        .description("*")
        .add_option(CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "list",
            "List your upcoming reminders",
        ))
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "delete",
                "Delete an upcoming reminder",
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "reminder_id",
                    "Timezone to set",
                )
                .set_autocomplete(true)
                .required(true),
            ),
        )
}

/// Handle the reminders command.
pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    let o = command.data.options();

    let subcommand = &o.get(0).context("missing option")?;

    match subcommand.name {
        "list" => super::reminders_list::run(ctx, command).await?,
        "delete" => super::reminders_delete::run(ctx, command).await?,
        _ => unreachable!(),
    };

    Ok(())
}
