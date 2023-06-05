use serenity::prelude::TypeMapKey;
use tyme_db::MySqlPool;

#[derive(Debug)]
pub struct Database;

impl TypeMapKey for Database {
    type Value = MySqlPool;
}