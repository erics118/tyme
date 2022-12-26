use serenity::{
    async_trait,
    model::{application::interaction::Interaction, gateway::Ready},
    prelude::*,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if crate::events::interaction_create::run(ctx, interaction)
            .await
            .is_ok()
        {
            println!("good");
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        if crate::events::ready::run(ctx, ready).await.is_ok() {
            println!("good");
        }
    }
}
