mod interactions;

use std::env;

use anyhow::{Context as AnyhowContext, Result};
use serenity::{
    async_trait,
    model::{
        application::{command::Command, interaction::Interaction},
        channel::Message,
        gateway::Ready,
        prelude::*,
    },
    prelude::*,
};

struct Handler;

// struct Handler<'a> {
//     interaction_commands:
//         HashMap<String, fn(&'a Context, &'a ApplicationCommandInteraction) ->
// Result<()>>, }
// impl<'a> Handler<'a> {
// fn register_interaction_command(
// &mut self,
// name: String,
// command: Box<
// for<'b> fn(
// &'a Context,
// &'b ApplicationCommandInteraction,
// ) -> dyn std::future::Future<Output = Result<(), anyhow::Error>>,
// >,
// ) {
// self.interaction_commands.insert(name, command);
// }
// }

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            match command.data.name.as_str() {
                "test" => interactions::commands::test::run(&ctx, &command),
                _ => todo!(),
            }
            .await
            .unwrap_or_else(|e| eprintln!("error: {}", e));
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        // pings the bot
        if msg
            .content
            .trim()
            .starts_with(&ctx.cache.current_user_id().mention().to_string())
        {
            let Some(content) = msg
                .content
                .trim()
                .strip_prefix(&ctx.cache.current_user_id().mention().to_string()) else {
                    return;
                };
            println!("{}", content);
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        if 1 == 0 {
            let Ok(commands) = Command::set_global_application_commands(&ctx.http, |commands| {
                commands.create_application_command(|command| interactions::commands::test::register(command))
            })
            .await else {
                println!("f");
                return;
            };
            // self.register_interaction_command("test".to_string(), commands::test::run);
            println!(
                "registered commands: {:?}",
                commands.iter().map(|c| c.name.as_str()).collect::<Vec<_>>()
            );
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let token = env::var("DISCORD_TOKEN").context("Missing `DISCORD_TOKEN` env var")?;

    let mut client = Client::builder(token, GatewayIntents::GUILD_MESSAGES)
        .event_handler(Handler {})
        .await
        .context("Error creating client")?;

    client.start().await?;
    Ok(())
}
