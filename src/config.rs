//! Inject dotenv and env variables into the Config struct
//!
//! The envy crate injects environment variables into a struct.
//!
//! dotenv allows environment variables to be augmented/overwriten by a
//! .env file.
//!
//! This file throws the Config struct into a CONFIG lazy_static to avoid
//! multiple processing.

use crate::error::Result;
use dotenv::dotenv;
use lazy_static::lazy_static;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub(crate) client: String,
    pub(crate) server: String,
}

// Throw the Config struct into a CONFIG lazy_static to avoid multiple processing
lazy_static! {
    pub static ref CONFIG: Config = config().unwrap_or_else(|error| panic!("{}", error));
}

/// Use envy to inject dotenv and env vars into the Config struct
pub fn config() -> Result<Config> {
    dotenv().ok();
    Ok(envy::from_env::<Config>()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_the_config() {
        assert_ne!(config().unwrap().server, "".to_string());
        assert_ne!(*CONFIG.server, "".to_string());
    }
}
