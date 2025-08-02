use std::path::PathBuf;
use clap::{Parser, Subcommand};

/// CLI for the Oxide web server
#[derive(Parser)]
#[command(name = "oxide", version = "0.1", author = "Pawelgit1234")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// Supported commands for the `oxide` CLI
#[derive(Subcommand)]
pub enum Command {
    /// Runs the server
    Run {
        /// Path to the configuration file
        #[arg(short, long)]
        file: PathBuf,

        /// Run the server as a daemon in the background
        #[arg(short = 'd', long)]
        daemon: bool,
    },

    /// Validates the configuration file without starting the server
    Check {
        /// Path to the configuration file
        #[arg(short, long)]
        file: PathBuf,
    },

    /// Stops the running daemon
    Stop,

    /// Reloads the configuration file without stopping the server
    Reload,

    /// Displays the most recent log entries
    Logs,
}
