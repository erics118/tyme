use anyhow::Result;
use serenity::{client::Context, model::application::Interaction};

use crate::interactions::{autocompletes, commands};

pub async fn run(ctx: Context, interaction: Interaction) -> Result<()> {
    match interaction {
        Interaction::Command(command) => {
            log::trace!("Received interaction command: {}", command.data.name);

            commands::exec(ctx, command).await?;
            Ok(())
        },
        Interaction::Autocomplete(autocomplete) => {
            log::trace!("Received autocomplete: {}", autocomplete.data.name);

            autocompletes::exec(ctx, autocomplete).await
        },
        _ => Ok(()),
    }
}
