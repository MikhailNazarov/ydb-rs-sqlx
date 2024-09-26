use std::{env, str::FromStr};

use tracing::{info, Level};

use ydb_sqlx::{with_name, YdbPoolOptions};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logs();
    let connection_string = env::var("YDB_CONNECTION_STRING").unwrap_or_else(|_| "grpc://localhost:2136?database=/local".to_string());
    
    let pool = YdbPoolOptions::new()
        .connect(&connection_string).await?;
    let row: (i32,) = sqlx::query_as("SELECT 1+1").fetch_one(&pool).await?;
    assert_eq!(row.0, 2);

    // let conn = pool.acquire().await?;
    // sqlx::query("CREATE TABLE test2 (id Uint64 NOT NULL, name Utf8, age UInt32 NOT NULL, description Utf8, PRIMARY KEY (id))")
    //     .execute(conn.schema())
    //     .await?;

    let test_user_info = UserInfo {
        id: 1,
        name: "test".to_string(),
        age: 33,
        description: None
    };

    


    sqlx::query("INSERT INTO test2 (id, name, age, description) VALUES ( $arg_1, $arg_2, $age, $arg_3)")
        .bind(test_user_info.id)
        .bind(test_user_info.name)
        .bind(with_name("age", test_user_info.age))
        .bind(test_user_info.description)
        .execute(&pool)
        .await?;

    let users: Vec<UserInfo> =
        sqlx::query_as("SELECT * FROM test2 WHERE age > $age AND age < $arg_1")
            .bind(with_name("age", 30))
            .bind(40)
            .fetch_all(&pool)
            .await?;

    assert!(users.len() > 0);
    info!("users found: {}", users.len());

    Ok(())
}

#[allow(unused)]
#[derive(sqlx::FromRow)]
struct UserInfo {
    id: u64,
    name: String,
    age: u32,
    description: Option<String>,
}

fn init_logs() {
    let level = env::var("RUST_LOG").unwrap_or("INFO".to_string());
    let log_level = Level::from_str(&level).unwrap();
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(log_level)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Error setting subscriber");
}
