use color_eyre::eyre::Result;
use serenity::{
    client::Context,
    model::{gateway::Ready, prelude::Activity},
};

pub async fn run(ctx: Context, ready: Ready) -> Result<()> {
    log::info!("Bot connected as: {}", ready.user.name);

    ctx.set_activity(Activity::listening("eirk")).await;
    log::trace!("Set status");

    Ok(())
}
