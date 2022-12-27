use anyhow::{Context as AnyhowContext, Result};
use serenity::{client::Context, model::application::interaction::Interaction};

use crate::data::interaction_commands::InteractionCommands;

pub async fn run(ctx: Context, interaction: Interaction) -> Result<()> {
    if let Interaction::ApplicationCommand(command) = interaction {
        println!("Received command interaction: {}", command.data.name);
        let data = ctx.data.read().await;
        let commands = data
            .get::<InteractionCommands>()
            .context("Expected InteractionCommands in TypeMap.")?;
        let func = commands
            .get(&command.data.name)
            .context("unknown command")?;
        (func.run)(ctx.clone(), command)
            .await
            .context("command execution failed")?;
        Ok(())
    } else {
        Ok(())
    }
}
