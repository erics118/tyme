use std::process::{Command, Output};

use color_eyre::eyre::{Result, WrapErr};

pub fn execute(cmd: String) -> Result<Output> {
    let output = Command::new("/bin/zsh")
        .arg("-c")
        .arg(cmd)
        .output()
        .context("Failed to execute process")?;
    Ok(output)
}
