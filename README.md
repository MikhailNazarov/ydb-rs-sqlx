# ydb-sqlx - Sqlx intergration for ydb-rs-sdk

This crate provides Sqlx integration for ydb-rs-sdk. It is in under active development.

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
        sqlx::query_as("SELECT * FROM test2 WHERE age > $age AND age < $arg_1")
            .bind(with_name("age", 30))
            .bind(40)
            .fetch_all(&pool)
            .await?;

    assert!(users.len() > 0);

    Ok(())
}
```

## Arguments

There are two binding available:

- default unnamed - with generated name like `$arg_1`
- named by `with_name` function. you can specify name starting with or without $, but in query you should yous $-started name.

Ydb requires that every query params should be declared with `DECLARE` clause like this:

```sql
DECLARE $age AS Uint32;

SELECT * FROM test2 WHERE age > $age;

```

The library is do it for you. You specify only query and bind params to id with `bind` function.

## Checklist

- [x] Connect to ydb
- [x] Default credentials (using fromEnv)
- [ ] Custom credentials with options
- [x] Basic query
- [x] Binding parameters
- Support binding for types
    - Primitive types
      - [x] bool, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64
      - [x] String 