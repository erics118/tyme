use anyhow::{Context as AnyhowContext, Result};
use serenity::{
    client::Context,
    model::{mention::Mentionable, prelude::Message},
};

use crate::data::{message_commands::MessageCommands, self_id::SelfId};

pub async fn run(ctx: Context, message: Message) -> Result<()> {
    let data = ctx.data.read().await;

    let self_id = data
        .get::<SelfId>()
        .context("Expected SelfId in TypeMap.")?;

    let mention = &self_id.mention().to_string();

    if message.content.starts_with(mention) {
        log::trace!("Message command invoked");
        let content = message.content.get(mention.len()..).unwrap().to_string();

        let commands = data
            .get::<MessageCommands>()
            .context("Expected MesageCommands in TypeMap.")?;

        let command_name = &content.split(' ').next().unwrap();

        log::trace!("Command was {command_name}");

        commands
            .get(&command_name.to_string())
            .context("Invalid command")?(ctx.clone(), message);
    }

    Ok(())
}
