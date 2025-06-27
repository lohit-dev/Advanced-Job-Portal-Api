use crate::features::skills::model::Skill;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait SkillRepository {
    // Normal Skill Management
    async fn create_skill(&self, name: String) -> Result<Skill, sqlx::Error>;
    async fn get_skills(&self, page: u32, offset: usize) -> Result<Vec<Skill>, sqlx::Error>;
    async fn get_skill(
        &self,
        skill_id: Option<Uuid>,
        name: Option<&str>,
    ) -> Result<Option<Skill>, sqlx::Error>;
    async fn update_skill(
        &self,
        skill_id: Option<Uuid>,
        new_skill: Option<&str>,
    ) -> Result<Skill, sqlx::Error>;
    async fn delete_skill(&self, id: Uuid) -> Result<u64, sqlx::Error>;

    // User-skill Management
    async fn add_skill_to_user(&self, user_id: Uuid, skill_id: Uuid) -> Result<(), sqlx::Error>;
    async fn remove_skill_from_user(
        &self,
        user_id: Uuid,
        skill_id: Uuid,
    ) -> Result<(), sqlx::Error>;
    async fn get_skills_of_user(&self, user_id: Uuid) -> Result<Vec<Skill>, sqlx::Error>;
}
