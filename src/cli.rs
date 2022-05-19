use clap::Parser;
use anyhow::Result;

mod gh_net_start_command;
pub use gh_net_start_command::gh_net_start_command;

mod start_openocd_command;
pub use start_openocd_command::start_openocd_command;

#[derive(Parser, Debug)]
#[clap(name = "gh gdb")]
#[clap(version, about, long_about = None)]
pub struct CLI {
    /// OpenOCD configuration file path.
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
