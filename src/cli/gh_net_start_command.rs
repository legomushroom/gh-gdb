use std::str;
use std::process::Stdio;
use anyhow::{Result, bail};
use async_std::process::Command;

/// Check if user is logged in into GitHub CLI as
/// root user, and if not, log in.
pub async fn gh_net_start_command() -> Result<()> {
    let cmd = futures_lite::future::block_on(async move {
        let cmd = Command::new("gh")
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
            .kill_on_drop(true)
            .spawn();

        cmd
    })?;

    let output = cmd.output().await?;

    let output_str = str::from_utf8(&output.stderr)?.to_string();
    if output_str.len() > 0 {
        bail!(output_str);
    }

    return Ok(());
}
