use std::collections::HashMap;

use anyhow::{Context as AnyhowContext, Result};
use serenity::{
    client::Context,
    model::{
        application::{
            command::Command, interaction::application_command::ApplicationCommandInteraction,
        },
        channel::Message,
        gateway::Ready,
    },
};

use crate::{
    data::{
        interaction_commands::{InteractionCommand, InteractionCommands},
        message_commands::{MessageCommand, MessageCommands},
    },
    interactions, messages,
    utils::run::wrap_cmd,
};

macro_rules! store_interaction_command {
    ($map:ident, $cmd:tt) => {
        $map.insert(
            interactions::commands::$cmd::NAME.to_string(),
            InteractionCommand {
                run: wrap_cmd::<ApplicationCommandInteraction, _>(
                    interactions::commands::$cmd::run,
                ),
                register: Box::new(interactions::commands::$cmd::register),
            },
        )
    };
}

macro_rules! store_message_command {
    ($map:ident, $cmd:tt) => {
        $map.insert(
            messages::commands::$cmd::NAME.to_string(),
            MessageCommand {
                run: wrap_cmd::<Message, _>(messages::commands::$cmd::run),
            },
        )
    };
}

pub async fn run(ctx: Context, ready: Ready) -> Result<()> {
    log::info!("{} is connected!", ready.user.name);

    let mut data = ctx.data.write().await;

    data.insert::<InteractionCommands>(HashMap::default());
    data.insert::<MessageCommands>(HashMap::default());

    let int_cmds = data
        .get_mut::<InteractionCommands>()
        .expect("Expected InteractionCommands in TypeMap");

    store_interaction_command!(int_cmds, test);

    log::trace!("Stored interaction commands");

    if false {
        let commands = Command::set_global_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|command| {
                interactions::commands::test::register(command)
            })
        })
        .await
        .context("Unable to create global application commands")?;

        log::info!(
            "Registered interaction commands to discord: {:?}",
            commands.iter().map(|c| c.name.as_str()).collect::<Vec<_>>()
        );
    }

    let msg_cmds = data
        .get_mut::<MessageCommands>()
        .expect("Expected MessageCommands in TypeMap");

    store_message_command!(msg_cmds, shutdown);

    log::trace!("Stored all message commands");

    Ok(())
}
