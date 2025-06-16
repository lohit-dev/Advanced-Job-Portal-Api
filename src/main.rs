use axum::{Extension, Router, http::Method};
use e_commerce::{
    config::Config,
    core::state::build_state,
    features::{auth::routes as auth_routes, users::routes as user_routes},
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let config = Config::load();
    println!("🚀 Starting server...");

    let app_state = Arc::new(build_state(config).await);

    let app = Router::new()
        .nest("/api/auth", auth_routes::routes())
        .nest("/api/users", user_routes::routes())
        .layer(CorsLayer::new().allow_origin(Any).allow_methods([
            Method::GET,
            Method::PUT,
            Method::POST,
            Method::DELETE,
        ]))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(app_state.clone()));

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], app_state.config.app.port));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("🌐 Server listening on http://{}", addr);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();

    Ok(())
}
