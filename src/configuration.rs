use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Application {
    pub host: String ,
    pub port: usize,
}

#[derive(Debug, Deserialize)]
pub struct Postgres {
    pub host: String,
    pub port: usize,
    pub username: String,
    pub password: String,
    pub db: String,
}

impl Postgres {
    pub fn connection_string(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}", self.username, self.password, self.host, self.port, self.db)
    }
}

#[derive(Debug, Deserialize)]
pub struct Authentication {
    pub secret: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub application: Application,
    pub database: Postgres,
    pub authentication: Authentication,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("appsettings"))?;

        // Add in settings from the environment (with a prefix of APP and '_' as separator)
        // Eg.. `APP_APPLICATION_PORT=5001 would set `Settings.application.port`
        s.merge(Environment::with_prefix("app").separator("_"))?;

        // Deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}
