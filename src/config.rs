use config::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub database_dsn: String,
    pub http_port: u16,
    pub log_output: String, // "console", "file", or "both"
}

pub fn load_config() -> AppConfig {
    let settings = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .unwrap();
    settings.try_deserialize::<AppConfig>().unwrap()
}
