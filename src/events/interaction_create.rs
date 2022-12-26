use anyhow::{Context as AnyhowContext, Result};
use serenity::{model::application::interaction::Interaction, prelude::*};

use crate::interactions::commands::InteractionCommands;

pub async fn run(ctx: Context, interaction: Interaction) -> Result<()> {
    if let Interaction::ApplicationCommand(command) = interaction {
        println!("Received command interaction: {}", command.data.name);
        ctx.data
            .read()
            .await
            .get::<InteractionCommands>()
            .context("Expected InteractionCommands in TypeMap.")?
            .get(&command.data.name)
            .context("unknown command")?(ctx.clone(), command)
        .await
        .context("command execution failed")?;
        Ok(())
    } else {
        Ok(())
    }
}
