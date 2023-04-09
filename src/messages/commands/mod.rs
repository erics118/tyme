pub mod db;
pub mod execute;
pub mod register;
pub mod shutdown;

use color_eyre::eyre::Result;
use serenity::{client::Context, model::prelude::Message};

use crate::message_commands;

message_commands!(execute, register, shutdown, db);
