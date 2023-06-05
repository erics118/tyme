//! Database for use in serenity's TypeMap.

use serenity::prelude::TypeMapKey;
use tyme_db::MySqlPool;

/// Database struct for use in serenity's TypeMap.
#[derive(Debug, Copy, Clone)]
pub struct Database;

impl TypeMapKey for Database {
    type Value = MySqlPool;
}
