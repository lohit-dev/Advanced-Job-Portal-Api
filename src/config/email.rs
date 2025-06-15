use std::env;

#[derive(Debug, Clone)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_from: String,
}

impl EmailConfig {
    pub fn from_env() -> Self {
        Self {
            smtp_host: env::var("SMTP_HOST").expect("SMTP_HOST is missing"),
            smtp_port: env::var("SMTP_PORT")
                .expect("SMTP_PORT is missing")
                .parse()
                .expect("SMTP_PORT must be a number"),
            smtp_username: env::var("SMTP_USERNAME").expect("SMTP_USERNAME is missing"),
            smtp_password: env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD is missing"),
            smtp_from: env::var("SMTP_FROM").expect("SMTP_FROM is missing"),
        }
    }
}
