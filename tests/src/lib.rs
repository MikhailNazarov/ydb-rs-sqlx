

pub mod connect;


pub use tokio::net::lookup_host;
pub use ydb_sqlx::database::Ydb;
pub use sqlx::Acquire;
pub use ydb_sqlx::with_name;
pub use sqlx::pool;
pub use ydb_sqlx::YdbPool;

pub async fn connect_local() -> sqlx::Result<YdbPool> {
    let pool = ydb_sqlx::database::Ydb::connect_opts(
        "grpc://localhost:2136?database=/local",
        |opts|opts.log_statements(tracing_log::log::LevelFilter::Info)
    ).await?;
    Ok(pool)
}
#[tokio::test]
pub async fn test_optional_datetime(){

    let pool = connect_local().await.unwrap();

    let mut tr = pool.begin().await.unwrap();
    let conn = tr.acquire().await.unwrap();
   
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS test_opt_dt(
            id Int32 NOT NULL,
            expires_at DateTime,
            PRIMARY KEY (id)
        )
    "#).execute(conn.schema()).await.unwrap();


    sqlx::query(r#"
        insert into test_opt_dt(id, expires_at) values
        (1, CurrentUtcDate()),
        (2, CurrentUtcDate()),
        (3, CurrentUtcDate()),
        (4, NULL)
    "#).execute(&mut *conn).await.unwrap();
}


#[tokio::test]
pub async fn test_optional_string(){
   

    let pool = connect_local().await.unwrap();

    let mut tr = pool.begin().await.unwrap();
    let conn = tr.acquire().await.unwrap();
   
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS test_opt(
            id Int32 NOT NULL,
            title Utf8,
            PRIMARY KEY (id)
        )
    "#).execute(conn.schema()).await.unwrap();
   
    
    sqlx::query(r#"
        insert into test_opt(id, title) values
        (1, 'title1'),
        (2, 'title2'),
        (3, 'title3')
    "#).execute(&mut *conn).await.unwrap();

    let rows = sqlx::query_as::<_,(i32, Option<String>)>(r#"
        select * from test_opt
    "#).fetch_all(&mut *conn).await.unwrap();

    assert_eq!(3, rows.len());
}

pub async fn test_optional_json() {
    let pool = connect_local().await.unwrap();

    let mut tr = pool.begin().await.unwrap();
    let conn = tr.acquire().await.unwrap();

    #[derive(Clone, Default)]
    pub struct UserData {
        pub pet: String,
    }
    let user_data = UserData {
        pet: "Elephant".to_string(),
    };

    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS test_opt(
            id Int32 NOT NULL,
            json Json,
            PRIMARY KEY (id)
        )
    "#).execute(conn.schema()).await.unwrap();

    for id in 1..3 {
        sqlx::query(r#"
            insert into test_opt(id, json) values
            VALUES ( $arg_1, $arg_2)
        "#).bind(id).bind(user_data.clone()).execute(&mut *conn).await.unwrap();
    }
    {
    let row = sqlx::query_as::<_,(i32, Option<UserData>)>(r#"
        select * from test_opt where id = $id
    "#).bind(("id", 1))
    .fetch_one(&mut *conn).await.unwrap();
    
    assert_eq!(row.1, Some(user_data));
    
    }

}

#[tokio::test]
pub async fn test_opt(){
    
   
    let pool = connect_local().await.unwrap();

    let mut tr = pool.begin().await.unwrap();
    let conn = tr.acquire().await.unwrap();
   
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS test_opt(
            id Int32 NOT NULL,
            title Utf8,
            PRIMARY KEY (id)
        )
    "#).execute(conn.schema()).await.unwrap();
   
    
    sqlx::query(r#"
        insert into test_opt(id, title) values
        (1, 'title1'),
        (2, 'title2'),
        (3, 'title3')
    "#).execute(&mut *conn).await.unwrap();
    
    {
    let row = sqlx::query_as::<_,(i32, Option<String>)>(r#"
        select * from test_opt where id = $id
    "#).bind(("id", 1))
    .fetch_one(&mut *conn).await.unwrap();
    
    assert_eq!(row.1, Some("title1".to_string()));
    
    }

}


// #[tokio::test]
// pub async fn test_explain(){
//     use tests::*;
//     let ctx = TestContext::new().await;

//     let mut tr = ctx.pool().begin().await.unwrap();
//     let conn = tr.acquire().await.unwrap();
   
//     sqlx::query(r#"
//         CREATE TABLE IF NOT EXISTS test_opt(
//             id Int32 NOT NULL,
//             title Utf8,
//             PRIMARY KEY (id)
//         )
//     "#).execute(conn.schema()).await.unwrap();
   
    
//     sqlx::query(r#"
//         insert into test_opt(id, title) values
//         (1, 'title1'),
//         (2, 'title2'),
//         (3, 'title3')
//     "#).execute(&mut *conn).await.unwrap();
    
//     {
//      let res = (&mut *conn).describe(r#"
//         declare $id as Int64;
//         select id, title from test_opt where id = $id;
//     "#).await;
   
    
//     // assert!(res.is_ok(),"{}",res.err().unwrap());

//     //let res = (&mut *conn).prepare("select id, title from test_opt where id = 1").await;
   
    
//     assert!(res.is_ok(),"{}",res.err().unwrap());
    
    
//     }

// }
