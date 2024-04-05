use std::{env, str::FromStr};

use tracing::{info, Level};
use ydb_rs_sqlx::{database::Ydb, YdbPoolOptions};

#[tokio::main]
async fn main() -> Result<(), sqlx_core::error::Error> {
    init_logs();
    let connection_string = env::var("YDB_CONNECTION_STRING").unwrap();
    // let options = YdbConnectOptions::from_str(&connection_string)?;
    // options.connect().await?;
    //info!("connected");

    let pool = YdbPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await?;
    info!("connected");
    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL/MariaDB)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);

    Ok(())
}

fn init_logs() {
    let level = env::var("RUST_LOG").unwrap_or("INFO".to_string());
    let log_level = Level::from_str(&level).unwrap();
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(log_level)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Error setting subscriber");
}
