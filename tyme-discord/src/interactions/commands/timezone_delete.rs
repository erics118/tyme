use anyhow::{Context as _, Result};
use serenity::{all::CommandInteraction, client::Context};
use tyme_db::Timezone;

use crate::{create_message, data::database::Database};

/// Delete a user's timezone.
pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    let db = {
        let data = ctx.data.read().await;
        data.get::<Database>()
            .context("Expected `Database` in TypeMap")?
            .clone()
    };

    let res = Timezone::delete(&db, command.user.id).await.map_or_else(
        |_| "No timezone is set.".to_string(),
        |t| format!("Your timezone of `{}` has been deleted.", t.timezone),
    );

    command
        .create_response(&ctx.http, create_message!(/ res,))
        .await?;

    Ok(())
}
