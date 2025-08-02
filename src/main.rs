mod config;
mod core;
mod http;

use clap::Command;
use config::Config;
use config::Cli;
use clap::Parser;

#[tokio::main]
async fn main() {
    // let cli = Cli::parse();
    // match cli.command {
    //     Command::Run { file, daemon } => {
    //         if daemon {
    //         
    //         } else {
    //         
    //         }
    //     }
    //     Command::Stop => 
    // }

    let config = Config::from_file("oxide.yaml").unwrap();
}
