use anyhow::{Context as AnyhowContext, Result};
use serenity::{http::Http, model::application::command::Command, prelude::TypeMapKey};

use crate::data::interaction_commands::InteractionCommands;

pub async fn register_interaction_commands(
    http: impl AsRef<Http>,
    int_cmds: &mut <InteractionCommands as TypeMapKey>::Value,
) -> Result<Vec<Command>> {
    let commands = Command::set_global_application_commands(http, |commands| {
        commands.create_application_command(|command| {
            for (_, cmd) in int_cmds.iter() {
                (cmd.register)(command);
            }
            command
        })
    })
    .await
    .context("Unable to register interaction commands")?;

    Ok(commands)
}
