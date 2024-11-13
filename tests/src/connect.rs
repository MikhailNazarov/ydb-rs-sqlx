pub use std::env;


#[tokio::test]
pub async fn connect(){
    
    let pool = ydb_sqlx::database::Ydb::connect("grpc://localhost:2136?database=/local").await;

    assert!(pool.is_ok());
}


#[tokio::test]
pub async fn connect_opts(){
   
    let pool = ydb_sqlx::database::Ydb::connect_opts(
        "grpc://localhost:2136?database=/local",
        |opts|opts.log_statements(tracing_log::log::LevelFilter::Info)
    ).await;

    assert!(pool.is_ok());
}



#[tokio::test]
pub async fn connect_env(){

    env::set_var("YDB_CONNECTION_STRING", "grpc://localhost:2136?database=/local");
    
    let pool = ydb_sqlx::database::Ydb::connect_env().await;

    assert!(pool.is_ok());
}

#[tokio::test]
pub async fn connect_env_opts(){
   
    env::set_var("YDB_CONNECTION_STRING", "grpc://localhost:2136?database=/local");
    let pool = ydb_sqlx::database::Ydb::connect_env_opts(
        |opts|opts.log_statements(tracing_log::log::LevelFilter::Info)
    ).await;

    assert!(pool.is_ok());
}
