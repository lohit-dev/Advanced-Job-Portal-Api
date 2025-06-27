use crate::{
    features::{
        skills::{model::Skill, repository::SkillRepository},
        users::model::User,
    },
    utils::mappers::{map_row_to_skill, map_row_to_user},
};
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[derive(Clone)]
pub struct SkillService {
    pub db: PgPool,
}

#[async_trait]
impl SkillRepository for SkillService {
    async fn create_skill(&self, name: String) -> Result<super::model::Skill, sqlx::Error> {
        let row = sqlx::query("INSERT INTO skills (name) VALUES ($1) RETURNING id, name")
            .bind(&name)
            .fetch_one(&self.db)
            .await?;

        Ok(map_row_to_skill(&row))
    }

    async fn get_skills(
        &self,
        page: u32,
        limit: usize,
    ) -> Result<Vec<super::model::Skill>, sqlx::Error> {
        let offset = (page - 1) * limit as u32;
        let rows = sqlx::query("SELECT * FROM skills ORDER BY name DESC LIMIT $1 OFFSET $2")
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&self.db)
            .await?;

        let skills = rows.iter().map(map_row_to_skill).collect();

        Ok(skills)
    }

    async fn get_skill(
        &self,
        skill_id: Option<Uuid>,
        skill_name: Option<&str>,
    ) -> Result<Option<Skill>, sqlx::Error> {
        let mut skill: Option<Skill> = None;

        if let Some(skill_id) = skill_id {
            let row = sqlx::query("SELECT * FROM skills WHERE id = $1")
                .bind(skill_id)
                .fetch_optional(&self.db)
                .await?;

            if let Some(row) = row {
                skill = Some(map_row_to_skill(&row));
            }
        } else if let Some(name) = skill_name {
            let row = sqlx::query("SELECT * FROM skills where LOWER(name) = LOWER($1)")
                .bind(name)
                .fetch_optional(&self.db)
                .await?;

            if let Some(row) = row {
                skill = Some(map_row_to_skill(&row))
            }
        }

        Ok(skill)
    }

    async fn update_skill(
        &self,
        skill_id: Option<Uuid>,
        new_skill: Option<&str>,
    ) -> Result<Skill, sqlx::Error> {
        let row = sqlx::query(
            "UPDATE skills
            SET name = $1, updated_at = Now()
            WHERE id = $2
            RETURNING id, name",
        )
        .bind(new_skill)
        .bind(skill_id)
        .fetch_one(&self.db)
        .await?;

        Ok(map_row_to_skill(&row))
    }

    async fn delete_skill(&self, id: uuid::Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM skills WHERE id = $1")
            .bind(id)
            .execute(&self.db)
            .await?;

        Ok(result.rows_affected())
    }

    async fn add_skill_to_user(&self, user_id: Uuid, skill_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO user_skills (user_id, skill_id) VALUES ($1, $2)")
            .bind(user_id)
            .bind(skill_id)
            .execute(&self.db)
            .await?;
        Ok(())
    }

    async fn remove_skill_from_user(
        &self,
        user_id: Uuid,
        skill_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM user_skills WHERE user_id = $1 AND skill_id = $2")
            .bind(user_id)
            .bind(skill_id)
            .execute(&self.db)
            .await?;
        Ok(())
    }

    async fn get_skills_of_user(&self, user_id: Uuid) -> Result<Vec<Skill>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT s.id, s.name FROM skills s \
             INNER JOIN user_skills us ON s.id = us.skill_id \
             WHERE us.user_id = $1",
        )
        .bind(user_id)
        .fetch_all(&self.db)
        .await?;
        Ok(rows.iter().map(map_row_to_skill).collect())
    }

    async fn get_users_of_skill(&self, skill_id: Uuid) -> Result<Vec<User>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT * FROM users u \
             INNER JOIN user_skills us ON u.id = us.user_id \
             WHERE us.skill_id = $1",
        )
        .bind(skill_id)
        .fetch_all(&self.db)
        .await?;

        let users = rows.iter().map(|row| map_row_to_user(row)).collect();

        Ok(users)
    }

    async fn get_skill_count(&self) -> Result<i64, sqlx::Error> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM skills")
            .fetch_one(&self.db)
            .await?;
        Ok(row.get::<i64, _>("count"))
    }
}
