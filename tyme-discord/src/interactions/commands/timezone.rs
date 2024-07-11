use crate::create_command;

create_command! {
    / timezone
        + set "Set your default timezone"
            > String timezone "Timezone to set" required autocomplete
        + get "Get your default timezone"
        + delete "Delete your default timezone"
}
