use crate::interaction_commands;

interaction_commands!(test, remind, reminders, timezone);

mod reminders_delete;
mod reminders_list;

mod timezone_delete;
mod timezone_get;
mod timezone_set;
