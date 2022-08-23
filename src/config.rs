use config::Config;

pub fn get_settings() -> Config {
    Config::builder()
        // Add in `./toke-config.toml`
        .add_source(config::File::with_name("./toke-config.toml"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("TOKE"))

        // Defaults
        .set_default("pidfile", "/var/run/user/$UID/toke.pid").unwrap()
        .build()
        .unwrap()
}
