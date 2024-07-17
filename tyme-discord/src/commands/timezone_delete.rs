use tyme_db::Timezone;

use crate::types::*;

/// Deletes your timezone.
#[poise::command(slash_command)]
pub async fn delete(ctx: Context<'_>) -> Result<(), Error> {
    let db = ctx.data().db.lock().await;

    let res = Timezone::delete(&db, ctx.author().id.into())
        .await
        .map_or_else(
            |_| "No timezone is set.".to_string(),
            |t| format!("Your timezone of `{}` has been deleted.", t.timezone),
        );

    ctx.reply(res).await?;

    Ok(())
}
