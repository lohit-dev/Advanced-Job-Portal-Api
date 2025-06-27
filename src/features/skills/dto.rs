use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateSkillDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateSkillDto {
    pub id: uuid::Uuid,
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillResponseDto {
    pub id: String,
    pub name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SkillListResponseDto {
    pub status: String,
    pub skills: Vec<SkillResponseDto>,
    pub results: usize,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AddUserSkillDto {
    pub user_id: uuid::Uuid,
    pub skill_id: uuid::Uuid,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RemoveUserSkillDto {
    pub user_id: uuid::Uuid,
    pub skill_id: uuid::Uuid,
}
