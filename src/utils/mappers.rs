use crate::features::{
    auth::model::AuthProvider,
    skills::{dto::SkillResponseDto, model::Skill},
    users::model::{User, UserRole},
};
use chrono::{DateTime, Utc};
use sqlx::{Row, postgres::PgRow};
use uuid::Uuid;

pub fn map_row_to_user(row: &PgRow) -> User {
    User {
        id: row.get::<Uuid, _>("id"),
        name: row.get::<String, _>("name"),
        email: row.get::<String, _>("email"),
        password: row.get::<String, _>("password"),
        verified: row.get::<bool, _>("verified"),
        created_at: row.get::<Option<DateTime<Utc>>, _>("created_at"),
        updated_at: row.get::<Option<DateTime<Utc>>, _>("updated_at"),
        verification_token: row.get::<Option<String>, _>("verification_token"),
        token_expires_at: row.get::<Option<DateTime<Utc>>, _>("token_expires_at"),
        role: row.get::<UserRole, _>("role"),
        provider: row.get::<AuthProvider, _>("provider"),
        skills: row.get::<Option<Vec<SkillResponseDto>>, _>("skills"),
    }
}

pub fn map_row_to_skill(row: &PgRow) -> Skill {
    Skill {
        id: row.get::<Uuid, _>("id"),
        name: row.get::<String, _>("name"),
    }
}
