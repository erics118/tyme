use poise::serenity_prelude::{CacheHttp, ChannelId, CreateMessage, Mentionable, UserId};
use tyme_db::{Pool, Reminder};

use crate::{
    types::*,
    utils::timestamp::{DiscordTimestamp, TimestampFormat},
};

/// Notify users of past reminders.
pub async fn notify_past_reminders(db: &Pool, http: impl CacheHttp) -> Result<()> {
    let reminders = Reminder::get_all_past_reminders(db).await?;

    for r in reminders {
        log::info!("{r:#?}");

        let message = format!(
            "Reminder for {}: {}\nSet {}",
            UserId::from(r.user_id).mention(),
            r.message,
            r.created_at.discord_timestamp(TimestampFormat::Relative),
        );

        ChannelId::from(r.channel_id)
            .send_message(&http, CreateMessage::new().content(message))
            .await?;
    }

    Ok(())
}
