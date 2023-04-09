use color_eyre::eyre::Result;
use serenity::{
    client::Context,
    model::{application::command::Command, channel::Message},
};

use crate::interactions::commands::register_all;

pub async fn run(ctx: Context, _message: Message) -> Result<()> {
    Command::set_global_application_commands(ctx.http, |commands| register_all(commands)).await?;

    Ok(())
}
