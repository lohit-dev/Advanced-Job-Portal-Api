use axum::{
    Router,
    routing::{get, post},
};

use crate::features::auth::handlers::{self, github_callback, github_login};

pub fn public_routes() -> Router {
    Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
        .route("/verify", get(handlers::verify_email))
        .route("/forgot-password", post(handlers::forgot_password))
        .route("/reset-password", post(handlers::reset_password))
}

pub fn oauth_routes() -> Router {
    Router::new()
        .route("/github", get(github_login))
        .route("/github/callback", get(github_callback))
        .route("/google", get(handlers::google_login))
        .route("/google/callback", get(handlers::google_callback))
}

pub fn routes() -> Router {
    Router::new().merge(oauth_routes()).merge(public_routes())
}
