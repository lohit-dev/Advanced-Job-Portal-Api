use axum::{
    Router,
    routing::{get, post},
};

use crate::features::auth::handlers;

pub fn public_routes() -> Router {
    Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
        .route("/verify", get(handlers::verify_email))
        .route("/forgot-password", post(handlers::forgot_password))
        .route("/reset-password", post(handlers::reset_password))
        // .route("/google", get(handlers::google_login))
        // .route("/google/callback", get(handlers::google_callback))
}

pub fn routes() -> Router {
    Router::new().merge(public_routes())
}

