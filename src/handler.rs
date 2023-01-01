use async_anyhow_logger::catch;
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
        log::trace!("Running message create event");

        catch(events::message_create::run(ctx, message)).await;

        log::trace!("Ran message create event");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        log::trace!("Running interaction create event");

        catch(events::interaction_create::run(ctx, interaction)).await;

        log::trace!("Ran interaction create event");
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        log::trace!("Running ready event");

        catch(events::ready::run(ctx, ready)).await;

        log::trace!("Ran ready event");
    }
}
