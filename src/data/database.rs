use std::sync::Arc;

use serenity::prelude::TypeMapKey;
use tokio::sync::{Mutex, RwLock};
use tokio_postgres::Client as DbClient;

pub struct Database;

impl TypeMapKey for Database {
    type Value = Mutex<Arc<RwLock<DbClient>>>;
}
