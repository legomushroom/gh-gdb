use clap::Parser;
use anyhow::Result;
use std::process::Stdio;
use async_std::process::Command;

mod get_codespace_name;
pub use get_codespace_name::get_codespace_name;

mod gh_login_command;
pub use gh_login_command::gh_login_command;

mod gh_check_command;
pub use gh_check_command::gh_check_command;

mod gh_net_start_command;
pub use gh_net_start_command::gh_net_start_command;

mod start_openocd_command;
pub use start_openocd_command::start_openocd_command;

// openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg -c 'bindto 0.0.0.0'

/// CLI commands and arguments parser
#[derive(Parser, Debug)]
#[clap(name = "gh openocd")]
#[clap(version, about, long_about = None)]
pub struct CLI {
    #[clap(
        short,
        long,
        min_values = 2,
    )]
    pub file: Vec<String>,
}   

/// Start command implementation.
impl CLI {
    /// Start command implementation.
    pub async fn run(&self) -> Result<()> {
        let codespace_name = get_codespace_name().await?;

        // check the environmet setup on the remote side
        Command::new("gh")
            .args(
                &[
                    "codespace",
                    "-c",
                    &codespace_name,
                    "ssh",
                    vec![
                        gh_check_command(),
                        gh_login_command(),
                        // gh_extension_command(self.repo.as_str()),
                        // gh_net_start_remote_command(self.trace.as_str(), is_dns),
                    ].join(" && ").as_str(),
                ],
            )
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?
            .output().await?;

        // run `gh net` and `openocd` in parallel
        let (result1, result2) = tokio::try_join!(
            tokio::spawn(gh_net_start_command()),
            tokio::spawn(
                start_openocd_command(
                    self.file[0].clone(),
                    self.file[1].clone(),
                ),
            ),
        )?;

        result1?;
        result2?;

        return Ok(());
    }
}
