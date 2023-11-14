use config::{File, FileFormat};
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(Deserialize, Default, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut builder = config::Config::builder();
    builder = builder.add_source(File::new("configuration.yaml", FileFormat::Yaml));

    match builder.build() {
        Ok(config) => config.try_deserialize(),
        Err(e) => Err(e),
    }
}
