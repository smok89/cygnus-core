use std::env;
use sea_orm::{Database, DatabaseConnection};
use dotenv::dotenv;

pub async fn connect_db() -> DatabaseConnection {
dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(&database_url).await.expect("Failed to connect to the database")
}
