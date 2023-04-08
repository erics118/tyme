use color_eyre::eyre::Result;
use serenity::{
    client::Context,
    model::{
        application::interaction::Interaction,
        prelude::interaction::application_command::ApplicationCommandInteraction,
    },
};

use crate::interaction_commands;

struct InteractionCommands;

interaction_commands!(test);

pub async fn run(ctx: Context, interaction: Interaction) -> Result<()> {
    match interaction {
        Interaction::ApplicationCommand(command) => {
            log::trace!("Received interaction command: {}", command.data.name);

            // let commands = data
            //     .get::<InteractionCommands>()
            //     .context("Expected InteractionCommands in TypeMap")?;
            //
            // let cmd = commands
            //     .get(&command.data.name)
            //     .context("Unknown command")?;
            //
            // (cmd.run)(ctx.clone(), command)
            //     .await
            //     .context("Command execution failed")?;
            InteractionCommands::exec(ctx, command).await?;
            Ok(())
        },
        Interaction::MessageComponent(_component) => Ok(()),
        Interaction::ModalSubmit(_modal) => Ok(()),
        Interaction::Autocomplete(_complete) => Ok(()),
        Interaction::Ping(_ping) => Ok(()),
    }
}
