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
        message_commands::MessageCommands,
        self_id::SelfId,
    },
    interactions, messages,
    utils::run::wrap_cmd,
};

pub async fn run(ctx: Context, ready: Ready) -> Result<()> {
    println!("{} is connected!", ready.user.name);
    let mut data = ctx.data.write().await;

    data.insert::<InteractionCommands>(HashMap::default());
    data.insert::<MessageCommands>(HashMap::default());
    data.insert::<SelfId>(ctx.cache.current_user_id());

    let commands = data
        .get_mut::<InteractionCommands>()
        .expect("Expected InteractionCommands in TypeMap.");

    commands.insert(
        "test".into(),
        InteractionCommand {
            run: wrap_cmd::<ApplicationCommandInteraction, _>(interactions::commands::test::run),
        },
    );

    let commands = Command::set_global_application_commands(&ctx.http, |commands| {
        commands
            .create_application_command(|command| interactions::commands::test::register(command))
    })
    .await
    .context("Unable to create global application commands")?;

    println!(
        "registered commands: {:?}",
        commands.iter().map(|c| c.name.as_str()).collect::<Vec<_>>()
    );

    let msg_cmds = data
        .get_mut::<MessageCommands>()
        .expect("Expected MessageCommands in TypeMap.");
    msg_cmds.insert(
        "shutdown".into(),
        wrap_cmd::<Message, _>(messages::commands::shutdown::run),
    );
    Ok(())
}
