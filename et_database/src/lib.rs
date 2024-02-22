use sqlx::sqlite::{SqlitePoolOptions};
use std::{env, fs};
use std::env::VarError;
use std::path::Path;
use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};
use tokio;


pub async fn run_database_connection() {
    create_db().await;
}

async fn create_db() {
    let home_dir = dirs::home_dir().expect("Could not find the home directory");
    // Bind the result of join to a variable
    let db_path_buf = home_dir.join(".et/my_database.db");
    // Now, you can borrow from db_path_buf safely
    let db_path = db_path_buf.to_str().unwrap();

    // Ensure the directory exists
    ensure_db_directory_exists(&db_path_buf).await;

    match env::var("DATABASE_URL") {
        Ok(db_url) => {
            if !Sqlite::database_exists(db_path).await.unwrap_or(false) {
                println!("Creating database {}", db_path);
                match Sqlite::create_database(db_path).await {
                    Ok(_) => println!("Create db success"),
                    Err(error) => panic!("error: {}", error),
                }
            } else {
                println!("Database already exists");
            }

            let pool = SqlitePool::connect(&db_path).await.unwrap();
            // Run migrations located in the "migrations" directory relative to the project root
            sqlx::migrate!().run(&pool).await.unwrap();
            // let result = sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY NOT NULL, name VARCHAR(250) NOT NULL);").execute(&db).await.unwrap();
            // println!("Create user table result: {:?}", result);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

async fn ensure_db_directory_exists(db_path: &Path) {
    if let Some(parent_dir) = db_path.parent() {
        if !parent_dir.exists() {
            match fs::create_dir_all(parent_dir) {
                Ok(_) => println!("Directory created successfully."),
                Err(e) => panic!("Failed to create directory: {}", e),
            }
        }
    }
}