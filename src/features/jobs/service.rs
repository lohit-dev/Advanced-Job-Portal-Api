use crate::features::jobs::model::{Job, RoundCategory};
use crate::features::jobs::repository::JobRepository;
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[derive(Clone)]
pub struct JobService {
    pub db: PgPool,
}

#[async_trait]
impl JobRepository for JobService {
    async fn create_job(&self, job: Job) -> Result<Job, sqlx::Error> {
        let round_details_json = serde_json::to_value(&job.round_details).ok();
        let row = sqlx::query_as::<_, Job>(
            "INSERT INTO jobs (id, title, description, company, location, salary_min, salary_max, job_type, rounds, round_details, experience_min, experience_max, is_remote, application_deadline, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            RETURNING *"
        )
        .bind(job.id)
        .bind(&job.title)
        .bind(&job.description)
        .bind(&job.company)
        .bind(&job.location)
        .bind(job.salary_min)
        .bind(job.salary_max)
        .bind(&job.job_type)
        .bind(job.rounds)
        .bind(round_details_json)
        .bind(job.experience_min)
        .bind(job.experience_max)
        .bind(job.is_remote)
        .bind(job.application_deadline)
        .bind(job.created_at)
        .bind(job.updated_at)
        .fetch_one(&self.db)
        .await?;
        Ok(row)
    }

    async fn get_jobs(&self, page: u32, limit: usize) -> Result<Vec<Job>, sqlx::Error> {
        let offset = (page - 1) * limit as u32;
        let rows = sqlx::query_as::<_, Job>(
            "SELECT * FROM jobs ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.db)
        .await?;
        Ok(rows)
    }

    async fn get_job(&self, job_id: Uuid) -> Result<Option<Job>, sqlx::Error> {
        let row = sqlx::query_as::<_, Job>("SELECT * FROM jobs WHERE id = $1")
            .bind(job_id)
            .fetch_optional(&self.db)
            .await?;
        Ok(row)
    }

    async fn update_job(&self, job: Job) -> Result<Job, sqlx::Error> {
        let round_details_json = serde_json::to_value(&job.round_details).ok();
        let row = sqlx::query_as::<_, Job>(
            "UPDATE jobs SET title = $1, description = $2, company = $3, location = $4, salary_min = $5, salary_max = $6, job_type = $7, rounds = $8, round_details = $9, experience_min = $10, experience_max = $11, is_remote = $12, application_deadline = $13, updated_at = NOW() WHERE id = $14 RETURNING *"
        )
        .bind(&job.title)
        .bind(&job.description)
        .bind(&job.company)
        .bind(&job.location)
        .bind(job.salary_min)
        .bind(job.salary_max)
        .bind(&job.job_type)
        .bind(job.rounds)
        .bind(round_details_json)
        .bind(job.experience_min)
        .bind(job.experience_max)
        .bind(job.is_remote)
        .bind(job.application_deadline)
        .bind(job.id)
        .fetch_one(&self.db)
        .await?;
        Ok(row)
    }

    async fn delete_job(&self, job_id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM jobs WHERE id = $1")
            .bind(job_id)
            .execute(&self.db)
            .await?;
        Ok(result.rows_affected())
    }

    async fn get_job_count(&self) -> Result<i64, sqlx::Error> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM jobs")
            .fetch_one(&self.db)
            .await?;
        Ok(row.get::<i64, _>("count"))
    }

    async fn get_jobs_by_skill(
        &self,
        skill_id: Uuid,
        page: u32,
        limit: usize,
    ) -> Result<Vec<Job>, sqlx::Error> {
        let offset = (page - 1) * limit as u32;
        let rows = sqlx::query_as::<_, Job>(
            "SELECT j.* FROM jobs j INNER JOIN job_skills js ON j.id = js.job_id WHERE js.skill_id = $1 ORDER BY j.created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(skill_id)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.db)
        .await?;
        Ok(rows)
    }

    async fn get_jobs_by_round_category(
        &self,
        round_category_id: Uuid,
        page: u32,
        limit: usize,
    ) -> Result<Vec<Job>, sqlx::Error> {
        let offset = (page - 1) * limit as u32;
        // Use JSONB array containment operator to check if the UUID is present in stages
        let rows = sqlx::query_as::<_, Job>(
            "SELECT * FROM jobs WHERE round_details->'stages' @> $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(format!("[\"{}\"]", round_category_id))
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.db)
        .await?;
        Ok(rows)
    }

    async fn get_round_categories(&self) -> Result<Vec<RoundCategory>, sqlx::Error> {
        let rows =
            sqlx::query_as::<_, RoundCategory>("SELECT * FROM round_categories ORDER BY name ASC")
                .fetch_all(&self.db)
                .await?;
        Ok(rows)
    }

    async fn get_round_category_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<RoundCategory>, sqlx::Error> {
        let row =
            sqlx::query_as::<_, RoundCategory>("SELECT * FROM round_categories WHERE id = $1")
                .bind(id)
                .fetch_optional(&self.db)
                .await?;
        Ok(row)
    }

    async fn add_skills_to_job(
        &self,
        job_id: Uuid,
        skill_ids: Vec<Uuid>,
    ) -> Result<(), sqlx::Error> {
        for skill_id in skill_ids {
            sqlx::query(
                "INSERT INTO job_skills (job_id, skill_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            )
            .bind(job_id)
            .bind(skill_id)
            .execute(&self.db)
            .await?;
        }
        Ok(())
    }

    async fn remove_skills_from_job(
        &self,
        job_id: Uuid,
        skill_ids: Vec<Uuid>,
    ) -> Result<(), sqlx::Error> {
        for skill_id in skill_ids {
            sqlx::query("DELETE FROM job_skills WHERE job_id = $1 AND skill_id = $2")
                .bind(job_id)
                .bind(skill_id)
                .execute(&self.db)
                .await?;
        }
        Ok(())
    }

    async fn get_skills_of_job(
        &self,
        job_id: Uuid,
    ) -> Result<Vec<crate::features::skills::model::Skill>, sqlx::Error> {
        let rows = sqlx::query_as::<_, crate::features::skills::model::Skill>(
            "SELECT s.* FROM skills s INNER JOIN job_skills js ON s.id = js.skill_id WHERE js.job_id = $1"
        )
        .bind(job_id)
        .fetch_all(&self.db)
        .await?;
        Ok(rows)
    }

    async fn get_jobs_of_skill(&self, skill_id: Uuid) -> Result<Vec<Job>, sqlx::Error> {
        let rows = sqlx::query_as::<_, Job>(
            "SELECT j.* FROM jobs j INNER JOIN job_skills js ON j.id = js.job_id WHERE js.skill_id = $1"
        )
        .bind(skill_id)
        .fetch_all(&self.db)
        .await?;
        Ok(rows)
    }
}
