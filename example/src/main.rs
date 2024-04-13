use std::{env, str::FromStr};

use tracing::Level;
use ydb_rs_sqlx::YdbPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::error::Error> {
    init_logs();
    let connection_string = env::var("YDB_CONNECTION_STRING").unwrap();

    let pool = YdbPoolOptions::new().connect(&connection_string).await?;
    let row: (i32,) = sqlx::query_as("SELECT 1+1").fetch_one(&pool).await?;
    assert_eq!(row.0, 2);

    let users: Vec<UserInfo> = sqlx::query_as("SELECT * FROM test2 WHERE age > $age")
        .bind(with_name("age", 30))
        .fetch_all(&pool)
        .await?;

    assert!(users.len() > 0);

    Ok(())
}

#[derive(sqlx::FromRow)]
struct UserInfo {
    id: u64,
    name: String,
    age: u32,
}

fn init_logs() {
    let level = env::var("RUST_LOG").unwrap_or("INFO".to_string());
    let log_level = Level::from_str(&level).unwrap();
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(log_level)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Error setting subscriber");
}
