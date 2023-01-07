use std::{future::Future, pin::Pin};

use anyhow::Result;
use serenity::client::Context;

pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + Sync + 'static>>;

pub type Run<T> = Box<dyn Fn(Context, T) -> BoxFuture<Result<()>> + Send + Sync + 'static>;

pub fn wrap_cmd<T: 'static, F>(f: fn(Context, T) -> F) -> Run<T>
where
    F: Future<Output = Result<()>> + Send + Sync + 'static,
{
    Box::new(move |ctx, command| Box::pin(f(ctx, command)))
}
