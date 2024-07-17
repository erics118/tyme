use crate::types::*;

#[poise::command(
    slash_command,
    subcommands(
        "crate::commands::timezone_delete::delete",
        "crate::commands::timezone_get::get",
        "crate::commands::timezone_set::set",
    )
)]
pub async fn timezone(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}
