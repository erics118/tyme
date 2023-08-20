use crate::create_interaction_command_only_subcommands;

create_interaction_command_only_subcommands! {
    timezone
    + set "Set your default timezone"
    >> String timezone "Timezone to set" required autocomplete
    + get "Get your default timezone"
    + delete "Delete your default timezone"
}
