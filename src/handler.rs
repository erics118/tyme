use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{application::interaction::Interaction, channel::Message, gateway::Ready},
};

use crate::events;
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        if events::message_create::run(ctx, message).await.is_ok() {
            println!("ran message event");
        }
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if events::interaction_create::run(ctx, interaction)
            .await
            .is_ok()
        {
            println!("ran interaction create event");
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        if events::ready::run(ctx, ready).await.is_ok() {
            println!("bot is ready");
        }
    }
}
