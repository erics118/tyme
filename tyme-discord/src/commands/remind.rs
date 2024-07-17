use chrono::Utc;
use tyme_db::Reminder;

use crate::{
    types::*,
    utils::{
        human_time::{CheckedAddHumanTime, HumanTime},
        timestamp::{DiscordTimestamp, TimestampFormat},
    },
};

/// List your upcoming reminders.
#[poise::command(slash_command)]
pub async fn remind(
    ctx: Context<'_>,
    #[description = "When to remind you"] when: String,
    #[description = "What to remind you about"] message: String,
) -> Result<(), Error> {
    let db = ctx.data().db.lock().await;

    let now = ctx.created_at().naive_utc();

    // parse `when`
    let Ok(a) = HumanTime::parse(&when) else {
        ctx.reply("Invalid time.").await?;

        return Ok(());
    };

    let Ok(now) = now.checked_add(a) else {
        ctx.reply("Invalid time.").await?;

        return Ok(());
    };

    // create reminder
    let r = Reminder {
        id: None,
        created_at: Utc::now().naive_utc(),
        time: now,
        message: message.to_string(),
        user_id: ctx.author().id.into(),
        channel_id: ctx.channel_id().into(),
        guild_id: ctx.guild_id().map(u64::from),
    };

    r.create(&db).await?;

    let msg = format!(
        "Reminder set for {} on {}",
        r.message,
        r.time.discord_timestamp(TimestampFormat::ShortDateTime)
    );
    ctx.reply(msg).await?;

    Ok(())
}
