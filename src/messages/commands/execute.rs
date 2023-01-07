use anyhow::Result;
use serenity::{client::Context, model::channel::Message};

use crate::utils::execute::execute;

pub static NAME: &str = "execute";

pub async fn run(ctx: Context, message: Message) -> Result<()> {
    let cmd = message
        .content
        .split(' ')
        .skip(2)
        .collect::<Vec<&str>>()
        .join(" ");
    log::info!("Running shell command: {cmd}");
    let output = execute(cmd)?;

    message
        .channel_id
        .send_message(&ctx.http, |msg| {
            msg.content(format!(
                "```\nstdout:\n{}\n\n\nstderr:\n{}\n\n\n{}```",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr),
                output.status
            ))
        })
        .await?;

    Ok(())
}
