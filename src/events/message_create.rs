use anyhow::{Context as AnyhowContext, Result};
use serenity::{
    client::Context,
    model::{mention::Mentionable, prelude::Message},
};

use crate::data::message_commands::MessageCommands;

pub async fn run(ctx: Context, message: Message) -> Result<()> {
    let mention = ctx.cache.current_user_id().mention().to_string();

    if message.content.starts_with(&mention) {
        log::trace!("Message command invoked");

        let content = message
            .content
            .get(mention.len()..)
            .unwrap()
            .trim()
            .to_string();

        let data = ctx.data.read().await;

        let commands = data
            .get::<MessageCommands>()
            .context("Expected MesageCommands in TypeMap.")?;

        let command_name = &content.split(' ').next().unwrap();

        log::trace!("Recieved message command: {command_name}");

        let cmd = commands
            .get(&command_name.to_string())
            .context("Invalid command")?;

        (cmd.run)(ctx.clone(), message);
    }

    Ok(())
}
