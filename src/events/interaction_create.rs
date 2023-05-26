use anyhow::Result;
use serenity::{client::Context, model::application::Interaction};

use crate::interactions::commands::exec;

pub async fn run(ctx: Context, interaction: Interaction) -> Result<()> {
    match interaction {
        Interaction::Command(command) => {
            log::trace!("Received interaction command: {}", command.data.name);

            exec(ctx, command).await?;
            Ok(())
        },
        _ => Ok(()),
    }
}
