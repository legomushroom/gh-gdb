use clap::Args;
use anyhow::Result;

/// Start command definition.
#[derive(Args, Debug)]
#[clap(name = "gh tun")]
pub struct StartCommand {
    /// Location of the app, e.g. on local or remote machine.
    /// Defaults to "local".
    #[clap(
        short,
        long,
        max_values = 1,
        possible_values = &["local", "remote"],
        default_value = "local",
    )]
    location: String,

    #[clap(
        name = "local-devname",
        short,
        long,
        max_values = 1,
        default_value = "any",
    )]
    local_dev_name: String,

    #[clap(
        name = "remove-devname",
        short,
        long,
        max_values = 1,
        default_value = "any",
    )]
    remote_dev_name: String,

    /// Use 100.64.0.0/10 subnet by default (from 100.64.0.0 to 100.127.255.255).
    /// This is called the “Carrier Grade NAT” (CGNAT) address space, reserved by RFC 6598,
    /// IANA-Reserved IPv4 Prefix for Shared Address Space.
    /// 
    /// https://datatracker.ietf.org/doc/html/rfc6598
    #[clap(
        short,
        long,
        max_values = 1,
        default_value = "100.64.0.0/10",
    )]
    network: String,

    #[clap(
        name = "local-ip",
        short,
        long,
        max_values = 1,
        default_value = "auto",
    )]
    local_ip: String,

    #[clap(
        name = "remote-ip",
        short,
        long,
        max_values = 1,
        default_value = "auto",
    )]
    remote_ip: String,
}

/// Start command implementation.
impl StartCommand {
    /// Start command implementation.
    pub async fn run(&self) -> Result<()> {
        return Ok(());
    }
}
