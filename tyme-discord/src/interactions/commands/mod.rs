//! Interaction commands.

use crate::interaction_commands;

interaction_commands! {
    test,
    remind,
    reminders[list, delete],
    timezone[get, set, delete],
}
