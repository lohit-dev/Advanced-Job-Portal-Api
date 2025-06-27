use axum::routing::get;
use axum::{Extension, Router, http::Method};
use e_commerce::{
    config::Config,
    core::state::build_state,
    features::{
        auth::routes as auth_routes, mail::mails::get_base_template_path,
        skills::routes as skill_routes, users::routes as user_routes,
    },
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv::dotenv().ok();

    let config = Config::load();
    println!("🚀 Starting server...");

    let base_path = get_base_template_path().expect("Unable to get the base path");
    println!("The base path is {:?}", base_path);

    let app_state = Arc::new(build_state(config).await);

    let app = Router::new()
        .nest("/api/", health_routes())
        .nest("/api/auth", auth_routes::routes())
        .nest("/api/users", user_routes::routes())
        .nest("/api/skills", skill_routes::routes())
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

    let health_url = "https://https://advanced-job-portal-api.onrender.com/api/health".to_string();

    // Just for render to keep this alive
    tokio::spawn(async move {
        let client = reqwest::Client::new();
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(300)).await; // 5 minutes
            match client.get(&health_url).send().await {
                Ok(resp) => println!("Health ping: {} ✅", resp.status()),
                Err(err) => eprintln!("Health ping failed: {:?}", err),
            }
        }
    });

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();

    Ok(())
}

pub fn health_routes() -> Router {
    Router::new().route("/health", get(|| async { "Ok" }))
}
