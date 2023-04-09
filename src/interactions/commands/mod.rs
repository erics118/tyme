pub mod test;

use color_eyre::eyre::Result;
use serenity::{
    builder::CreateApplicationCommands, client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

use crate::interaction_commands;
interaction_commands!(test);
