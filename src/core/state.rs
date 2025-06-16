use sqlx::PgPool;

use crate::config::{Config, database::init_db};

pub struct AppState {
    pub db: PgPool,
    pub config: Config,
}

pub async fn build_state(config: Config) -> AppState {
    let db = init_db(&config.database.database_url).await.unwrap();
    AppState { db, config }
}
