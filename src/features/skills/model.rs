use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Skill {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct UserSkill {
    pub id: Uuid,
    pub user_id: Uuid,
    pub skill_id: Uuid,
}
