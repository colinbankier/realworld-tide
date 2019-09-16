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

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("app"))?;

        // Deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}
