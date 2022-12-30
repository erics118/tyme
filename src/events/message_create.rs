use anyhow::{Context as AnyhowContext, Result};
use serenity::{
    client::Context,
    model::{mention::Mentionable, prelude::Message},
};

use crate::data::message_commands::MessageCommands;

pub async fn run(ctx: Context, message: Message) -> Result<()> {
    let mention = ctx.cache.current_user_id().mention().to_string();

    let owner_id = ctx
        .http
        .get_current_application_info()
        .await
        .context("Couldn't get application info")?
        .owner
        .id;

    if message.author.id == owner_id && message.content.starts_with(&mention) {
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
            .context("Expected MesageCommands in TypeMap")?;

        let command_name = &content.split(' ').next().unwrap().to_string();

        log::trace!("Received message command: {command_name}");

        let cmd = commands.get(command_name).context("Unknown command")?;

        (cmd.run)(ctx.clone(), message)
            .await
            .context("Command execution failed")?;
    }

    Ok(())
}
