use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use validator::Validate;

use crate::features::users::model::User;

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

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub struct SkillResponseDto {
    pub id: String,
    pub name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SkillListResponseDto {
    pub status: String,
    pub skills: Vec<SkillResponseDto>,
    pub results: usize,
    pub has_next_page: bool,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AddUserSkillDto {
    pub skill_id: uuid::Uuid,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RemoveUserSkillDto {
    pub skill_id: uuid::Uuid,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UsersOfSkillResponseDto {
    pub status: String,
    pub users: Vec<User>,
    pub results: usize,
}
