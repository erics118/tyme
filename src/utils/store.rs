macro_rules! store_interaction_command {
    ($map:ident, $cmd:tt) => {
        $map.insert(
            interactions::commands::$cmd::NAME.to_string(),
            InteractionCommand {
                run: wrap_cmd::<ApplicationCommandInteraction, _>(
                    interactions::commands::$cmd::run,
                ),
                register: Box::new(interactions::commands::$cmd::register),
            },
        )
    };
}

macro_rules! store_message_command {
    ($map:ident, $cmd:tt) => {
        $map.insert(
            messages::commands::$cmd::NAME.to_string(),
            MessageCommand {
                run: wrap_cmd::<Message, _>(messages::commands::$cmd::run),
            },
        )
    };
}

pub(crate) use store_interaction_command;
pub(crate) use store_message_command;
