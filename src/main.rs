use clap::StructOpt;
use gh_tun::CLI;

#[tokio::main]
async fn main() {
    // parse CLI arguments
    let cli = CLI::parse();

    println!("cli: {:?}", cli);

    // // run appropriate commands
    // match args.command {
    //     Commands::Start(start_command) => {
    //         // output error, if present
    //         if let Err(error) = start_command.run().await {
    //             eprintln!("\n{}", error);

    //             process::exit(1);
    //         }

    //         process::exit(0);
    //     },
    //     Commands::Exists => {
    //         print!("{}", EXISTS_COMMAND_OUTPUT);
    //     },
    // }
}
