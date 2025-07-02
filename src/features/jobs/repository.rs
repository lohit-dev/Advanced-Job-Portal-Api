use crate::features::jobs::model::{Job, RoundCategory};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait JobRepository {
    async fn create_job(&self, job: Job) -> Result<Job, sqlx::Error>;
    async fn get_jobs(&self, page: u32, limit: usize) -> Result<Vec<Job>, sqlx::Error>;
    async fn get_job(&self, job_id: Uuid) -> Result<Option<Job>, sqlx::Error>;
    async fn update_job(&self, job: Job) -> Result<Job, sqlx::Error>;
    async fn delete_job(&self, job_id: Uuid) -> Result<u64, sqlx::Error>;
    async fn get_job_count(&self) -> Result<i64, sqlx::Error>;

    async fn get_jobs_by_skill(
        &self,
        skill_id: Uuid,
        page: u32,
        limit: usize,
    ) -> Result<Vec<Job>, sqlx::Error>;

    async fn get_jobs_by_round_category(
        &self,
        round_category_id: Uuid,
        page: u32,
        limit: usize,
    ) -> Result<Vec<Job>, sqlx::Error>;

    // For round categories
    async fn get_round_categories(&self) -> Result<Vec<RoundCategory>, sqlx::Error>;

    async fn get_round_category_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<RoundCategory>, sqlx::Error>;

    async fn add_skills_to_job(
        &self,
        job_id: Uuid,
        skill_ids: Vec<Uuid>,
    ) -> Result<(), sqlx::Error>;
    async fn remove_skills_from_job(
        &self,
        job_id: Uuid,
        skill_ids: Vec<Uuid>,
    ) -> Result<(), sqlx::Error>;
    async fn get_skills_of_job(
        &self,
        job_id: Uuid,
    ) -> Result<Vec<crate::features::skills::model::Skill>, sqlx::Error>;
    async fn get_jobs_of_skill(&self, skill_id: Uuid) -> Result<Vec<Job>, sqlx::Error>;
}
