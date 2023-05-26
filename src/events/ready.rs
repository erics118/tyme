use anyhow::Result;
use serenity::{
    client::Context,
    gateway::ActivityData,
    model::gateway::{ActivityType, Ready},
};

pub async fn run(ctx: Context, ready: Ready) -> Result<()> {
    log::info!("Bot connected as: {}", ready.user.name);

    ctx.set_activity(Some(ActivityData {
        name: "eirk".to_string(),
        kind: ActivityType::Listening,
        url: None,
    }));
    log::trace!("Set status");

    Ok(())
}
