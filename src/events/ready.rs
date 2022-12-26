use std::{collections::HashMap, future::Future, pin::Pin};

use anyhow::{Context as AnyhowContext, Result};
use serenity::{
    model::{
        application::command::Command, gateway::Ready,
        prelude::interaction::application_command::ApplicationCommandInteraction,
    },
    prelude::*,
};

use crate::{interactions, interactions::commands::InteractionCommands};

pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + Sync + 'static>>;

pub type Run<T> = Box<dyn Fn(Context, T) -> BoxFuture<anyhow::Result<()>> + Send + Sync + 'static>;

impl TypeMapKey for InteractionCommands {
    type Value = HashMap<String, Run<ApplicationCommandInteraction>>;
}

pub fn wrap_cmd<T: 'static, F>(f: fn(Context, T) -> F) -> Run<T>
where
    F: Future<Output = anyhow::Result<()>> + Send + Sync + 'static,
{
    Box::new(move |ctx, command| Box::pin(f(ctx, command)))
}

pub async fn run(ctx: Context, ready: Ready) -> Result<()> {
    println!("{} is connected!", ready.user.name);
    let mut data = ctx.data.write().await;
    let commands = data
        .get_mut::<InteractionCommands>()
        .expect("Expected InteractionCommands in TypeMap.");

    commands.insert(
        "test".into(),
        wrap_cmd::<ApplicationCommandInteraction, _>(interactions::commands::test::run),
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
    Ok(())
}
