mod interactions;

use std::{collections::HashMap, env, future::Future, pin::Pin};

use anyhow::{Context as AnyhowContext, Result};
use serenity::{
    async_trait,
    model::{
        application::{command::Command, interaction::Interaction},
        gateway::Ready,
        prelude::{interaction::application_command::ApplicationCommandInteraction, *},
    },
    prelude::*,
};

struct InteractionCommands;

impl TypeMapKey for InteractionCommands {
    type Value = HashMap<String, CommandRun>;
}

struct Handler;

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + Sync + 'static>>;
type CommandRun = Box<
    dyn Fn(Context, ApplicationCommandInteraction) -> BoxFuture<anyhow::Result<()>>
        + Send
        + Sync
        + 'static,
>;

fn wrap_cmd<F>(f: fn(Context, ApplicationCommandInteraction) -> F) -> CommandRun
where
    F: Future<Output = anyhow::Result<()>> + Send + Sync + 'static,
{
    Box::new(move |ctx, command| Box::pin(f(ctx, command)))
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {}", command.data.name);
            ctx.data
                .read()
                .await
                .get::<InteractionCommands>()
                .expect("Expected InteractionCommands in TypeMap.")
                .get(&command.data.name)
                .expect("unknown command")(ctx.clone(), command)
            .await
            .expect("command execution failed");
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let mut data = ctx.data.write().await;
        let commands = data
            .get_mut::<InteractionCommands>()
            .expect("Expected InteractionCommands in TypeMap.");

        commands.insert("test".into(), wrap_cmd(interactions::commands::test::run));

        let Ok(commands) = Command::set_global_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|command| interactions::commands::test::register(command))
        })
        .await else {
            println!("f");
            return;
        };

        println!(
            "registered commands: {:?}",
            commands.iter().map(|c| c.name.as_str()).collect::<Vec<_>>()
        );
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // simple_logger::SimpleLogger::new()
    //     .with_utc_timestamps()
    //     .with_colors(trnwrap();
    let token = env::var("DISCORD_TOKEN").context("Missing `DISCORD_TOKEN` env var")?;

    let mut client = Client::builder(token, GatewayIntents::GUILD_MESSAGES)
        .event_handler(Handler)
        .await
        .context("Error creating client")?;
    {
        let mut data = client.data.write().await;
        data.insert::<InteractionCommands>(HashMap::default());
    }
    client.start().await?;
    Ok(())
}
