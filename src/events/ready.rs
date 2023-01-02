use std::collections::HashMap;

use anyhow::Result;
use serenity::{
    client::Context,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        channel::Message, gateway::Ready, prelude::Activity,
    },
};

use crate::{
    data::{
        interaction_commands::{InteractionCommand, InteractionCommands},
        message_commands::{MessageCommand, MessageCommands},
    },
    interactions, messages,
    utils::{
        register::register_interaction_commands,
        run::wrap_cmd,
        store::{store_interaction_command, store_message_command},
    },
};

pub async fn run(ctx: Context, ready: Ready) -> Result<()> {
    log::info!("Bot connected as: {}", ready.user.name);

    ctx.set_activity(Activity::listening("eirk")).await;
    log::trace!("Set status");

    let mut data = ctx.data.write().await;

    data.insert::<InteractionCommands>(HashMap::default());
    data.insert::<MessageCommands>(HashMap::default());

    let int_cmds = data
        .get_mut::<InteractionCommands>()
        .expect("Expected InteractionCommands in TypeMap");

    store_interaction_command!(int_cmds, test);

    log::trace!("Stored interaction commands");

    let commands = register_interaction_commands(&ctx.http, int_cmds).await?;

    log::info!(
        "Registered interaction commands to discord: {:?}",
        commands.iter().map(|c| c.name.as_str()).collect::<Vec<_>>()
    );

    let msg_cmds = data
        .get_mut::<MessageCommands>()
        .expect("Expected MessageCommands in TypeMap");

    store_message_command!(msg_cmds, shutdown);
    store_message_command!(msg_cmds, execute);

    log::trace!("Stored all message commands");

    Ok(())
}
