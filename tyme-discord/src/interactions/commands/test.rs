use anyhow::Result;
use serenity::{all::CommandInteraction, client::Context};

use crate::{create_command, create_message};

create_command! {
    / test
    | "A test command"
}

/// Handle the test command.
pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    command
        .create_response(&ctx.http, create_message!(/ "success",))
        .await?;

    Ok(())
}
