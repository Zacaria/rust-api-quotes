use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;
use std::env;
use std::fs;

pub async fn init_db_pool() -> Pool<Postgres> {
    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
            let db_port = env::var("DB_PORT").expect("DB_PORT must be set");
            let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
            let db_user = env::var("DB_USER").expect("DB_USER must be set");
            let db_password = env::var("DB_PW_FILE").expect("DB_PW_FILE must be set");

            let password = fs::read_to_string(db_password)
                .expect("Failed to read database password from Docker secret");

            format!(
                "postgres://{}:{}@{}:{}/{}",
                db_user, password, db_host, db_port, db_name
            )
        }
    };

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres")
}
