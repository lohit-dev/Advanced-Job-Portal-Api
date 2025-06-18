use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{self, Display, write},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String,
    pub status: StatusCode,
}

#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    InvalidHashFormat,
    HashingError,
    InvalidToken,
    ServerError,
    WrongCredentials,
    EmailExist,
    UserNoLongerExist,
    TokenNotProvided,
    PermissionDenied,
    UserNotAuthenticated,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            ErrorMessage::EmptyPassword => "Password cannot be empty",
            ErrorMessage::ExceededMaxPasswordLength(len) => {
                return write!(f, "Password exceeds max length: {} characters", len);
            }
            ErrorMessage::InvalidHashFormat => "Password hash format is invalid",
            ErrorMessage::HashingError => "An error occurred while hashing the password",
            ErrorMessage::InvalidToken => "Token is invalid or malformed",
            ErrorMessage::ServerError => "Internal server error",
            ErrorMessage::WrongCredentials => "Incorrect email or password",
            ErrorMessage::EmailExist => "Email already exists",
            ErrorMessage::UserNoLongerExist => "User no longer exists",
            ErrorMessage::TokenNotProvided => "Authorization token not provided",
            ErrorMessage::PermissionDenied => "You do not have permission to access this resource",
            ErrorMessage::UserNotAuthenticated => "User authentication required",
        };
        write!(f, "{msg}")
    }
}

impl HttpError {
    pub fn new(message: impl Into<String>, status: StatusCode) -> Self {
        HttpError {
            message: message.into(),
            status,
        }
    }

    pub fn server_error(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::BAD_REQUEST,
        }
    }

    pub fn unique_constraint_violation(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::CONFLICT,
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::UNAUTHORIZED,
        }
    }

    pub fn into_http_response(self) -> Response {
        let json_response = Json(ErrorResponse {
            status: "fail".to_string(),
            message: self.message.clone(),
        });

        (self.status, json_response).into_response()
    }
}

impl Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HttpError: message: {}, status: {}",
            self.message, self.status
        )
    }
}

// Needed for into response
impl Error for HttpError {}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        self.into_http_response()
    }
}


#[derive(Debug)]
pub enum OAuthError {
    OAuth(String),
    Http(reqwest::Error),
}

impl std::fmt::Display for OAuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OAuthError::OAuth(e) => write!(f, "OAuth error: {}", e),
            OAuthError::Http(e) => write!(f, "HTTP error: {}", e),
        }
    }
}

impl std::error::Error for OAuthError {}

impl From<reqwest::Error> for OAuthError {
    fn from(err: reqwest::Error) -> Self {
        OAuthError::Http(err)
    }
} 