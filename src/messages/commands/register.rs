use anyhow::Result;
use serenity::{client::Context, model::channel::Message};

use crate::{
    data::interaction_commands::InteractionCommands, utils::register::register_interaction_commands,
};

pub static NAME: &str = "register";

pub async fn run(ctx: Context, message: Message) -> Result<()> {
    let data = ctx.data.read().await;

    let int_cmds = data
        .get::<InteractionCommands>()
        .expect("Expected InteractionCommands in TypeMap");

    let commands = register_interaction_commands(&ctx.http, int_cmds).await?;

    let content = format!(
        "Registered interaction commands to discord: {:?}",
        commands.iter().map(|c| c.name.as_str()).collect::<Vec<_>>()
    );

    log::info!("{content}");

    message
        .channel_id
        .send_message(&ctx.http, |msg| msg.content(content))
        .await?;

    Ok(())
}
