use crate::config::{app::AppConfig, database::DatabaseConfig, email::EmailConfig};
use dotenv::dotenv;

pub mod app;
pub mod database;
pub mod email;

#[derive(Debug)]
pub struct Config {
    pub app: AppConfig,
    pub database: DatabaseConfig,
    pub email: EmailConfig,
}

impl Config {
    pub fn load() -> Self {
        dotenv().ok();

        Self {
            app: AppConfig::from_env(),
            database: DatabaseConfig::from_env(),
            email: EmailConfig::from_env(),
        }
    }
}
