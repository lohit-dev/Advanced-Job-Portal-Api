use crate::{features::skills::handlers, middleware::auth::auth};
use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};

pub fn public_routes() -> Router {
    Router::new()
        .route("/", get(handlers::get_skills).post(handlers::create_skill))
        .route("/{skill_id}", get(handlers::get_skill))
        .route("/find", get(handlers::get_skill_by_name))
}

pub fn protected_routes() -> Router {
    Router::new()
        .route("/{skill_id}", put(handlers::update_skill))
        .route("/{skill_id}", delete(handlers::delete_skill))
        .route("/user/{user_id}/add", post(handlers::add_skill_to_user))
        .route(
            "/user/{user_id}/remove",
            post(handlers::remove_skill_from_user),
        )
        .route("/user/{user_id}", get(handlers::get_skills_of_user))
        .route("/users/find", get(handlers::get_users_of_skill))
        .route_layer(middleware::from_fn(auth))
}

pub fn routes() -> Router {
    Router::new()
        .merge(protected_routes())
        .merge(public_routes())
}
