mod commands;

use std::env;

use anyhow::{Context as AnyhowContext, Result};
use serenity::{
    async_trait,
    model::{
        application::{command::Command, interaction::Interaction},
        gateway::Ready,
    },
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            match command.data.name.as_str() {
                "test" => commands::test::run(&ctx, &command),
                _ => todo!(),
            }
            .await
            .unwrap_or_else(|e| eprintln!("error: {}", e));
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let Ok(commands) = Command::set_global_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|command| commands::test::register(command))
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
    let token = env::var("DISCORD_TOKEN").context("Missing `DISCORD_TOKEN` env var")?;

    let mut client = Client::builder(
        token,
        GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILDS,
    )
    .event_handler(Handler {})
    .await
    .context("Error creating client")?;

    client.start().await?;
    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<()> {
// let token = std::env::var("DISCORD_TOKEN").context("missing DISCORD_TOKEN")?;
//
// let intents =
// GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES |
// GatewayIntents::MESSAGE_CONTENT; let framework = poise::Framework::builder()
// .options(poise::FrameworkOptions {
// commands: vec![
// crate::commands::age::age(),
// crate::commands::register::register(),
// crate::commands::shutdown::shutdown(),
// ],
// todo: add on_error, log to a channel
// todo: add pre_command to log the command run in console
// allowed_mentions: Some(&*CreateAllowedMentions::default().empty_parse()),
// prefix_options: poise::PrefixFrameworkOptions {
// prefix: Some("~".into()),
// mention_as_prefix: true,
// execute_self_messages: false,
// ignore_bots: true,
// case_insensitive_commands: true,
// ..Default::default()
// },
// todo: add event handler
// ..Default::default()
// })
// .token(token)
// .intents(intents)
// .setup(|ctx, ready, framework| {
// Box::pin(async move {
// poise::builtins::register_globally(ctx,
// &framework.options().commands).await?; println!("{}", ready.user.name);
// Ok(Data {})
// })
// });
//
// framework.run().await?;
// Ok(())
// }
