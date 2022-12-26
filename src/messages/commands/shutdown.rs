use anyhow::{Context as AnyhowContext, Result};

pub async fn run(ctx: Context<'_>) -> Result<()> {
    ctx.framework()
        .shard_manager()
        .lock()
        .await
        .shutdown_all()
        .await;
    Ok(())
}
