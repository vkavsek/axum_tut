use crate::{Error, Result};
use core::panic;
use std::{env, sync::OnceLock};

/// Tries to create a config from enviroment variables declared in .cargo/config.toml file.
pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        Config::load_from_env()
            .unwrap_or_else(|er| panic!("FATAL - while loading config - Cause: {er:?}"))
    })
}

#[allow(non_snake_case)]
pub struct Config {
    // Db
    pub DB_URL: String,
    // Web
    pub WEB_FOLDER: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            DB_URL: get_env("SERVICE_DB_URL")?,
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}
