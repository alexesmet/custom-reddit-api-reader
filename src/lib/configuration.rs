
use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub client_id: String,
    pub secret_token: String,
    pub username: String,
    pub password: String
}


pub fn get_configuration() -> Result<Configuration, ConfigError> {
    let config = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?;

    config.try_deserialize()
}
