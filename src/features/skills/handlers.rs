use crate::{
    core::{
        errors::{ErrorMessage, HttpError},
        result::{RequestQueryDto, Response},
        state::AppState,
    },
    features::skills::{
        dto::{
            AddUserSkillDto, CreateSkillDto, RemoveUserSkillDto, SkillListResponseDto,
            SkillResponseDto, UpdateSkillDto, UsersOfSkillResponseDto,
        },
        repository::SkillRepository,
    },
};
use axum::{
    Extension, Json,
    extract::{Path, Query},
    response::IntoResponse,
};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

pub async fn get_skills(
    Query(query_params): Query<RequestQueryDto>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);

    let skills = app_state
        .skill_service
        .get_skills(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = SkillListResponseDto {
        status: "success".to_string(),
        results: skills.len(),
        skills: skills
            .into_iter()
            .map(|s| SkillResponseDto {
                id: s.id.to_string(),
                name: s.name,
            })
            .collect(),
    };
    Ok(Json(response))
}

pub async fn get_skill(
    Path(skill_id): Path<Uuid>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let skill = app_state
        .skill_service
        .get_skill(Some(skill_id), None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?
        .ok_or_else(|| HttpError::not_found(ErrorMessage::NotFound.to_string()))?;

    let response = SkillResponseDto {
        id: skill.id.to_string(),
        name: skill.name,
    };
    Ok(Json(response))
}

pub async fn get_skill_by_name(
    Query(params): Query<HashMap<String, String>>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let name = params
        .get("name")
        .ok_or_else(|| HttpError::bad_request("Missing 'name' query parameter".to_string()))?;
    let skill = app_state
        .skill_service
        .get_skill(None, Some(name.as_str()))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?
        .ok_or_else(|| HttpError::not_found(ErrorMessage::NotFound.to_string()))?;

    let response = SkillResponseDto {
        id: skill.id.to_string(),
        name: skill.name,
    };
    Ok(Json(response))
}

pub async fn create_skill(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<CreateSkillDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;
    let skill = app_state
        .skill_service
        .create_skill(body.name)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    let response = SkillResponseDto {
        id: skill.id.to_string(),
        name: skill.name,
    };
    Ok(Json(response))
}

pub async fn update_skill(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<UpdateSkillDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let skill = app_state
        .skill_service
        .update_skill(Some(body.id), Some(&body.name))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let res = Response {
        status: "Success",
        message: format!("Skill with id - {:#?} Updated Succesfully", skill.id),
    };

    Ok(Json(res))
}

pub async fn delete_skill(
    Path(skill_id): Path<Uuid>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let affected = app_state
        .skill_service
        .delete_skill(skill_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    if affected == 0 {
        return Err(HttpError::not_found(ErrorMessage::NotFound.to_string()));
    }
    let response = Response {
        status: "success",
        message: "Skill deleted successfully".to_string(),
    };
    Ok(Json(response))
}

pub async fn add_skill_to_user(
    Path(user_id): Path<Uuid>,
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<AddUserSkillDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;
    app_state
        .skill_service
        .add_skill_to_user(user_id, body.skill_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    let response = Response {
        status: "success",
        message: "Skill added to user successfully".to_string(),
    };
    Ok(Json(response))
}

pub async fn remove_skill_from_user(
    Path(user_id): Path<Uuid>,
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<RemoveUserSkillDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;
    app_state
        .skill_service
        .remove_skill_from_user(user_id, body.skill_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    let response = Response {
        status: "success",
        message: "Skill removed from user successfully".to_string(),
    };
    Ok(Json(response))
}

pub async fn get_skills_of_user(
    Path(user_id): Path<Uuid>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let skills = app_state
        .skill_service
        .get_skills_of_user(user_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    let response = SkillListResponseDto {
        status: "success".to_string(),
        results: skills.len(),
        skills: skills
            .into_iter()
            .map(|s| SkillResponseDto {
                id: s.id.to_string(),
                name: s.name,
            })
            .collect(),
    };
    Ok(Json(response))
}

pub async fn get_users_of_skill(
    Query(params): Query<HashMap<String, String>>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let skill_id = params
        .get("skill_id")
        .ok_or_else(|| HttpError::bad_request("Missing 'skill_id' query parameter".to_string()))?;
    let skill_id = uuid::Uuid::parse_str(skill_id)
        .map_err(|_| HttpError::bad_request("Invalid 'skill_id' format".to_string()))?;

    let users = app_state
        .skill_service
        .get_users_of_skill(skill_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = UsersOfSkillResponseDto {
        status: "success".to_string(),
        results: users.len(),
        users,
    };
    Ok(Json(response))
}
