use chrono::NaiveDateTime;
use serenity::model::id::{ChannelId, GuildId, UserId};
use sqlx::types::Uuid;

pub struct Reminder {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub time: NaiveDateTime,
    pub message: String,
    pub author_id: UserId,
    pub thread_id: ChannelId,
    pub channel_id: ChannelId,
    pub guild_id: GuildId,
}
