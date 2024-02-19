use sqlx::sqlite::{SqlitePoolOptions};
use std::env;
use std::env::VarError;
use sqlx::migrate::MigrateDatabase;
use sqlx::Sqlite;
use tokio;

pub async fn run_database_connection() {
    create_db().await;
}

async fn create_db() {
    match env::var("DATABASE_URL") {
        Ok(db_url) => {
            if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
                println!("Creating database {}", db_url);
                match Sqlite::create_database(&db_url).await {
                    Ok(_) => println!("Create db success"),
                    Err(error) => panic!("error: {}", error),
                }
            } else {
                println!("Database already exists");
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}