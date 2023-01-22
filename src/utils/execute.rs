use std::process::{Command, Output};

use color_eyre::eyre::{WrapErr, Result};

pub fn execute(cmd: String) -> Result<Output> {
    let output = Command::new("/usr/bin/zsh")
        .arg("-c")
        .arg(cmd)
        .output()
        .context("Failed to execute process")?;
    Ok(output)
}
