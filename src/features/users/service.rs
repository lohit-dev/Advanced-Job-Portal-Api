use crate::features::users::repository::UserRepository;
use async_trait::async_trait;
use sqlx::PgPool;

pub struct UserService {
    pub db: PgPool,
}

#[async_trait]
impl UserRepository for UserService {
    async fn get_user(
        &self,
        user_id: Option<uuid::Uuid>,
        name: Option<&str>,
        email: Option<&str>,
        token: Option<&str>,
    ) -> Result<Option<super::model::User>, sqlx::Error> {
        todo!()
    }

    async fn get_users(
        &self,
        page: u32,
        limit: usize,
    ) -> Result<Vec<super::model::User>, sqlx::Error> {
        todo!()
    }

    async fn save_user<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        password: T,
        verification_token: T,
        token_expires_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<super::model::User, sqlx::Error> {
        todo!()
    }

    async fn get_user_count(&self) -> Result<i64, sqlx::Error> {
        todo!()
    }

    async fn update_user_name<T: Into<String> + Send>(
        &self,
        user_id: uuid::Uuid,
        name: T,
    ) -> Result<super::model::User, sqlx::Error> {
        todo!()
    }

    async fn update_user_role(
        &self,
        user_id: uuid::Uuid,
        role: super::model::UserRole,
    ) -> Result<super::model::User, sqlx::Error> {
        todo!()
    }

    async fn update_user_password(
        &self,
        user_id: uuid::Uuid,
        password: String,
    ) -> Result<super::model::User, sqlx::Error> {
        todo!()
    }

    async fn verifed_token(&self, token: &str) -> Result<(), sqlx::Error> {
        todo!()
    }

    async fn add_verifed_token(
        &self,
        user_id: uuid::Uuid,
        token: &str,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), sqlx::Error> {
        todo!()
    }
}
