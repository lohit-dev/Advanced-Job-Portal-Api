use crate::{
    core::{
        errors::{ErrorMessage, HttpError},
        result::{Response, UserLoginResponseDto},
        state::AppState,
    },
    features::{
        auth::{
            dto::{ForgotPasswordRequestDto, ResetPasswordRequestDto, VerifyEmailQueryDto},
            model::{AuthProvider, GoogleCallbackQuery},
            oauth::google_oauth::GoogleOAuth,
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
use axum::{
    Extension, Json,
    extract::Query,
    http::{HeaderMap, StatusCode, header},
    response::{Html, IntoResponse, Redirect},
};
use axum_extra::extract::cookie::Cookie;
use chrono::{Duration, Utc};
use oauth2::PkceCodeVerifier;
use std::sync::Arc;
use validator::Validate;

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

    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Email Verified</title>
    <style>
        body {
            background-color: #f4f4f4;
            font-family: Arial, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            margin: 0;
        }
        .container {
            background-color: #ffffff;
            padding: 40px;
            border-radius: 10px;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
            text-align: center;
            max-width: 400px;
        }
        h1 {
            color: #28a745;
            margin-bottom: 20px;
        }
        p {
            color: #555555;
            font-size: 16px;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>Email Verified ✅</h1>
        <p>Your email has been successfully verified.</p>
        <p>You can now close this window.</p>
    </div>
</body>
</html>
"#;

    // let response = Json(UserLoginResponseDto {
    //     status: "success".to_string(),
    //     token,
    // });

    let mut response = Html(html).into_response();
    response.headers_mut().extend(headers); // still set cookie

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

pub async fn google_login(
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let google_oauth = GoogleOAuth::new(&app_state.config.oauth);
    let (auth_url, csrf_token, pkce_verifier) = google_oauth.generate_auth_url();

    // Store CSRF token and PKCE verifier in cookies for validation later
    let csrf_cookie = Cookie::build(("oauth_csrf", csrf_token.secret()))
        .path("/")
        .max_age(time::Duration::minutes(10))
        .http_only(true)
        .secure(true)
        .same_site(axum_extra::extract::cookie::SameSite::Lax)
        .build();

    let pkce_cookie = Cookie::build(("oauth_pkce", pkce_verifier.secret()))
        .path("/")
        .max_age(time::Duration::minutes(10))
        .http_only(true)
        .secure(true)
        .same_site(axum_extra::extract::cookie::SameSite::Lax)
        .build();

    let mut headers = HeaderMap::new();
    headers.append(header::SET_COOKIE, csrf_cookie.to_string().parse().unwrap());
    headers.append(header::SET_COOKIE, pkce_cookie.to_string().parse().unwrap());

    let redirect = Redirect::temporary(&auth_url);
    let mut response = redirect.into_response();
    response.headers_mut().extend(headers);

    Ok(response)
}

pub async fn google_callback(
    Query(query): Query<GoogleCallbackQuery>,
    Extension(app_state): Extension<Arc<AppState>>,
    cookies: axum_extra::extract::CookieJar,
) -> Result<impl IntoResponse, HttpError> {
    // Retrieve and validate CSRF token
    let stored_csrf = cookies
        .get("oauth_csrf")
        .ok_or_else(|| HttpError::bad_request("Missing CSRF token".to_string()))?
        .value();

    if stored_csrf != query.state {
        return Err(HttpError::bad_request("Invalid CSRF token".to_string()));
    }

    // Retrieve PKCE verifier
    let pkce_secret = cookies
        .get("oauth_pkce")
        .ok_or_else(|| HttpError::bad_request("Missing PKCE verifier".to_string()))?
        .value();

    let pkce_verifier = PkceCodeVerifier::new(pkce_secret.to_string());

    // Exchange authorization code for user info
    let google_oauth = GoogleOAuth::new(&app_state.config.oauth);
    let http_client = reqwest::Client::new();

    let google_user = google_oauth
        .exchange_code(query.code, pkce_verifier, &http_client)
        .await
        .map_err(|e| HttpError::server_error(format!("OAuth exchange failed: {}", e)))?;

    // Check if user exists in database
    let existing_user = app_state
        .user_service
        .get_user(None, None, Some(&google_user.email), None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user = match existing_user {
        Some(user) => user,
        None => {
            let new_user = app_state
                .user_service
                .save_oauth_user(
                    google_user.name.clone(),
                    google_user.email.clone(),
                    AuthProvider::Google,
                )
                .await
                .map_err(|e| HttpError::server_error(e.to_string()))?;

            send_welcome_email(&google_user.email, &google_user.name)
                .await
                .map_err(|e| {
                    HttpError::server_error(format!("Failed to send welcome email: {}", e))
                })?;

            new_user
        }
    };

    // Generate JWT token for the user
    let token = AuthService::create_token(
        &user.id.to_string(),
        app_state.config.app.jwt_secret.as_bytes(),
        app_state.config.app.jwt_maxage,
    )
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    // Create auth cookie
    let cookie_duration = time::Duration::minutes(app_state.config.app.jwt_maxage * 60);
    let auth_cookie = Cookie::build(("token", token.clone()))
        .path("/")
        .max_age(cookie_duration)
        .http_only(true)
        .secure(true) // Use secure in production
        .same_site(axum_extra::extract::cookie::SameSite::Lax)
        .build();

    // Clear OAuth temporary cookies
    let clear_csrf = Cookie::build(("oauth_csrf", ""))
        .path("/")
        .max_age(time::Duration::seconds(0))
        .build();

    let clear_pkce = Cookie::build(("oauth_pkce", ""))
        .path("/")
        .max_age(time::Duration::seconds(0))
        .build();

    let mut headers = HeaderMap::new();
    headers.append(header::SET_COOKIE, auth_cookie.to_string().parse().unwrap());
    headers.append(header::SET_COOKIE, clear_csrf.to_string().parse().unwrap());
    headers.append(header::SET_COOKIE, clear_pkce.to_string().parse().unwrap());

    // Redirect to frontend success page or return JSON response
    // let frontend_url = "http://localhost:5173/dashboard"; // Adjust as needed
    // let redirect = Redirect::temporary(frontend_url);
    // let mut response = redirect.into_response();
    // response.headers_mut().extend(headers);

    let response = Json(UserLoginResponseDto {
        status: "success".to_string(),
        token,
    });
    let mut response = response.into_response();
    response.headers_mut().extend(headers);

    Ok(response)
}
