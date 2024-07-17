use crate::commands;

commands! {
    help,
    register,
    remind,
    reminders[delete, list],
    shutdown,
    timezone[delete, get, set],
}
