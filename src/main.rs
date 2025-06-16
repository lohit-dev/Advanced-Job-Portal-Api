use e_commerce::{
    config::{database::init_db, Config},
    features::users::{model::User, repository::UserRepository, service::UserService},
};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    // Load configuration
    let config = Config::load();
    println!("Attempting to connect to database...");

    // Initialize database connection with SSL
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database.database_url)
        .await
        .map_err(|e| {
            eprintln!("❌ Database connection error: {}", e);
            e
        })?;

    println!("✅ Successfully connected to NeonDB!");

    let user_service = UserService { db: pool };
    match user_service
        .get_users(1, 10).await{
            Ok(S)=>{println!("User : {:#?}",S)}
            Err(_) => println!("No users"),
        }
    Ok(())
}
