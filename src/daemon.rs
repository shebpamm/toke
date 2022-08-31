extern crate daemonize_me;

use std::any::Any;
use std::fs;
use std::process::exit;
use std::path::Path;
use std::env;
use std::thread;
use std::sync::{Arc, Mutex};

use std::os::unix::net::UnixListener;
use std::os::unix::net::UnixStream;
use std::io::Write;

use std::time::{SystemTime, UNIX_EPOCH, Duration};

use nix::unistd::Pid;
use nix::sys::signal::{self, Signal};

use super::config;
use super::secrets;

pub use daemonize_me::Daemon;



fn after_init(_: Option<&dyn Any>) {
    println!("Initialized the daemon!");
    return
}

fn token_from_ldap() -> String {
    return "asd".to_string();
}

fn token_valid(token: &str) -> bool {
    return true; 
}

fn handle_client(mut stream: UnixStream, token_mutex: Arc<Mutex<String>>) {
    let token = token_mutex.lock().unwrap();

    stream.write_all(*&token.as_bytes());
}

fn refresh_token_loop(token_mutex: Arc<Mutex<String>>) {
    loop {
        thread::sleep(Duration::from_millis(1000));

        let mut token = token_mutex.lock().unwrap();

        *token = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();

    }
}

fn execute<'a>() {
    let settings = config::get_settings().unwrap();

    let current_token: String = env::vars().find(|(key, _value)| key == "VAULT_TOKEN").map_or(token_from_ldap(), |var| var.1);

    let token_mutex = Arc::new(Mutex::new(current_token));

    let refresher_token_mutex = Arc::clone(&token_mutex);

    let refresher = thread::spawn(move || refresh_token_loop(refresher_token_mutex));

    let socket_path: String = settings.get("socketfile").unwrap();

    let listener = UnixListener::bind(socket_path).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                /* connection succeeded */
                let handler_token_mutex = Arc::clone(&token_mutex);
                thread::spawn(|| handle_client(stream, handler_token_mutex));
            }
            Err(_err) => {
                /* connection failed */
                break;
            }
        }
    }
}

pub fn start() {
    
    if !secrets::credentials_present() {
        println!("Credentials not present! Please configure them first.");
        return;
    }

    let settings = config::get_settings().unwrap();

    let stdout_path: String = settings.get("stdout").unwrap();
    let stderr_path: String = settings.get("stderr").unwrap();
    let pidfile: String = settings.get("pidfile").unwrap();

    let stdout = fs::File::create(stdout_path).unwrap();
    let stderr = fs::File::create(stderr_path).unwrap();

    if Path::new(&pidfile).exists() {
        println!("Daemon already running!");
        return
    }

    let daemon = Daemon::new()
        .pid_file(pidfile, Some(false))
        .umask(0o000)
        .work_dir(".")
        .stdout(stdout)
        .stderr(stderr)
        .setup_post_init_hook(after_init, None)
        .start();

    match daemon {
        Ok(_) => println!("Daemonized with success"),
        Err(e) => {
            eprintln!("Error, {}", e);
            exit(-1);
        },
    }

    execute();
}

pub fn stop() {
    let settings = config::get_settings().unwrap();
    let pidfile: String = settings.get("pidfile").unwrap();
    let socket_path: String = settings.get("socketfile").unwrap();

    let pid = fs::read_to_string(&pidfile).expect("Unable to read pidfile").parse::<i32>().unwrap();
    
    signal::kill(Pid::from_raw(pid), Signal::SIGTERM).unwrap();

    fs::remove_file(&pidfile).unwrap();
    fs::remove_file(&socket_path).unwrap();
}
