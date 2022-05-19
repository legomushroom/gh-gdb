use std::str;
use anyhow::{Result, bail};
use std::process::Stdio;
use async_std::process::Command;

pub async fn start_openocd_command(
    config1: String,
    config2: String,
) -> Result<()> {
    let output = Command::new("openocd")
        .args(
            &[
                "-f",
                config1.as_str(),
                "-f",
                config2.as_str(),
                "-c",
                "bindto 0.0.0.0"
            ],
        )
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()?
        .output().await?;

    let output_str = str::from_utf8(&output.stderr)?.to_string();
    if output_str.len() > 0 {
        bail!(output_str);
    }

    return Ok(());
}
