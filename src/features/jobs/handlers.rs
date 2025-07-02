use crate::features::jobs::dto::{CreateJobDto, UpdateJobDto};
use crate::features::jobs::model::Job;
use crate::{
    core::{
        errors::{ErrorMessage, HttpError},
        result::Response,
        state::AppState,
    },
    features::jobs::repository::JobRepository,
};
use axum::{Extension, Json, extract::Path, response::IntoResponse};
use std::sync::Arc;
use uuid::Uuid;

// CRUD Handlers
pub async fn get_jobs(
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let jobs = app_state
        .job_service
        .get_jobs(1, 100)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    Ok(Json(jobs))
}

pub async fn get_job(
    Path(job_id): Path<Uuid>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let job = app_state
        .job_service
        .get_job(job_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?
        .ok_or_else(|| HttpError::not_found(ErrorMessage::NotFound.to_string()))?;
    Ok(Json(job))
}

pub async fn create_job(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<CreateJobDto>,
) -> Result<impl IntoResponse, HttpError> {
    // Convert DTO to Job (you may want a helper for this)
    let job = Job {
        id: Uuid::new_v4(),
        title: body.title,
        description: body.description,
        company: body.company,
        location: body.location,
        salary_min: body.salary_min,
        salary_max: body.salary_max,
        job_type: body.job_type,
        rounds: body.rounds,
        round_details: serde_json::to_value(body.round_details).unwrap_or_default(),
        experience_min: body.experience_min,
        experience_max: body.experience_max,
        is_remote: body.is_remote.unwrap_or(false),
        application_deadline: body.application_deadline,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    let job = app_state
        .job_service
        .create_job(job)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    // Add skills if provided
    if !body.skills.is_empty() {
        app_state
            .job_service
            .add_skills_to_job(job.id, body.skills)
            .await
            .map_err(|e| HttpError::server_error(e.to_string()))?;
    }
    Ok(Json(job))
}

pub async fn update_job(
    Path(job_id): Path<Uuid>,
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<UpdateJobDto>,
) -> Result<impl IntoResponse, HttpError> {
    let mut job = app_state
        .job_service
        .get_job(job_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?
        .ok_or_else(|| HttpError::not_found(ErrorMessage::NotFound.to_string()))?;
    // Update fields if provided
    if let Some(title) = body.title {
        job.title = title;
    }
    if let Some(description) = body.description {
        job.description = description;
    }
    if let Some(company) = body.company {
        job.company = company;
    }
    if let Some(location) = body.location {
        job.location = location;
    }
    if let Some(salary_min) = body.salary_min {
        job.salary_min = Some(salary_min);
    }
    if let Some(salary_max) = body.salary_max {
        job.salary_max = Some(salary_max);
    }
    if let Some(job_type) = body.job_type {
        job.job_type = job_type;
    }
    if let Some(rounds) = body.rounds {
        job.rounds = rounds;
    }
    if let Some(round_details) = body.round_details {
        job.round_details = serde_json::to_value(round_details).unwrap_or_default();
    }
    if let Some(experience_min) = body.experience_min {
        job.experience_min = Some(experience_min);
    }
    if let Some(experience_max) = body.experience_max {
        job.experience_max = Some(experience_max);
    }
    if let Some(is_remote) = body.is_remote {
        job.is_remote = is_remote;
    }
    if let Some(application_deadline) = body.application_deadline {
        job.application_deadline = Some(application_deadline);
    }
    job.updated_at = chrono::Utc::now();
    let job = app_state
        .job_service
        .update_job(job)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    if let Some(skills) = body.skills {
        app_state
            .job_service
            .add_skills_to_job(job.id, skills)
            .await
            .map_err(|e| HttpError::server_error(e.to_string()))?;
    }
    Ok(Json(job))
}

pub async fn delete_job(
    Path(job_id): Path<Uuid>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let affected = app_state
        .job_service
        .delete_job(job_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    if affected == 0 {
        return Err(HttpError::not_found(ErrorMessage::NotFound.to_string()));
    }
    let response = Response {
        status: "success",
        message: "Job deleted successfully".to_string(),
    };
    Ok(Json(response))
}

// Skills management handlers
pub async fn get_skills_of_job(
    Path(job_id): Path<Uuid>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let skills = app_state
        .job_service
        .get_skills_of_job(job_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    Ok(Json(skills))
}

pub async fn add_skills_to_job(
    Path(job_id): Path<Uuid>,
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<Vec<Uuid>>,
) -> Result<impl IntoResponse, HttpError> {
    app_state
        .job_service
        .add_skills_to_job(job_id, body)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    let response = Response {
        status: "success",
        message: "Skills added to job successfully".to_string(),
    };
    Ok(Json(response))
}

pub async fn remove_skills_from_job(
    Path(job_id): Path<Uuid>,
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<Vec<Uuid>>,
) -> Result<impl IntoResponse, HttpError> {
    app_state
        .job_service
        .remove_skills_from_job(job_id, body)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    let response = Response {
        status: "success",
        message: "Skills removed from job successfully".to_string(),
    };
    Ok(Json(response))
}

pub async fn get_jobs_of_skill(
    Path(skill_id): Path<Uuid>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let jobs = app_state
        .job_service
        .get_jobs_of_skill(skill_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    Ok(Json(jobs))
}
