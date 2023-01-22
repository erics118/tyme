use std::{collections::HashMap, sync::Arc};

use serenity::{model::channel::Message, prelude::TypeMapKey};
use tokio::sync::{Mutex, RwLock};
use tokio_postgres::Client as DbClient;

pub struct Database;

impl TypeMapKey for Database {
    type Value = Mutex<Arc<RwLock<DbClient>>>;
}
