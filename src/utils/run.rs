use std::{future::Future, pin::Pin};

use anyhow::Result;
use serenity::client::Context;
use tokio_postgres::Client as DbClient;
pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + Sync + 'static>>;

pub type Run<T> =
    Box<dyn Fn(Context, T, DbClient) -> BoxFuture<Result<()>> + Send + Sync + 'static>;

pub fn wrap_cmd<T, F>(f: fn(Context, T, DbClient) -> F) -> Run<T>
where
    T: 'static,
    F: Future<Output = Result<()>> + Send + Sync + 'static,
{
    Box::new(move |ctx, command, db| Box::pin(f(ctx, command, db)))
}
