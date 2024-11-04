

pub mod connect;

#[cfg(test)]
pub mod tests{
    pub use ydb_sqlx::database::Ydb;
    pub use sqlx::Acquire;
    pub use ydb_sqlx::with_name;
    pub use sqlx::pool;
}

#[tokio::test]
pub async fn test_optional_string(){
   

    use tests::*;
    let pool = Ydb::connect_env().await.unwrap();

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

#[tokio::test]
pub async fn test_opt(){
    use tests::*;
   
    let pool = Ydb::connect_env().await.unwrap();

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
    "#).bind(with_name("id", 1))
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
