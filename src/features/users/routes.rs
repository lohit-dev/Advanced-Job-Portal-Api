use axum::{
    Router, middleware,
    routing::{delete, get, put},
};

use crate::{features::users::handlers, middleware::auth::auth};

pub fn public_routes() -> Router {
    Router::new()
        .route("/", get(handlers::get_users).post(handlers::create_user))
        .route("/{user_id}", get(handlers::get_user))
        .route("/email/{email}", get(handlers::get_user_by_email))
}

pub fn protected_routes() -> Router {
    Router::new()
        .route("/{user_id}", put(handlers::update_user))
        .route("/{user_id}/role", put(handlers::update_user_role))
        .route("/{user_id}", delete(handlers::delete_user))
        .route_layer(middleware::from_fn(auth))
}

pub fn routes() -> Router {
    Router::new()
        .merge(protected_routes())
        .merge(public_routes())
}
