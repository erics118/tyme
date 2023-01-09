use serenity::prelude::TypeMapKey;
use tokio_postgres::Client as PgClient;

pub struct Database;

impl TypeMapKey for Database {
    type Value = PgClient;
}
