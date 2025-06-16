use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::Query,
    http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
};
use axum_extra::extract::cookie::Cookie;
use chrono::{Duration, Utc};
use validator::Validate;

use crate::{
    core::{
        error::{ErrorMessage, HttpError},
        result::{Response, UserLoginResponseDto},
        state::AppState,
    },
    features::{
        auth::{
            dto::{ForgotPasswordRequestDto, ResetPasswordRequestDto, VerifyEmailQueryDto},
            repository::AuthRepository,
            service::AuthService,
        },
        mail::mails::{send_forgot_password_email, send_verification_email, send_welcome_email},
        users::{
            dto::{LoginUserDto, RegisterUserDto},
            repository::UserRepository,
        },
    },
};

pub async fn register(
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
    let verification_token = uuid::Uuid::new_v4().to_string();
    let token_expires_at = Utc::now() + Duration::hours(24);

    let user = app_state
        .user_service
        .save_user(
            body.name,
            body.email,
            hashed_password,
            verification_token.clone(),
            token_expires_at,
        )
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    send_verification_email(&user.email, &user.name, &verification_token)
        .await
        .map_err(|e| {
            HttpError::server_error(format!("Failed to send verification email: {}", e))
        })?;

    let response = Response {
        status: "success",
        message: "Registration successful! Please check your email to verify your account."
            .to_string(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn login(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<LoginUserDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let user = app_state
        .user_service
        .get_user(None, None, Some(&body.email), None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?
        .ok_or_else(|| HttpError::bad_request(ErrorMessage::WrongCredentials.to_string()))?;

    let password_matched = AuthService::compare(&body.password, &user.password)
        .map_err(|_| HttpError::bad_request(ErrorMessage::WrongCredentials.to_string()))?;

    if !password_matched {
        return Err(HttpError::bad_request(
            ErrorMessage::WrongCredentials.to_string(),
        ));
    }

    let token = AuthService::create_token(
        &user.id.to_string(),
        app_state.config.app.jwt_secret.as_bytes(),
        app_state.config.app.jwt_maxage,
    )
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    let cookie_duration = time::Duration::minutes(app_state.config.app.jwt_maxage * 60);
    let cookie = Cookie::build(("token", token.clone()))
        .path("/")
        .max_age(cookie_duration)
        .http_only(true)
        .build();

    let mut headers = HeaderMap::new();
    headers.append(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    let response = Json(UserLoginResponseDto {
        status: "success".to_string(),
        token,
    });

    let mut response = response.into_response();
    response.headers_mut().extend(headers);

    Ok(response)
}

pub async fn verify_email(
    Query(query_params): Query<VerifyEmailQueryDto>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let user = app_state
        .user_service
        .get_user(None, None, None, Some(&query_params.token))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?
        .ok_or_else(|| HttpError::unauthorized(ErrorMessage::InvalidToken.to_string()))?;

    if let Some(expires_at) = user.token_expires_at {
        if Utc::now() > expires_at {
            return Err(HttpError::bad_request(
                "Verification token has expired".to_string(),
            ));
        }
    } else {
        return Err(HttpError::bad_request(
            "Invalid verification token".to_string(),
        ));
    }

    app_state
        .user_service
        .verifed_token(&query_params.token)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    send_welcome_email(&user.email, &user.name)
        .await
        .map_err(|e| HttpError::server_error(format!("Failed to send welcome email: {}", e)))?;

    let token = AuthService::create_token(
        &user.id.to_string(),
        app_state.config.app.jwt_secret.as_bytes(),
        app_state.config.app.jwt_maxage,
    )
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    let cookie_duration = time::Duration::minutes(app_state.config.app.jwt_maxage * 60);
    let cookie = Cookie::build(("token", token.clone()))
        .path("/")
        .max_age(cookie_duration)
        .http_only(true)
        .build();

    let mut headers = HeaderMap::new();
    headers.append(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    // let frontend_url = "http://localhost:5173/settings".to_string();
    // let redirect = Redirect::to(&frontend_url);
    // let mut response = redirect.into_response();

    let response = Json(UserLoginResponseDto {
        status: "success".to_string(),
        token,
    });

    let mut response = response.into_response();
    response.headers_mut().extend(headers);

    Ok(response)
}

pub async fn forgot_password(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<ForgotPasswordRequestDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let user = app_state
        .user_service
        .get_user(None, None, Some(&body.email), None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?
        .ok_or_else(|| HttpError::bad_request("Email not found!".to_string()))?;

    let verification_token = uuid::Uuid::new_v4().to_string();
    let expires_at = Utc::now() + Duration::minutes(30);

    app_state
        .user_service
        .add_verifed_token(user.id, &verification_token, expires_at)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let base_url = "https://e-commerce-backend-rs.onrender.com/api/auth/verify";
    let reset_link = format!(
        "https://e-commerce-backend-rs.onrender.com/api/auth/reset-password?token={}",
        verification_token
    );
    send_forgot_password_email(&user.email, &reset_link, &user.name)
        .await
        .map_err(|e| {
            HttpError::server_error(format!("Failed to send reset password email: {}", e))
        })?;

    let response = Response {
        status: "success",
        message: "Password reset link has been sent to your email.".to_string(),
    };

    Ok(Json(response))
}

pub async fn reset_password(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<ResetPasswordRequestDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let user = app_state
        .user_service
        .get_user(None, None, None, Some(&body.token))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?
        .ok_or_else(|| HttpError::bad_request("Invalid or expired token".to_string()))?;

    if let Some(expires_at) = user.token_expires_at {
        if Utc::now() > expires_at {
            return Err(HttpError::bad_request(
                "Verification token has expired".to_string(),
            ));
        }
    } else {
        return Err(HttpError::bad_request(
            "Invalid verification token".to_string(),
        ));
    }

    let hashed_password = AuthService::hash_password(body.new_password)
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    app_state
        .user_service
        .update_user_password(user.id, hashed_password)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    app_state
        .user_service
        .verifed_token(&body.token)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = Response {
        status: "success",
        message: "Password has been successfully reset.".to_string(),
    };

    Ok(Json(response))
}
