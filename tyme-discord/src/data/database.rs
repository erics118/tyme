use std::sync::Arc;

use serenity::prelude::TypeMapKey;
use tokio::sync::Mutex;
use tyme_db::MySqlPool;

#[derive(Debug)]
pub struct Database;

impl TypeMapKey for Database {
    type Value = Arc<Mutex<MySqlPool>>;
}
