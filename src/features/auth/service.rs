use crate::{
    core::errors::{ErrorMessage, HttpError},
    features::auth::{model::TokenClaims, repository::AuthRepository},
};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};

pub struct AuthService;

const MAX_PASSWORD_LENGTH: usize = 64;

impl AuthRepository for AuthService {
    fn create_token(
        user_id: &str,
        secret: &[u8],
        expires_in_seconds: i64,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        if user_id.is_empty() {
            return Err(jsonwebtoken::errors::ErrorKind::InvalidSubject.into());
        }

        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::minutes(expires_in_seconds)).timestamp() as usize;
        let claims = TokenClaims {
            sub: user_id.to_string(),
            iat,
            exp,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret),
        )
    }

    fn decode_token<T: Into<String>>(
        token: T,
        secret: &[u8],
    ) -> Result<String, crate::core::errors::HttpError> {
        let decode = decode::<TokenClaims>(
            &token.into(),
            &DecodingKey::from_secret(secret),
            &Validation::new(Algorithm::HS256),
        );

        match decode {
            Ok(token) => Ok(token.claims.sub),
            Err(_) => Err(HttpError::new(
                ErrorMessage::InvalidToken.to_string(),
                StatusCode::UNAUTHORIZED,
            )),
        }
    }

    fn hash_password(password: impl Into<String>) -> Result<String, ErrorMessage> {
        let password = password.into();

        if password.is_empty() {
            return Err(ErrorMessage::EmptyPassword);
        }

        if password.len() > MAX_PASSWORD_LENGTH {
            return Err(ErrorMessage::ExceededMaxPasswordLength(MAX_PASSWORD_LENGTH));
        }

        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| ErrorMessage::HashingError)?
            .to_string();

        Ok(hashed_password)
    }

    fn compare(password: &str, hashed_password: &str) -> Result<bool, ErrorMessage> {
        if password.is_empty() {
            return Err(ErrorMessage::EmptyPassword);
        }

        if password.len() > MAX_PASSWORD_LENGTH {
            return Err(ErrorMessage::ExceededMaxPasswordLength(MAX_PASSWORD_LENGTH));
        }

        let parsed_hash =
            PasswordHash::new(hashed_password).map_err(|_| ErrorMessage::InvalidHashFormat)?;

        let password_matched = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok_and(|_| true);

        Ok(password_matched)
    }
}
