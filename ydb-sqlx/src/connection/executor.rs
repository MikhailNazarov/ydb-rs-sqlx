use std::collections::HashMap;

use futures::future::BoxFuture;
use futures_core::stream::BoxStream;

use futures::StreamExt;
use itertools::Either;
use rustring_builder::StringBuilder;
use sqlx_core::executor::Execute;
use sqlx_core::executor::Executor;

use sqlx_core::Error;
use tracing::debug;
use tracing::info;
use ydb::Query;
use ydb::YdbOrCustomerError;

use crate::error::err_ydb_or_customer_to_sqlx;

use crate::error::err_ydb_to_sqlx;
use crate::statement::YdbStatement;
use crate::typeinfo::YdbTypeInfo;
use crate::{database::Ydb, query::YdbQueryResult, row::YdbRow};
use sqlx_core::describe::Describe;

use super::YdbConnection;

fn build_query<'q, E: 'q>(mut query: E) -> Query
where
    E: Execute<'q, Ydb>,
{
    let mut sb = StringBuilder::new();
    let mut params = HashMap::new();

    if let Some(arguments) = query.take_arguments() {
        for arg in arguments.into_iter() {
            arg.declare(&mut sb);
            arg.add_to_params(&mut params);
        }
        sb.append_line("");
    }

    sb.append(query.sql());

    let sql = sb.to_string();
    //info!("{}", sql);

    let mut query = Query::new(sql);
    if !params.is_empty() {
        //info!("PARAMS: {:?}", params);
        query = query.with_params(params);
    }
    
    query
}

// impl YdbConnection{
//     pub(crate) async fn execute_scheme<Q: Into<String>>(&mut self, query: Q) -> Result<(), Error> {
        
//         self.client.table_client().retry_execute_scheme_query(query).await.map_err(|e| err_ydb_to_sqlx(e))?;

//         Ok(())
//     }
// }

impl<'c> Executor<'c> for &'c mut YdbConnection {
    type Database = Ydb;

    fn execute<'e, 'q: 'e, E: 'q>(
            self,
            query: E,
        ) -> BoxFuture<'e, Result<YdbQueryResult, Error>>
        where
            'c: 'e,
            E: Execute<'q, Self::Database>, {

        Box::pin(async move{
            //debug!("{}",query.sql());
            let query = build_query(query);
            let _result = if let Some(tr) = &mut self.transaction {
                
                 tr.query(query.clone()).await.map_err(|e| err_ydb_to_sqlx(e))?
                
            }else{
                self.client
                .table_client()
                .retry_transaction(|t| async {
                    let mut t = t;
                    let result = t.query(query.clone()).await?;
                    t.commit().await?;
                    Ok(result)
                })
                .await.map_err(|e| err_ydb_or_customer_to_sqlx(e))?
            };

            Ok(YdbQueryResult{
               rows_affected: 0 //todo!
            })
        })
        
    }

    fn fetch_many<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> BoxStream<'e, Result<sqlx_core::Either<YdbQueryResult, YdbRow>, Error>>
    where
        'c: 'e,
        E: Execute<'q, Ydb>,
    {
    
        let result = Box::pin(async move {
            let query = build_query(query);
            if let Some(tr) = &mut self.transaction {
                
                let result = tr.query(query.clone()).await.map_err(|e| err_ydb_to_sqlx(e))?;
                Ok(Some(result.into_results()))
            } else {
                
            self.client
                .table_client()
                .retry_transaction(|t| async {
                    let mut t = t;
                    let result = t.query(query.clone()).await?;
                    t.commit().await?;
                    Ok(Some(result.into_results()))
                })
                .await
                .map_err(|e| err_ydb_or_customer_to_sqlx(e))
            }
        });
        let stream = futures::stream::once(result)
            .map(|r| {
                let mut err = Vec::with_capacity(1);

                let results = match r {
                    Ok(rs) => rs.unwrap_or_default(),
                    Err(e) => {
                        err.push(Err(e));
                        vec![]
                    }
                };

                let rows = results
                    .into_iter()
                    .map(|rs| rs.rows().into_iter())
                    .flatten()
                    .map(|r| match YdbRow::from(r) {
                        Ok(r) => Ok(Either::Right(r)),
                        Err(e) => Err(e),
                    })
                    .chain(err);

                futures::stream::iter(rows)
            })
            .flatten();

        Box::pin(stream)
    }

    fn fetch_optional<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> futures::future::BoxFuture<'e, Result<Option<YdbRow>, Error>>
    where
        'c: 'e,
        E: Execute<'q, Ydb>,
    {
        Box::pin(async move {
            let query = build_query(query);
            if let Some(tr) = &mut self.transaction {
                let result = tr.query(query.clone()).await
                .map_err(|e| err_ydb_to_sqlx(e))?;

                    if let Some(row) = result.into_only_row().ok() {
                        let row = YdbRow::from(row)?;
                        Ok(Some(row))
                    } else {
                        Ok(None)
                    }
            }else{
            self.client
                .table_client()
                .retry_transaction(|t| async {
                    //YdbRow::from(row)
                    let mut t = t;
                    let result = t.query(query.clone()).await?;
                    t.commit().await?;
                    if let Some(row) = result.into_only_row().ok() {
                        let row = YdbRow::from(row).map_err(|e| YdbOrCustomerError::from_err(e))?;
                        Ok(Some(row))
                    } else {
                        Ok(None)
                    }
                })
                .await
                .map_err(|e| err_ydb_or_customer_to_sqlx(e))
            }
            
        })
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        _parameters: &'e [YdbTypeInfo],
    ) -> futures::future::BoxFuture<'e, Result<YdbStatement<'q>, Error>>
    where
        'c: 'e,
    {
        Box::pin(async move {
            let res = self.client.table_client().prepare_data_query(sql.to_owned()).await
            .map_err(|e| err_ydb_to_sqlx(e))?;
            println!("prepare_result: {:?}", res);
            Ok(YdbStatement {
                sql: todo!(),
                metadata: todo!(),
            })
        })
    }

    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> futures::future::BoxFuture<'e, Result<Describe<Ydb>, Error>>
    where
        'c: 'e,
    {
        Box::pin( async move {
            let explain_result = self.client.table_client().explain_data_query(sql.to_owned()).await
            .map_err(|e| err_ydb_to_sqlx(e))?;
            println!("explain_result: {:?}", explain_result);

            Ok(Describe::<Ydb>{
                columns: vec![],
                parameters: todo!(),
                nullable: todo!(),   
            })
        })
       
        //self.client.table_client().explain_data_query()
    }
}
