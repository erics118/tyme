use std::{fmt::Display, future::Future};

use color_eyre::eyre::{Result, WrapErr};

#[allow(dead_code)]
pub async fn catch_context<F>(context: &'static str, future: F)
where
    F: Future<Output = Result<()>> + Send,
{
    if let Err(err) = future.await.context(context) {
        log::error!("{:?}", err);
    }
}

#[allow(dead_code)]
pub async fn catch_with_context<O, C, F>(context: C, future: F)
where
    O: Display + Send + Sync + 'static,
    C: FnOnce() -> O + Send,
    F: Future<Output = Result<()>> + Send,
{
    if let Err(err) = future.await.with_context(context) {
        log::error!("{:?}", err);
    }
}

#[allow(dead_code)]
pub async fn catch<F>(future: F)
where
    F: Future<Output = Result<()>> + Send,
{
    if let Err(err) = future.await {
        log::error!("{:?}", err);
    }
}
