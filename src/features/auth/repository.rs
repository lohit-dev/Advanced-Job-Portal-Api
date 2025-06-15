use async_trait::async_trait;

use crate::core::error::HttpError;

#[async_trait]
pub trait authRepository {
    fn create_token(
        user_id: &str,
        secret: &[u8],
        expires_in_seconds: i64,
    ) -> Result<String, jsonwebtoken::errors::Error>;

    fn decode_token<T: Into<String>>(token: T, secret: &[u8]) -> Result<String, HttpError>;
}
