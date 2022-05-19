use clap::Parser;
use anyhow::Result;

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

    #[clap(
        short,
        long,
    )]
    pub command: Option<String>,
}   

/// Start command implementation.
impl CLI {
    /// Start command implementation.
    pub async fn run(&self) -> Result<()> {
        return Ok(());
    }
}
