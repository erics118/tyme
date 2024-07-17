use crate::types::*;

/// Registers all commands.
#[poise::command(prefix_command, owners_only)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_globally(ctx, &ctx.framework().options().commands).await?;

    ctx.reply("Registered all application commands.").await?;

    Ok(())
}
