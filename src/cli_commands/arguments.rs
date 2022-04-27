use clap::{Parser, Subcommand};

use super::commands::StartCommand;

// mod commands;
// use commands::StartCommand;

/// CLI commands and arguments parser
#[derive(Parser, Debug)]
#[clap(name = "gh tun")]
#[clap(version, about, long_about = None)]
pub struct CLI {
    /// Extension CLI commands.
    #[clap(subcommand)]
    pub command: Commands,
}

/// All supported CLI commands.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Starts VPN Gateway. Example: `$ gh net start`
    Start(StartCommand),
    /// `Hidden` exists command can be used to check
    /// if `gh net` extension is installed. Simply
    /// outputs "yes" into the stdoud and exits.
    #[clap(hide = true)]
    Exists,
}
