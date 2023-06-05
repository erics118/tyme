use anyhow::Result;
use serenity::{builder::CreateMessage, client::Context, model::channel::Message};

use crate::utils::execute::execute;

/// Execute a shell command and output the result.
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
        .send_message(
            &ctx.http,
            CreateMessage::new()
                .content(format!(
                    "```\nstdout:\n{}\n\n\nstderr:\n{}\n\n\n{}```",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr),
                    output.status
                ))
                .reference_message(&message),
        )
        .await?;

    Ok(())
}
