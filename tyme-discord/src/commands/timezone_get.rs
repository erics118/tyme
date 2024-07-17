use tyme_db::Timezone;

use crate::types::*;

/// Gets your timezone.
#[poise::command(slash_command)]
pub async fn get(ctx: Context<'_>) -> Result<(), Error> {
    let db = ctx.data().db.lock().await;

    let res = Timezone::get(&db, ctx.author().id.into())
        .await
        .map_or_else(
            |_| "No timezone is set".to_string(),
            |t| format!("Your timezone is `{}`", t.timezone.name()),
        );

    ctx.reply(res).await?;

    Ok(())
}
