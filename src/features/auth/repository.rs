use crate::core::error::{ErrorMessage, HttpError};

pub trait AuthRepository {
    fn create_token(
        user_id: &str,
        secret: &[u8],
        expires_in_seconds: i64,
    ) -> Result<String, jsonwebtoken::errors::Error>;

    fn decode_token<T: Into<String>>(token: T, secret: &[u8]) -> Result<String, HttpError>;
    fn hash(password: impl Into<String>) -> Result<String, ErrorMessage>;
    fn compare(password: &str, hashed_password: &str) -> Result<bool, ErrorMessage>;
}
