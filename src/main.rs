mod config;
mod core;
mod http;

use config::Command;
use config::Config;
use config::Cli;
use clap::Parser;
use core::Server;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Run { file, daemon } => {
            let config = Config::from_file(file).unwrap();

            if daemon {
                // TODO: implement daemon mode
                eprintln!("Daemon mode not implemented yet");
            } else {
                let server = Server::new(config).await.unwrap();
                let _ = server.run().await;
            }
        }

        Command::Check { file } => {
            match Config::from_file(file) {
                Ok(_) => println!("Configuration is valid."),
                Err(e) => {
                    eprintln!("Configuration error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Command::Stop => {
            // TODO: send stop signal to daemon
            eprintln!("Stop command not implemented yet.");
        }

        Command::Reload => {
            // TODO: send reload signal to daemon
            eprintln!("Reload command not implemented yet.");
        }

        Command::Logs => {
            // TODO: read and display recent logs
            eprintln!("Logs command not implemented yet.");
        }
    }
}

