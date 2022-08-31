use config::{Config,ConfigError};
use xdg;
use std::fs;

pub fn get_settings() -> Result<Config, ConfigError> {

    let xdg_dirs = xdg::BaseDirectories::with_prefix("toke").unwrap();
    let pid_path = xdg_dirs.place_runtime_file("toke.pid")
                        .expect("cannot create pid directory");
    let socket_path = xdg_dirs.place_runtime_file("toke.socket")
                        .expect("cannot create socket directory");
    let config_path = xdg_dirs.place_config_file("config.toml")
                        .expect("cannot create configuration directory");
    let stdout_path = xdg_dirs.place_state_file("stdout.log")
                        .expect("cannot create stdout log directory");
    let stderr_path = xdg_dirs.place_state_file("stderr.log")
                        .expect("cannot create stderr log directory");

    // Make sure file exists
    fs::OpenOptions::new()
        .write(true)
        .truncate(false)
        .create(true)
        .open(&config_path)
        .unwrap();

    Config::builder()
        // Add in `./toke-config.toml`
        .add_source(config::File::with_name(&config_path.into_os_string().into_string().unwrap()))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("TOKE"))

        // Defaults
        .set_default("pidfile", pid_path.into_os_string().into_string().unwrap())?
        .set_default("socketfile", socket_path.into_os_string().into_string().unwrap())?
        .set_default("stdout",  stdout_path .into_os_string().into_string().unwrap())?
        .set_default("stderr",  stderr_path .into_os_string().into_string().unwrap())?
        .build()
}
