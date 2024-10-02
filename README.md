# Sqlx intergration for ydb-rs-sdk

This crate provides Sqlx integration for [ydb-rs-sdk](https://github.com/ydb-platform/ydb-rs-sdk). It is in under active development.

## Basic examples

### Simple select
```rust 
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection_string = env::var("YDB_CONNECTION_STRING")?;

    let pool = YdbPoolOptions::new().connect(&connection_string).await?;
    let row: (i32,) = sqlx::query_as("SELECT 1+1").fetch_one(&pool).await?;
    assert_eq!(row.0, 2);

    Ok(())
}
```

### Query with conditions and parse result to struct
```rust

#[derive(sqlx::FromRow)]
struct UserInfo {
    id: u64,
    name: String,
    age: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection_string = env::var("YDB_CONNECTION_STRING")?;
    let pool = YdbPoolOptions::new().connect(&connection_string).await?;

     let users: Vec<UserInfo> =
        sqlx::query_as("SELECT * FROM test2 WHERE age >= $min_age AND age <= $max_age")
            .bind(("min_age", 30))
            .bind(("max_age", 40))
            .fetch_all(&pool)
            .await?;

    assert!(users.len() > 0);

    Ok(())
}
```

## Schema queries

Schema queries should be executed with SchemaExecutor.

You could use `pool.schema()` or `conn.schema()` to get SchemaExecutor:
```rust
sqlx::query("CREATE TABLE test2 (id Uint64 NOT NULL, name Utf8, age UInt8, description Utf8, PRIMARY KEY (id))")
        .execute(pool.schema())
        .await?;
```



## Arguments

There are two binding available:

- default unnamed - with generated name like `$arg_1`
- named by `with_name` function. you can specify name starting with or without dollr sign, but in query you should use $-started name.
    ```rust
        bind(with_name("age", 30))
    ```    
- named by tuple ("name", value) 
    ```rust
        bind(("age", 30))
    ```

Ydb requires that every query params should be declared with `DECLARE` clause like this:

```sql
DECLARE $age AS Uint32;

SELECT * FROM test2 WHERE age > $age;

```

The library is do it for you. You specify only query and bind params to it with `bind` function.

## Checklist

- [x] Connect to ydb
- [x] Default credentials (using fromEnv)
- [x] Custom credentials with options
- [x] Basic query
- [x] Binding parameters
- Support types
    - Numeric
        - [x] Bool	
        - [x] Int8 	
        - [x] Int16 	
        - [x] Int32 	
        - [x] Int64 	
        - [x] Uint8 	
        - [x] Uint16 	
        - [x] Uint32 	
        - [x] Uint64
        - [x] Float 
        - [x] Double 	
        - [ ] Decimal 
    - String types
        - [x] String
        - [x] Utf8
        - [x] Json
        - [x] JsonDocument
        - [x] Yson
        - [ ] Uuid
    - Date and time
        - [x] Date
        - [x] Datetime
        - [x] Timestamp
        - [x] Interval
    - [x] Optional
- [ ] Prepared statements
- [ ] Transactions
- [ ] Compile-type checked queries
- [ ] Migrations
- [ ] Log Statements
