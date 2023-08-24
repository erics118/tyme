use anyhow::Result;
use serenity::{client::Context, model::channel::Message};

use crate::{create_message, utils::execute::execute};

/// Execute a shell command and output the result.
pub async fn run(ctx: Context, message: Message) -> Result<()> {
    let cmd = message
        .content
        .split(' ')
        .skip(2)
        .collect::<Vec<_>>()
        .join(" ");

    log::info!("Running shell command: {cmd}");
    let output = execute(cmd)?;

    message
        .channel_id
        .send_message(
            &ctx.http,
            create_message!(
                format!(
                    "```\nstdout:\n{}\n\n\nstderr:\n{}\n\n\n{}```",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr),
                    output.status
                ),
                @ message
            ),
        )
        .await?;

    Ok(())
}
