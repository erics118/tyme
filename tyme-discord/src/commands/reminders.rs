use crate::types::*;

#[poise::command(
    slash_command,
    subcommands(
        "crate::commands::reminders_list::list",
        "crate::commands::reminders_delete::delete",
    )
)]
pub async fn reminders(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}
