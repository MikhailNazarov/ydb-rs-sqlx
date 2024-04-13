use futures_core::stream::BoxStream;

use sqlx_core::executor::Execute;
use sqlx_core::executor::Executor;

use sqlx_core::Error;
use ydb::Query;
use ydb::YdbOrCustomerError;

use crate::error::err_ydb_or_customer_to_sqlx;

use crate::statement::YdbStatement;
use crate::typeinfo::YdbTypeInfo;
use crate::{database::Ydb, query::YdbQueryResult, row::YdbRow};

use super::YdbConnection;

impl<'c> Executor<'c> for &'c mut YdbConnection {
    type Database = Ydb;

    fn fetch_many<'e, 'q: 'e, E: 'q>(
        self,
        _query: E,
    ) -> BoxStream<'e, Result<sqlx_core::Either<YdbQueryResult, YdbRow>, Error>>
    where
        'c: 'e,
        E: Execute<'q, Ydb>,
    {
        todo!()
    }

    fn fetch_optional<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> futures_core::future::BoxFuture<'e, Result<Option<YdbRow>, Error>>
    where
        'c: 'e,
        E: Execute<'q, Ydb>,
    {
        Box::pin(async move {
            let query = Query::new(query.sql().to_string());
            self.client
                .table_client()
                .retry_transaction(|t| async {
                    //YdbRow::from(row)
                    let mut t = t;
                    let x = t.query(query.clone()).await?;

                    if let Some(row) = x.into_only_row().ok() {
                        let row = YdbRow::from(row).map_err(|e| YdbOrCustomerError::from_err(e))?;
                        Ok(Some(row))
                    } else {
                        Ok(None)
                    }
                })
                .await
                .map_err(|e| err_ydb_or_customer_to_sqlx(e))
        })
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        _sql: &'q str,
        _parameters: &'e [YdbTypeInfo],
    ) -> futures_core::future::BoxFuture<'e, Result<YdbStatement<'q>, Error>>
    where
        'c: 'e,
    {
        todo!()
    }

    fn describe<'e, 'q: 'e>(
        self,
        _sql: &'q str,
    ) -> futures_core::future::BoxFuture<'e, Result<sqlx_core::describe::Describe<Ydb>, Error>>
    where
        'c: 'e,
    {
        todo!()
    }
}
