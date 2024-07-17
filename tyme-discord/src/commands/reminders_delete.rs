use tyme_db::Reminder;

use crate::{
    types::*,
    utils::timestamp::{DiscordTimestamp, TimestampFormat},
};

// async fn autocomplete_reminder_id<'a>(ctx: Context<'_>, partial: &'a str) ->
// Vec<u64> {     let db = ctx.data().db.lock().await;
//     let reminder_ids = Reminder::get_all_by_user_id(&db,
// ctx.author().id.into())         .await
//         .unwrap_or_default()
//         .iter()
//         .map(|r| r.id.unwrap_or_default() as u64)
//         .collect::<Vec<_>>();

//     fuzzy_autocomplete::<u64>(partial, &reminder_ids)
// }

/// Deletes a reminder.
#[poise::command(slash_command)]
pub async fn delete(
    ctx: Context<'_>,
    #[description = "Reminder id to delete."]
    // #[autocomplete = "autocomplete_reminder_id"]
    id: u64,
) -> Result<(), Error> {
    let db = ctx.data().db.lock().await;

    let res = Reminder::delete_one_by_id(&db, id).await.map_or_else(
        |_| format!("Reminder with id `{}` does not exist", id),
        |r| {
            format!(
                "Deleted reminder of {}\nCreated {}",
                r.message,
                r.created_at.discord_timestamp(TimestampFormat::Relative),
            )
        },
    );

    ctx.reply(res).await?;

    Ok(())
}
