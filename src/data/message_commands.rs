use std::collections::HashMap;

use serenity::{model::channel::Message, prelude::TypeMapKey};

use crate::data::run::Run;

pub struct MessageCommands;

impl TypeMapKey for MessageCommands {
    type Value = HashMap<String, Run<Message>>;
}
