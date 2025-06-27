use sqlx::PgPool;

use crate::{
    config::{Config, database::init_db},
    features::{
        auth::service::AuthService, skills::service::SkillService, users::service::UserService,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Config,
    pub user_service: UserService,
    pub auth_service: AuthService,
    pub skill_service: SkillService,
}

pub async fn build_state(config: Config) -> AppState {
    let db = init_db(&config.database.database_url).await.unwrap();
    let user_service = UserService { db: db.clone() };
    let auth_service = AuthService;
    let skill_service = SkillService { db: db.clone() };

    AppState {
        db,
        config,
        user_service,
        auth_service,
        skill_service,
    }
}
