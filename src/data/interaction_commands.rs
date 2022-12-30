use std::collections::HashMap;

use serenity::{
    builder::CreateApplicationCommand,
    model::application::interaction::application_command::ApplicationCommandInteraction,
    prelude::*,
};

use crate::utils::run::Run;

pub type Register = Box<
    dyn Fn(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand + Send + Sync + 'static,
>;

pub struct InteractionCommand {
    pub run: Run<ApplicationCommandInteraction>,
    pub register: Register,
}

pub struct InteractionCommands;

impl TypeMapKey for InteractionCommands {
    type Value = HashMap<String, InteractionCommand>;
}
