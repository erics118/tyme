use std::process::{Command, Output};

use anyhow::{Context, Result};

pub fn execute(cmd: String) -> Result<Output> {
    let output = Command::new("/bin/zsh")
        .arg("-c")
        .arg(cmd)
        .output()
        .context("Failed to execute process")?;
    Ok(output)
}
