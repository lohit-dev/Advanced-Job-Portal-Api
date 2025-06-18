use crate::config::{
    app::AppConfig, database::DatabaseConfig, email::EmailConfig, oauth::OAuthConfig,
};
use dotenv::dotenv;

pub mod app;
pub mod database;
pub mod email;
pub mod oauth;

#[derive(Debug)]
pub struct Config {
    pub app: AppConfig,
    pub database: DatabaseConfig,
    pub email: EmailConfig,
    pub oauth: OAuthConfig,
}

impl Config {
    pub fn load() -> Self {
        dotenv().ok();

        Self {
            app: AppConfig::from_env(),
            database: DatabaseConfig::from_env(),
            email: EmailConfig::from_env(),
            oauth: OAuthConfig::from_env(),
        }
    }
}
