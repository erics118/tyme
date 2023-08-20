use crate::create_interaction_command_only_subcommands;

create_interaction_command_only_subcommands! {
    reminders
    + list "List your upcoming reminders"
    + delete "Delete an upcoming reminder"
}
