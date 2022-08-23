extern crate daemonize_me;

use std::any::Any;
use std::fs;
use std::process::exit;
use std::path::Path;

use nix::unistd::Pid;
use nix::sys::signal::{self, Signal};

use super::config;

pub use daemonize_me::Daemon;


fn after_init(_: Option<&dyn Any>) {
    println!("Initialized the daemon!");
    return
}

fn execute() {
    loop {
         std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

pub fn start() {
    let settings = config::get_settings();

    let stdout = fs::File::create("info.log").unwrap();
    let stderr = fs::File::create("err.log").unwrap();
    let pidfile: String = settings.get("pidfile").unwrap();

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
    let settings = config::get_settings();
    let pidfile: String = settings.get("pidfile").unwrap();

    let pid = fs::read_to_string(&pidfile).expect("Unable to read pidfile").parse::<i32>().unwrap();
    
    signal::kill(Pid::from_raw(pid), Signal::SIGTERM).unwrap();

    fs::remove_file(&pidfile).unwrap();
}
