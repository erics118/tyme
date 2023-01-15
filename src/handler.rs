use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{application::interaction::Interaction, channel::Message, gateway::Ready},
};
use tokio_postgres::Client as DbClient;

use crate::{events, utils::catch::catch};

pub struct Handler {
    pub db: DbClient,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        log::trace!("Running message create event");

        catch(events::message_create::run(ctx, message, self.db)).await;

        log::trace!("Ran message create event");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        log::trace!("Running interaction create event");

        catch(events::interaction_create::run(ctx, interaction, self.db)).await;

        log::trace!("Ran interaction create event");
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        log::trace!("Running ready event");

        catch(events::ready::run(ctx, ready)).await;

        log::trace!("Ran ready event");
    }
}
