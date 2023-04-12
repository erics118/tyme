use anyhow::Result;
use serenity::{client::Context, model::application::interaction::Interaction};

use crate::interactions::commands::exec;

pub async fn run(ctx: Context, interaction: Interaction) -> Result<()> {
    match interaction {
        Interaction::ApplicationCommand(command) => {
            log::trace!("Received interaction command: {}", command.data.name);

            exec(ctx, command).await?;
            Ok(())
        },
        Interaction::MessageComponent(_component) => Ok(()),
        Interaction::ModalSubmit(_modal) => Ok(()),
        Interaction::Autocomplete(_complete) => Ok(()),
        Interaction::Ping(_ping) => Ok(()),
    }
}
