use std::{future::Future, pin::Pin};

use color_eyre::eyre::Result;
use serenity::client::Context;

pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + Sync + 'static>>;

pub type Run<T> = Box<dyn for<'a> Fn(Context, T) -> BoxFuture<Result<()>> + Send + Sync + 'static>;

pub fn wrap_cmd<T, F>(f: for<'a> fn(Context, T) -> F) -> Run<T>
where
    T: 'static,
    F: Future<Output = Result<()>> + Send + Sync + 'static,
{
    Box::new(move |ctx, command| Box::pin(f(ctx, command)))
}
