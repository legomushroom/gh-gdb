use std::{fs, str, process::Stdio, path::Path};

use anyhow::{Result, bail};

use async_std::process::Command;

/// Function to get Codespace name from GitHub CLI by
/// reusing the `select` command.
pub async fn get_codespace_name() -> Result<String> {
    let file_path = match Path::new("/tmp/selected_codespace").as_os_str().to_str() {
        Some(s) => s,
        None => bail!("Cannot create selected codespace file."),
    };

    let output = Command::new("gh")
        .args(
            &[
                "codespace",
                "select",
                "-f",
                file_path,
            ],
        )
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::piped())
        .output().await?;

    let output_str = str::from_utf8(&output.stderr)?.to_string();
    if output_str.len() > 0 {
        let message = if output_str.contains("unknown command") {
            "GitHub CLI is too old, please update to at least v2.8.0 (Apr 13 2022) and try again. https://github.com/cli/cli#installation".to_string()
        } else {
            output_str
        };

        bail!(message);
    }

    if !Path::new(file_path).is_file() {
        bail!("No codespace selected.");
    };

    let codespace_name = fs::read_to_string(file_path)?;
    if codespace_name.len() == 0 {
        bail!("No Codespace selected (output file is empty).");
    }

    // try to cleanup the file
    let _res = fs::remove_file(file_path);

    return Ok(codespace_name);
}
