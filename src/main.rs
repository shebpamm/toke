use clap::{Args, Parser, Subcommand};
use strum_macros::EnumString;

use std::io::prelude::*;
use std::net::Shutdown;
use std::os::unix::net::UnixStream;
use std::path::Path;

mod config;
mod daemon;
mod secrets;

/// Vault token manager
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CLI {
    /// Subcommand
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Args)]
struct Auth {
    #[clap(subcommand)]
    command: Option<AuthCommands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Get cached token
    Token {
        /// Read/Refresh
        #[clap(default_value = "read")]
        command: TokenCommands,
    },
    Daemon {
        /// start/stop/restart
        #[clap(default_value = "start")]
        state: DaemonStates,
    },

    /// Authentication & Secrets
    Auth(Auth),
}

#[derive(Debug, EnumString, Subcommand)]
enum AuthCommands {
    #[strum(ascii_case_insensitive)]
    Read,

    #[strum(ascii_case_insensitive)]
    #[clap(arg_required_else_help = true)]
    Store {
        username: Option<String>,

        password: Option<String>,
    },
}

#[derive(Debug, EnumString)]
enum TokenCommands {
    #[strum(ascii_case_insensitive)]
    Read,

    #[strum(ascii_case_insensitive)]
    Refresh,
}

#[derive(Debug, EnumString)]
enum DaemonStates {
    #[strum(ascii_case_insensitive)]
    Start,

    #[strum(ascii_case_insensitive)]
    Stop,

    #[strum(ascii_case_insensitive)]
    Restart,
}

fn manage_token(command: TokenCommands) {
    match command {
        TokenCommands::Read => give_token(),
        TokenCommands::Refresh => refresh_token(),
    }
}

fn manage_daemon(state: DaemonStates) {
    match state {
        DaemonStates::Start => daemon::start(),
        DaemonStates::Stop => daemon::stop(),
        DaemonStates::Restart => println!("3"),
    }
}

fn manage_auth(auth: Auth) {
    let auth_command = auth.command.unwrap_or(AuthCommands::Read);
    match auth_command {
        AuthCommands::Read => {
            if secrets::credentials_present() {
                let (username, password) =
                    secrets::get_login_credentials().expect("Failed to retrieve credentials");

                println!("Username: {}\nPassword: {}", username, password);
            } else {
                println!("Credentials missing!")
            }
        }

        AuthCommands::Store { username, password } => {
            secrets::set_login_credentials(&username.unwrap(), &password.unwrap())
        }
    }
}

fn give_token() {
    let response = write_to_socket("read");

    println!("{response}");
}

fn refresh_token() {
    let response = write_to_socket("refresh");

    println!("{response}");
}

fn write_to_socket(command: &str) -> String {
    let settings = config::get_settings().unwrap();
    let socket_path: String = settings.get("socketfile").unwrap();

    if !Path::new(&socket_path).exists() {
        eprintln!("Daemon not running!");
        return "".to_owned();
    }

    let mut stream = UnixStream::connect(&socket_path).unwrap();

    match stream.write_all(command.as_bytes()) {
        Ok(_) => {
            stream.shutdown(Shutdown::Write).expect("shutdown failed");

            let mut response = String::new();
            stream.read_to_string(&mut response).unwrap();

            return response;
        }
        Err(_) => return "".to_owned(),
    }
}

fn main() {
    let args = CLI::parse();

    match args.command {
        Commands::Token { command } => manage_token(command),

        Commands::Daemon { state } => manage_daemon(state),

        Commands::Auth(auth) => manage_auth(auth),
    }
}
