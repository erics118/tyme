//! Handlers for events.

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{application::Interaction, channel::Message, gateway::Ready},
};

use crate::events;

/// Struct that handles all events.
#[derive(Debug, Copy, Clone)]
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        log::trace!("Running message create event");

        if let Err(err) = events::message_create::run(ctx, message).await {
            log::error!("{:?}", err);
        }

        log::trace!("Ran message create event");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        log::trace!("Running interaction create event");

        if let Err(err) = events::interaction_create::run(ctx, interaction).await {
            log::error!("{:?}", err);
        }

        log::trace!("Ran interaction create event");
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        log::trace!("Running ready event");

        if let Err(err) = events::ready::run(ctx, ready).await {
            log::error!("{:?}", err);
        }

        log::trace!("Ran ready event");
    }
}
