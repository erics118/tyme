use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::schema::reminders;

/// Reminder struct.
#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = reminders)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Reminder {
    /// The reminder's id.
    pub id: u64,

    /// The reminder's creation time.
    pub created_at: NaiveDateTime,

    /// The time to notify the user.
    pub time: NaiveDateTime,

    /// The reminder's message.
    pub message: String,

    /// The user's id.
    pub user_id: u64,

    /// The channel's id.
    pub channel_id: u64,

    /// The guild's id.
    /// If `None`, the reminder is a DM reminder.
    pub guild_id: Option<u64>,
}

#[derive(Insertable)]
#[diesel(table_name = reminders)]
pub struct NewReminder<'a> {
    /// The reminder's creation time.
    pub created_at: NaiveDateTime,

    /// The time to notify the user.
    pub time: NaiveDateTime,

    /// The reminder's message.
    pub message: &'a str,

    /// The user's id.
    pub user_id: u64,

    /// The channel's id.
    pub channel_id: u64,

    /// The guild's id.
    /// If `None`, the reminder is a DM reminder.
    pub guild_id: Option<u64>,
}
