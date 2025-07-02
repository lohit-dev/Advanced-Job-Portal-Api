use crate::{features::jobs::handlers, middleware::auth::auth};
use axum::{
    Router, middleware,
    routing::{delete, get, put},
};

pub fn public_routes() -> Router {
    Router::new()
        .route("/", get(handlers::get_jobs).post(handlers::create_job))
        .route("/{job_id}", get(handlers::get_job))
}

pub fn protected_routes() -> Router {
    Router::new()
        .route("/{job_id}", put(handlers::update_job))
        .route("/{job_id}", delete(handlers::delete_job))
        .route(
            "/{job_id}/skills",
            get(handlers::get_skills_of_job)
                .post(handlers::add_skills_to_job)
                .delete(handlers::remove_skills_from_job),
        )
        .route("/skills/{skill_id}", get(handlers::get_jobs_of_skill))
        .route_layer(middleware::from_fn(auth))
}

pub fn routes() -> Router {
    Router::new()
        .merge(protected_routes())
        .merge(public_routes())
}
