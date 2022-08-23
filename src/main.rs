use clap::{Parser, Subcommand};
use strum_macros::EnumString;

mod daemon;
mod config;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CLI {
    /// Subcommand
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Shell Env Commands
    #[clap(arg_required_else_help = true)]
    Env {
        /// The remote to clone
        command: String,
    },
    /// Manage Daemon
    Daemon {
        /// start/stop/restart
        #[clap(default_value = "start")]
        state: DaemonStates,
    }
}

#[derive(Debug, EnumString)]
enum DaemonStates {
    #[strum(ascii_case_insensitive)]
    Start, 

    #[strum(ascii_case_insensitive)]
    Stop,

    #[strum(ascii_case_insensitive)]
    Restart
}

fn manage_daemon(state: DaemonStates) {
    match state {
        DaemonStates::Start => daemon::start(),
        DaemonStates::Stop => daemon::stop(),
        DaemonStates::Restart => println!("3")
    }
}

fn main() {
    let args = CLI::parse();

    match args.command {
        Commands::Env { command } => {
            println!("{}", command);
        }
        Commands::Daemon { state } => manage_daemon(state)
    }
}
