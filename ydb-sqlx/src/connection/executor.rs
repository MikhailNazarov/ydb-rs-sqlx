
use futures::pin_mut;
use futures::Stream;
use futures::TryStreamExt;
use futures_core::stream::BoxStream;
use futures::StreamExt;
use itertools::Either;
use sqlx_core::executor::Execute;
use sqlx_core::executor::Executor;
use sqlx_core::logger::QueryLogger;
use sqlx_core::try_stream;
use sqlx_core::Error;
use tracing::info;
use crate::error::err_ydb_or_customer_to_sqlx;

use crate::error::err_ydb_to_sqlx;
use crate::query::build_query;
use crate::query::ParsedQuery;
use crate::statement::YdbStatement;
use crate::typeinfo::YdbTypeInfo;
use crate::{database::Ydb, query::YdbQueryResult, row::YdbRow};
use sqlx_core::describe::Describe;

use super::StatsMode;
use super::YdbConnection;

impl YdbConnection {

    pub(crate) async fn run<'e, 'c: 'e, 'q: 'e>( &'c mut self,
        query: ParsedQuery        
    ) ->  Result<impl Stream<Item = Result<Either<YdbQueryResult, YdbRow>, Error>> + 'e, Error> {

        let result = Box::pin(async move {
            if let Some(tr) = &mut self.transaction {

                let mut logger = QueryLogger::new(query.sql(), self.log_settings.clone());
                let query: ydb::Query = query.clone().into();

                let query = match &self.stats_mode {
                    StatsMode::None => {query},
                    mode => { query.with_stats(mode.into()) }
                };

                let result = tr.query(query).await.map_err(err_ydb_to_sqlx)?;
                
                let rows_affected = if let Some(stats) = result.stats(){
                    
                    //info!("{:?}", stats);
                    stats.affected_rows
                }else{
                    0
                };
                let rows =result.rows_len();
                
                let results = result.into_results();
                for _ in 1..=rows{
                    logger.increment_rows_returned();    
                }              
                
                logger.increase_rows_affected(rows_affected);
                Ok(Some(results))
            } else {                
                self.client
                    .table_client()
                    .retry_transaction(|t| async {

                        let mut logger = QueryLogger::new(query.sql(), self.log_settings.clone());
                        let query: ydb::Query = query.clone().into();
                        let query = match &self.stats_mode {
                            StatsMode::None => {query},
                            mode => { query.with_stats(mode.into()) }
                        };

                        let mut t = t;
                        let result = t.query(query.clone()).await?;
                        let rows =result.rows_len();
                        //info!("rows: {}", rows);

                        for _ in 1..=rows{
                            logger.increment_rows_returned();    
                        }
                       
                        let rows_affected = if let Some(stats) = result.stats(){
                    
                           // info!("{:?}", stats);
                            stats.affected_rows
                        }else{
                            0
                        };
                        
                        t.commit().await?;
                        
                        logger.increase_rows_affected(rows_affected);
                        Ok(Some(result.into_results()))
                    })
                    .await
                    .map_err( err_ydb_or_customer_to_sqlx)
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
                    .flat_map(|rs| rs.rows())
                    .map(|r| match YdbRow::from(r) {
                        Ok(r) => Ok(Either::Right(r)),
                        Err(e) => Err(e),
                    })
                    .chain(err);
                

                futures::stream::iter(rows)
            })
            .flatten();

        Ok(Box::pin(stream))        
    }
}



impl<'c> Executor<'c> for &'c mut YdbConnection {
    type Database = Ydb;

    fn fetch_many<'e, 'q: 'e, E: 'q + Execute<'q, Ydb>>(
        self,
        query: E,
    ) -> BoxStream<'e, Result<sqlx_core::Either<YdbQueryResult, YdbRow>, Error>>
    where
        'c: 'e
    {

        Box::pin(try_stream! {
            let query = build_query(query)?;
            let s = self.run(query).await?;
            pin_mut!(s);

            while let Some(v) = s.try_next().await? {
                r#yield!(v);
            }
            Ok(())
        })
    }

    fn fetch_optional<'e, 'q: 'e, E: 'q + Execute<'q, Ydb>>(
        self,
        query: E,
    ) -> futures::future::BoxFuture<'e, Result<Option<YdbRow>, Error>>
    where
        'c: 'e
    {
        
        Box::pin(async move {
            let query = build_query(query)?;
            let s = self.run(query).await?;
            pin_mut!(s);

            // With deferred constraints we need to check all responses as we
            // could get a OK response (with uncommitted data), only to get an
            // error response after (when the deferred constraint is actually
            // checked).
            let mut ret = None;
            while let Some(result) = s.try_next().await? {
                match result {
                    Either::Right(r) if ret.is_none() => ret = Some(r),
                    _ => {}
                }
            }
            Ok(ret)
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
            .map_err(err_ydb_to_sqlx)?;
            println!("prepare_result: {:?}", res);
            todo!()
            // Ok(YdbStatement {
            //     sql: todo!(),
            //     metadata: todo!(),
            // })
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
            .map_err(err_ydb_to_sqlx)?;
            println!("explain_result: {:?}", explain_result);

            todo!()
            // Ok(Describe::<Ydb>{
            //     columns: vec![],
            //     parameters: todo!(),
            //     nullable: todo!(),   
            // })
        })
       
        //self.client.table_client().explain_data_query()
    }
}
