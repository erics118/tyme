use serenity::prelude::TypeMapKey;
use sqlx::{Pool, Postgres};
use tokio::sync::Mutex;

pub struct Database;

impl TypeMapKey for Database {
    type Value = Mutex<Pool<Postgres>>;
}
