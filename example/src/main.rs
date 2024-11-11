use std::time::SystemTime;
use tracing::info;
use tracing_log::log::LevelFilter;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use ydb_sqlx::database::Ydb;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    init_logs();
    
    let pool = Ydb::connect_env_opts(
        |opt|opt
            .with_stats(ydb_sqlx::connection::StatsMode::Full)
            .log_statements(LevelFilter::Info)
    ).await?;

    let row: (i32,) = sqlx::query_as("SELECT 1+1").fetch_one(&pool).await?;
    assert_eq!(row.0, 2);

    let conn = pool.acquire().await?;
    sqlx::query("CREATE TABLE test4 (id Uint64 NOT NULL, name Utf8, age UInt8 NOT NULL, description Utf8, updated_at Timestamp, PRIMARY KEY (id))")
        .execute(conn.schema())
        .await?;

    let test_user_info = UserInfo {
        id: 13u64,
        name: "test".to_string(),
        age: 32u8,
        description: None,
        updated_at: None
    };

    let res = sqlx::query("DELETE FROM test4 where id = $id")
        .bind(("id",test_user_info.id))
        .execute(&pool)
        .await?;
    info!("rows affected: {}", res.rows_affected());


    sqlx::query("INSERT INTO test4 (id, name, age, description, updated_at) VALUES ( $arg_1, $arg_2, $age, $arg_3, CurrentUtcDateTime())")
        .bind(test_user_info.id)
        .bind(test_user_info.name)
        .bind(("age", test_user_info.age))
        .bind(test_user_info.description)
        .execute(&pool)
        .await?;

    let users: Vec<UserInfo> =
        sqlx::query_as("SELECT * FROM test4 WHERE age > $age AND age < $arg_1")
            .bind(("age", 30))
            .bind(40)
            .fetch_all(&pool)
            .await?;

    assert!(!users.is_empty());
    info!("users found: {}", users.len());

    Ok(())
}

#[allow(unused)]
#[derive(sqlx::FromRow)]
struct UserInfo {
    id: u64,
    name: String,
    age: u8,
    description: Option<String>,
    updated_at: Option<SystemTime>
}

fn init_logs() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
}
