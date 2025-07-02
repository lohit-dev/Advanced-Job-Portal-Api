use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "job_type")]
#[sqlx(rename_all = "PascalCase")]
pub enum JobType {
    Remote,
    OnSite,
    Hybrid,
}

impl JobType {
    pub fn to_str(&self) -> &str {
        match self {
            JobType::Remote => "Remote",
            JobType::OnSite => "OnSite",
            JobType::Hybrid => "Hybrid",
        }
    }
}

impl std::fmt::Display for JobType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobType::Remote => write!(f, "Remote"),
            JobType::OnSite => write!(f, "OnSite"),
            JobType::Hybrid => write!(f, "Hybrid"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct RoundCategory {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Job {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub company: String,
    pub location: String,
    pub salary_min: Option<i32>,
    pub salary_max: Option<i32>,
    pub job_type: JobType,
    pub rounds: i32,
    pub round_details: Value,
    pub experience_min: Option<i32>,
    pub experience_max: Option<i32>,
    pub is_remote: bool,
    pub application_deadline: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
