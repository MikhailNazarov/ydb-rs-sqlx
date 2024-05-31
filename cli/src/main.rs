use std::env;
use sqlx::{migrate::Migrator, Connection};
use ydb_sqlx::connection::YdbConnection;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();
    let connection_string = env::var("YDB_CONNECTION_STRING")
        .expect("YDB_CONNECTION_STRING must be set");
    
    let mut conn = YdbConnection::connect(&connection_string).await?;
   
    let path = std::path::Path::new("./migrations");
    let migrator = Migrator::new(path).await?;
    migrator.run_direct(&mut conn).await?;
    Ok(())
}