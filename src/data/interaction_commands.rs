use std::collections::HashMap;

use serenity::{
    model::application::interaction::application_command::ApplicationCommandInteraction, prelude::*,
};

use crate::utils::run::Run;

pub struct InteractionCommand {
    pub run: Run<ApplicationCommandInteraction>,
}

pub struct InteractionCommands;

impl TypeMapKey for InteractionCommands {
    type Value = HashMap<String, InteractionCommand>;
}
