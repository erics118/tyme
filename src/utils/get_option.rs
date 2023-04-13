use serenity::model::application::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};

pub trait GetOption {
    fn get_option(
        &self,
        command: ApplicationCommandInteraction,
        i: usize,
    ) -> Option<&CommandDataOptionValue>;
}

impl GetOption for ApplicationCommandInteraction {
    fn get_option(
        &self,
        _command: ApplicationCommandInteraction,
        _i: usize,
    ) -> Option<&CommandDataOptionValue> {
        // match command
        //     .data
        //     .options
        //     .get(i)
        //     .expect("Expected days option")
        //     .resolved
        //     .as_ref()
        // {
        //     Some(value) => Some(value),
        //     None => None,
        // }
        todo!()
    }
}
