use std::collections::HashMap;

use serenity::{model::channel::Message, prelude::TypeMapKey};

use crate::utils::run::Run;

pub struct MessageCommand {
    pub run: Run<Message>,
}

pub struct MessageCommands;

impl TypeMapKey for MessageCommands {
    type Value = HashMap<String, MessageCommand>;
}
