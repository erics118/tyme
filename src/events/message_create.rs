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

    if message.content.starts_with(&self_id.mention().to_string()) {
        let content = message
            .content
            .get(self_id.mention().to_string().len()..)
            .unwrap()
            .to_string();

        let commands = data
            .get::<MessageCommands>()
            .context("Expected MesageCommands in TypeMap.")?;
        let command_name = &content.split(' ').next().unwrap();

        commands
            .get(&command_name.to_string())
            .context("invalid command")?(ctx.clone(), message);
    }

    Ok(())
}
