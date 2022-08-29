use clap::{Parser, Subcommand, Args};
use strum_macros::EnumString;

mod daemon;
mod config;
mod secrets;

/// Simple program to greet a person
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

        password: Option<String>
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

fn manage_auth(auth: Auth) {

    let auth_command = auth.command.unwrap_or(AuthCommands::Read);
    match auth_command {
        AuthCommands::Read => {
            if secrets::credentials_present() {
                let (username, password) = secrets::get_login_credentials().expect("Failed to retrieve credentials");

                println!("Username: {}\nPassword: {}", username, password);
            } else {
                println!("Credentials missing!")
            }   
        },

        AuthCommands::Store { username, password } => {
            secrets::set_login_credentials(&username.unwrap(), &password.unwrap())
        }

    }

}

fn main() {
    let args = CLI::parse();

    match args.command {
        Commands::Env { command } => {
            println!("{}", command);
        },

        Commands::Daemon { state } => manage_daemon(state),

        Commands::Auth ( auth ) => manage_auth(auth),
    }
}
