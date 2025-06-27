use crate::{
    core::{
        errors::{ErrorMessage, HttpError},
        result::{RequestQueryDto, Response, UserData, UserListResponseDto, UserResponseDto},
        state::AppState,
    },
    features::{
        auth::{repository::AuthRepository, service::AuthService},
        skills::dto::SkillResponseDto,
        skills::repository::SkillRepository,
        users::{
            dto::{FilterUserDto, NameUpdateDto, RegisterUserDto, RoleUpdateDto},
            repository::UserRepository,
        },
    },
};
use axum::{
    Extension, Json,
    extract::{Path, Query},
    response::IntoResponse,
};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

pub async fn get_users(
    Query(query_params): Query<RequestQueryDto>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);

    let users = app_state
        .user_service
        .get_users(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user_count = app_state
        .user_service
        .get_user_count()
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let mut users_with_skills = Vec::new();
    for user in &users {
        let skills = app_state
            .skill_service
            .get_skills_of_user(user.id)
            .await
            .map_err(|e| HttpError::server_error(e.to_string()))?;
        let skills_dto = skills
            .into_iter()
            .map(|s| SkillResponseDto {
                id: s.id.to_string(),
                name: s.name,
            })
            .collect();
        users_with_skills.push(FilterUserDto::filter_user_with_skills(user, skills_dto));
    }

    let response = UserListResponseDto {
        status: "success".to_string(),
        users: users_with_skills,
        results: user_count,
    };

    Ok(Json(response))
}

pub async fn get_user(
    Path(user_id): Path<Uuid>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let user = app_state
        .user_service
        .get_user(Some(user_id), None, None, None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?
        .ok_or_else(|| HttpError::bad_request(ErrorMessage::UserNoLongerExist.to_string()))?;

    let skills = app_state
        .skill_service
        .get_skills_of_user(user.id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    let skills_dto = skills
        .into_iter()
        .map(|s| SkillResponseDto {
            id: s.id.to_string(),
            name: s.name,
        })
        .collect();

    let response = UserResponseDto {
        status: "success".to_string(),
        data: UserData {
            user: FilterUserDto::filter_user_with_skills(&user, skills_dto),
        },
    };

    Ok(Json(response))
}

pub async fn get_user_by_email(
    Path(email): Path<String>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let user = app_state
        .user_service
        .get_user(None, None, Some(&email), None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?
        .ok_or_else(|| HttpError::bad_request(ErrorMessage::UserNoLongerExist.to_string()))?;

    let response = UserResponseDto {
        status: "success".to_string(),
        data: UserData {
            user: FilterUserDto::filter_user(&user),
        },
    };

    Ok(Json(response))
}

pub async fn create_user(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<RegisterUserDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    // Check if email already exists
    if (app_state
        .user_service
        .get_user(None, None, Some(&body.email), None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?)
    .is_some()
    {
        return Err(HttpError::unique_constraint_violation(
            ErrorMessage::EmailExist.to_string(),
        ));
    }

    let hashed_password = AuthService::hash_password(body.password)
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    // Generate verification token and expiry
    let verification_token = Uuid::new_v4().to_string();
    let token_expires_at = Utc::now() + chrono::Duration::hours(24);

    let user = app_state
        .user_service
        .save_user(
            body.name,
            body.email,
            hashed_password,
            verification_token,
            token_expires_at,
        )
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = UserResponseDto {
        status: "success".to_string(),
        data: UserData {
            user: FilterUserDto::filter_user(&user),
        },
    };

    Ok(Json(response))
}

pub async fn update_user(
    Path(user_id): Path<Uuid>,
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<NameUpdateDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let user = app_state
        .user_service
        .update_user_name(user_id, body.name)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = UserResponseDto {
        status: "success".to_string(),
        data: UserData {
            user: FilterUserDto::filter_user(&user),
        },
    };

    Ok(Json(response))
}

pub async fn update_user_role(
    Path(user_id): Path<Uuid>,
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<RoleUpdateDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let user = app_state
        .user_service
        .update_user_role(user_id, body.role)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = UserResponseDto {
        status: "success".to_string(),
        data: UserData {
            user: FilterUserDto::filter_user(&user),
        },
    };

    Ok(Json(response))
}

pub async fn verify_user(
    Path(token): Path<String>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    app_state
        .user_service
        .verifed_token(&token)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = Response {
        status: "success",
        message: "User verified successfully".to_string(),
    };

    Ok(Json(response))
}

pub async fn delete_user(
    Path(user_id): Path<Uuid>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let user = app_state
        .user_service
        .get_user(Some(user_id), None, None, None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?
        .ok_or_else(|| HttpError::bad_request(ErrorMessage::UserNoLongerExist.to_string()))?;

    // Implement this Later

    let response = Response {
        status: "success",
        message: format!("User {} deleted successfully (Dummy)", user.name),
    };

    Ok(Json(response))
}
