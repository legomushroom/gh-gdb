use clap::StructOpt;
use gh_oocd::CLI;

#[tokio::main]
async fn main() {
    // parse CLI arguments
    let cli = CLI::parse();

    cli
        .run().await
        .unwrap();
}
