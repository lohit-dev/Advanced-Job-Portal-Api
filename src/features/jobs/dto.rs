use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::features::jobs::model::JobType;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateRoundDto {
    #[validate(length(min = 1, message = "At least one stage is required"))]
    pub stages: Vec<Uuid>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateJobDto {
    #[validate(length(min = 1, message = "Title is required"))]
    pub title: String,
    #[validate(length(min = 1, message = "Description is required"))]
    pub description: String,
    #[validate(length(min = 1, message = "Company is required"))]
    pub company: String,
    #[validate(length(min = 1, message = "Location is required"))]
    pub location: String,
    pub salary_min: Option<i32>,
    pub salary_max: Option<i32>,
    pub job_type: JobType,
    pub rounds: i32,
    pub round_details: Option<CreateRoundDto>,
    pub skills_required: Option<Uuid>, // Skill UUID
    pub experience_min: Option<i32>,
    pub experience_max: Option<i32>,
    pub is_remote: Option<bool>,
    pub application_deadline: Option<NaiveDate>,
    pub skills: Vec<Uuid>, // List of skill IDs
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateJobDto {
    pub id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub company: Option<String>,
    pub location: Option<String>,
    pub salary_min: Option<i32>,
    pub salary_max: Option<i32>,
    pub job_type: Option<JobType>,
    pub rounds: Option<i32>,
    pub round_details: Option<CreateRoundDto>,
    pub skills_required: Option<Uuid>,
    pub experience_min: Option<i32>,
    pub experience_max: Option<i32>,
    pub is_remote: Option<bool>,
    pub application_deadline: Option<NaiveDate>,
    pub skills: Option<Vec<Uuid>>, // Optional list of skill IDs
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobResponseDto {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub company: String,
    pub location: String,
    pub salary_min: Option<i32>,
    pub salary_max: Option<i32>,
    pub job_type: String,
    pub rounds: i32,
    pub round_details: Option<CreateRoundDto>,
    pub skills_required: Option<Uuid>,
    pub experience_min: Option<i32>,
    pub experience_max: Option<i32>,
    pub is_remote: bool,
    pub application_deadline: Option<NaiveDate>,
}
