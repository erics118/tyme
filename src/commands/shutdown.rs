use anyhow::Result;

pub async fn shutdown(ctx: Context<'_>) -> Result<()> {
    ctx.framework()
        .shard_manager()
        .lock()
        .await
        .shutdown_all()
        .await;
    Ok(())
}
