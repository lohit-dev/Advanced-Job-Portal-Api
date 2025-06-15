use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub jwt_secret: String,
    pub jwt_maxage: i64,
    pub port: u16,
    // pub max_password_length: usize,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET is missing"),
            jwt_maxage: env::var("JWT_MAXAGE")
                .expect("JWT_MAXAGE is missing")
                .parse()
                .expect("JWT_MAXAGE must be a number"),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT must be a number"),
            //     max_password_length: env::var("MAX_PASSWORD_LENGTH")
            //         .expect("MAX_PASSWORD_LENGTH is missing")
            //         .parse()
            //         .expect("MAX_PASSWORD_LENGTH must be a number"),
        }
    }
}
