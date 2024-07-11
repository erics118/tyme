use crate::create_command;

create_command! {
    / reminders
        + list "List your upcoming reminders"
        + delete "Delete an upcoming reminder"
            > String reminder_id "Reminder to delete" required autocomplete
}
