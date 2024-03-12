use lib_utils::envs::get_env;

use core::panic;
use std::sync::OnceLock;

/// Tries to create a config from enviroment variables declared in .cargo/config.toml file.
pub fn web_config() -> &'static WebConfig {
    static INSTANCE: OnceLock<WebConfig> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        WebConfig::load_from_env()
            .unwrap_or_else(|er| panic!("FATAL - while loading config - Cause: {er:?}"))
    })
}

#[allow(non_snake_case)]
pub struct WebConfig {
    pub WEB_FOLDER: String,
}

impl WebConfig {
    fn load_from_env() -> lib_utils::envs::Result<WebConfig> {
        Ok(WebConfig {
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
        })
    }
}
