use serenity::prelude::TypeMapKey;
use tokio::sync::Mutex;
use tokio_postgres::Client as PostgresClient;

pub struct Database;

impl TypeMapKey for Database {
    type Value = Mutex<PostgresClient>;
}
