use crate::YdbPool;
use crate::{database::Ydb, error::err_ydb_to_sqlx};
use crate::query::YdbQueryResult;
use crate::row::YdbRow;
use crate::statement::YdbStatement;
use crate::typeinfo::YdbTypeInfo;
use futures::future::BoxFuture;
use sqlx_core::connection::LogSettings;
use sqlx_core::describe::Describe;
use sqlx_core::executor::{Execute, Executor};
use sqlx_core::logger::QueryLogger;
use sqlx_core::Error;
use ydb::TableClient;
use std::fmt::{self, Debug};

use super::YdbConnection;


pub struct YdbSchemaExecutor {
    inner: YdbSchemaExecutorInner,
    log_settings: LogSettings,
}

pub enum YdbSchemaExecutorInner {
    Client(TableClient),
    Pool(YdbPool)
}

impl YdbSchemaExecutor{
    pub(crate) fn new(connection: &YdbConnection) -> Self {
        Self{
            inner: YdbSchemaExecutorInner::Client(connection.table_client()),
            log_settings: connection.log_settings.clone()
        }
    }
   
    pub(crate) fn from_pool(pool: YdbPool) -> Self {
        let log_settings = pool.connect_options().log_settings.clone();
        Self{
            inner: YdbSchemaExecutorInner::Pool(pool),
            log_settings
        }
    }
}
impl Debug for YdbSchemaExecutor{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("YdbSchemaExecutor")
    }
}

impl<'c> Executor<'c> for YdbSchemaExecutor {
    type Database = Ydb;

    fn execute<'e, 'q: 'e, E: 'q + Execute<'q, Ydb>>(self, query: E) -> BoxFuture<'e, Result<YdbQueryResult, Error>>
    where
        'c: 'e{
        
        Box::pin(async move{
            let _logger = QueryLogger::new(query.sql(), self.log_settings.clone());
            match self.inner {
                YdbSchemaExecutorInner::Client(table_client) => {
                    table_client.retry_execute_scheme_query(query.sql()).await
                        .map_err(err_ydb_to_sqlx)?;
                    }
                YdbSchemaExecutorInner::Pool(pool) => {
                    pool.acquire().await?
                        .table_client()
                        .retry_execute_scheme_query(query.sql()).await
                        .map_err( err_ydb_to_sqlx)?;
                }
            }
             
            
            Ok(YdbQueryResult::default())
        })

    }

    fn fetch_many<'e, 'q: 'e, E: 'q + Execute<'q, Ydb>>(
        self,
        _query: E,
    ) -> futures::prelude::stream::BoxStream<
        'e,
        Result<itertools::Either<YdbQueryResult, YdbRow>, Error>,
    >
    where
        'c: 'e
    {
        unimplemented!()
    }

    fn fetch_optional<'e, 'q: 'e, E: 'q + Execute<'q, Ydb>>(
        self,
        _query: E,
    ) -> futures::prelude::future::BoxFuture<'e, Result<Option<YdbRow>, Error>>
    where
        'c: 'e
    {
        unimplemented!()
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        _sql: &'q str,
        _parameters: &'e [YdbTypeInfo],
    ) -> BoxFuture<'e, Result<YdbStatement<'q>, Error>>
    where
        'c: 'e,
    {
        unimplemented!()
    }

    fn describe<'e, 'q: 'e>(self, _sql: &'q str) -> BoxFuture<'e, Result<Describe<Ydb>, Error>>
    where
        'c: 'e,
    {
        unimplemented!()
    }
}
