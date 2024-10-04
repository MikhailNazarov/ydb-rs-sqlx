use sqlx_core::{connection::ConnectOptions, database::{Database, HasStatementCache}};

use crate::{connection::YdbConnectOptions, value::YdbValueRef, YdbPool, YdbPoolOptions};

use super::{
    arguments::{YdbArgumentBuffer, YdbArguments},
    column::YdbColumn,
    connection::YdbConnection,
    query::YdbQueryResult,
    row::YdbRow,
    statement::YdbStatement,
    transaction::YdbTransactionManager,
    typeinfo::YdbTypeInfo,
    value::YdbValue,
};

#[derive(Debug)]
pub struct Ydb {}

impl Database for Ydb {
    type Connection = YdbConnection;

    type TransactionManager = YdbTransactionManager;

    type Row = YdbRow;

    type QueryResult = YdbQueryResult;

    type Column = YdbColumn;

    type TypeInfo = YdbTypeInfo;

    type Value = YdbValue;

    const NAME: &'static str = "Ydb";

    const URL_SCHEMES: &'static [&'static str] = &["grpcs"];

    type ValueRef<'r> = YdbValueRef<'r>;

    type Arguments<'q> = YdbArguments;

    type ArgumentBuffer<'q> = YdbArgumentBuffer;

    type Statement<'q> = YdbStatement<'q>;
}

impl HasStatementCache for Ydb {}

impl Ydb {
    pub async fn connect_env() -> Result<YdbPool, sqlx_core::Error> {
        let options = YdbConnectOptions::from_env()?;
        let pool = YdbPoolOptions::new().connect_with(options).await?;
        Ok(pool)
    }

    pub async fn connect_env_opts(opts: impl Fn(YdbConnectOptions)->YdbConnectOptions) -> Result<YdbPool, sqlx_core::Error> {
        let options = YdbConnectOptions::from_env()?;
        let options = opts(options);
        let pool = YdbPoolOptions::new().connect_with(options).await?;
        Ok(pool)
    }
   

    pub async fn connect(db_url: &str)->Result<YdbPool,sqlx_core::Error>{
        let url = url::Url::parse(&db_url)
            .map_err(|e| sqlx_core::Error::Configuration(e.into()))?;
        let options = YdbConnectOptions::from_url(&url)?;
        let pool = YdbPoolOptions::new().connect_with(options).await?;
        Ok(pool)
    }

    pub async fn connect_opts(db_url: &str, opts: impl Fn(YdbConnectOptions)->YdbConnectOptions)->Result<YdbPool,sqlx_core::Error>{
        let url = url::Url::parse(&db_url)
            .map_err(|e| sqlx_core::Error::Configuration(e.into()))?;
        let options = YdbConnectOptions::from_url(&url)?;
        let options = opts(options);
        let pool = YdbPoolOptions::new().connect_with(options).await?;
        Ok(pool)
    }
}
