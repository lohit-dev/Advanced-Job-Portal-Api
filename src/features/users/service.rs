use crate::{
    features::{
        auth::model::AuthProvider,
        users::{model::User, repository::UserRepository},
    },
    utils::mappers::map_row_to_user,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    pub db: PgPool,
}

#[async_trait]
impl UserRepository for UserService {
    async fn get_user(
        &self,
        user_id: Option<Uuid>,
        name: Option<&str>,
        email: Option<&str>,
        token: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error> {
        let mut user: Option<User> = None;

        if let Some(user_id) = user_id {
            let row = sqlx::query(
                "SELECT id, name, email, password, verified, created_at, updated_at, verification_token, token_expires_at, role, provider FROM users WHERE id = $1"
            )
            .bind(user_id)
            .fetch_optional(&self.db)
            .await?;

            if let Some(row) = row {
                user = Some(map_row_to_user(&row));
            }
        } else if let Some(name) = name {
            let row = sqlx::query(
                "SELECT id, name, email, password, verified, created_at, updated_at, verification_token, token_expires_at, role, provider FROM users WHERE name = $1"
            )
            .bind(name)
            .fetch_optional(&self.db)
            .await?;

            if let Some(row) = row {
                user = Some(map_row_to_user(&row));
            }
        } else if let Some(email) = email {
            let row = sqlx::query(
                "SELECT id, name, email, password, verified, created_at, updated_at, verification_token, token_expires_at, role, provider FROM users WHERE email = $1"
            )
            .bind(email)
            .fetch_optional(&self.db)
            .await?;

            if let Some(row) = row {
                user = Some(map_row_to_user(&row));
            }
        } else if let Some(token) = token {
            let row = sqlx::query(
                "SELECT id, name, email, password, verified, created_at, updated_at, verification_token, token_expires_at, role, provider FROM users WHERE verification_token = $1"
            )
            .bind(token)
            .fetch_optional(&self.db)
            .await?;

            if let Some(row) = row {
                user = Some(map_row_to_user(&row));
            }
        }

        Ok(user)
    }

    async fn get_users(&self, page: u32, limit: usize) -> Result<Vec<User>, sqlx::Error> {
        let offset = (page - 1) * limit as u32;
        let rows = sqlx::query(
            "SELECT id, name, email, password, verified, created_at, updated_at, verification_token, token_expires_at, role, provider FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.db)
        .await?;

        let users = rows.iter().map(map_row_to_user).collect();

        Ok(users)
    }

    async fn save_user<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        password: T,
        verification_token: T,
        token_expires_at: DateTime<Utc>,
    ) -> Result<User, sqlx::Error> {
        let row = sqlx::query(
            "INSERT INTO users (name, email, password, verification_token, token_expires_at, provider) 
            VALUES ($1, $2, $3, $4, $5, $6) 
            RETURNING id, name, email, password, verified, created_at, updated_at, verification_token, token_expires_at, role, provider"
        )
        .bind(name.into())
        .bind(email.into())
        .bind(password.into())
        .bind(verification_token.into())
        .bind(token_expires_at)
        .bind(AuthProvider::Local)
        .fetch_one(&self.db)
        .await?;

        Ok(map_row_to_user(&row))
    }

    async fn save_oauth_user<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        provider: AuthProvider,
    ) -> Result<User, sqlx::Error> {
        let row = sqlx::query(
            "INSERT INTO users (name, email, password, verification_token, token_expires_at, provider, verified) 
            VALUES ($1, $2, '', NULL, NULL, $3, true) 
            RETURNING id, name, email, password, verified, created_at, updated_at, verification_token, token_expires_at, role, provider"
        )
        .bind(name.into())
        .bind(email.into())
        .bind(provider)
        .fetch_one(&self.db)
        .await?;

        Ok(map_row_to_user(&row))
    }

    async fn get_user_count(&self) -> Result<i64, sqlx::Error> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM users")
            .fetch_one(&self.db)
            .await?;
        Ok(row.get::<i64, _>("count"))
    }

    async fn update_user_name<T: Into<String> + Send>(
        &self,
        user_id: Uuid,
        new_name: T,
    ) -> Result<User, sqlx::Error> {
        let row = sqlx::query(
            "UPDATE users
            SET name = $1, updated_at = Now()
            WHERE id = $2
            RETURNING id, name, email, password, verified, created_at, updated_at, verification_token, token_expires_at, role, provider"
        )
        .bind(new_name.into())
        .bind(user_id)
        .fetch_one(&self.db)
        .await?;

        Ok(map_row_to_user(&row))
    }

    async fn update_user_role(
        &self,
        user_id: Uuid,
        new_role: super::model::UserRole,
    ) -> Result<User, sqlx::Error> {
        let row = sqlx::query(
            "UPDATE users
            SET role = $1, updated_at = Now()
            WHERE id = $2
            RETURNING id, name, email, password, verified, created_at, updated_at, verification_token, token_expires_at, role, provider"
        )
        .bind(new_role)
        .bind(user_id)
        .fetch_one(&self.db)
        .await?;

        Ok(map_row_to_user(&row))
    }

    async fn update_user_password(
        &self,
        user_id: Uuid,
        new_password: String,
    ) -> Result<User, sqlx::Error> {
        let row = sqlx::query(
            "UPDATE users
            SET password = $1, updated_at = Now()
            WHERE id = $2
            RETURNING id, name, email, password, verified, created_at, updated_at, verification_token, token_expires_at, role, provider"
        )
        .bind(new_password)
        .bind(user_id)
        .fetch_one(&self.db)
        .await?;

        Ok(map_row_to_user(&row))
    }

    async fn verifed_token(&self, token: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE users
            SET verified = true, 
                updated_at = Now(),
                verification_token = NULL,
                token_expires_at = NULL,
                role = 'User'
            WHERE verification_token = $1",
            token
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    async fn add_verifed_token(
        &self,
        user_id: Uuid,
        token: &str,
        token_expires_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE users
            SET verification_token = $1, token_expires_at = $2, updated_at = Now()
            WHERE id = $3",
            token,
            token_expires_at,
            user_id
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}
