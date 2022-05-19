use anyhow::Result;
use std::process::Stdio;
use async_std::process::Command;

/// Check if user is logged in into GitHub CLI as
/// root user, and if not, log in.
pub async fn gh_net_start_command() -> Result<()> {
    Command::new("gh")
        .args(
            &[
                "net",
                "start",
                "--gui",
                "false",
            ],
        )
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;
        // .output().await?;

    return Ok(());
}
