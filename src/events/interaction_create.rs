use color_eyre::eyre::{bail, ContextCompat, Result, WrapErr};
use serenity::{client::Context, model::application::interaction::Interaction};

use crate::data::interaction_commands::InteractionCommands;

pub async fn run(ctx: Context, interaction: Interaction) -> Result<()> {
    let data = ctx.data.read().await;

    match interaction {
        Interaction::ApplicationCommand(command) => {
            log::trace!("Received interaction command: {}", command.data.name);

            let commands = data
                .get::<InteractionCommands>()
                .context("Expected InteractionCommands in TypeMap")?;

            let cmd = commands
                .get(&command.data.name)
                .context("Unknown command")?;

            (cmd.run)(ctx.clone(), command)
                .await
                .context("Command execution failed")?;

            Ok(())
        },
        Interaction::MessageComponent(_component) => Ok(()),
        Interaction::ModalSubmit(_modal) => Ok(()),
        Interaction::Autocomplete(_complete) => Ok(()),
        _ => bail!("Uh oh"),
    }
}
