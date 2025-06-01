use serde::Deserialize;
use std::net::{IpAddr, SocketAddr};
 

use std::env;
use config::{Config, Environment, File};

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: Option<DatabaseConfig>,
    pub logging: Option<LoggingConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: IpAddr,
    pub port: u16,
    pub workers: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
}

impl AppConfig {
    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.server.host, self.server.port)
    }
}


impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            // Базовый конфигурационный файл
            .add_source(File::with_name("config/default"))
            // Конфигурация для текущего режима (development/production)
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            // Локальная конфигурация (не добавляется в репозиторий)
            .add_source(File::with_name("config/local").required(false))
            // Добавляем переменные среды с префиксом "APP"
            .add_source(
                Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;

        config.try_deserialize()
    }
}