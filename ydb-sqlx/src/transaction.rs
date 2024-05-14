use futures::future::ok;
use sqlx_core::transaction::TransactionManager;
use tracing::error;

use crate::{connection::YdbConnection, error::err_ydb_to_sqlx};

use super::database::Ydb;

pub struct YdbTransactionManager {}

impl TransactionManager for YdbTransactionManager {
    type Database = Ydb;

    fn begin(
        conn: &mut YdbConnection,
    ) -> futures_util::future::BoxFuture<'_, Result<(), sqlx_core::Error>> {
        let tr = conn.table_client().create_interactive_transaction();
        conn.transaction = Some(Box::new(tr));

        Box::pin(ok(()))
    }

    fn commit(
        conn: &mut YdbConnection,
    ) -> futures_util::future::BoxFuture<'_, Result<(), sqlx_core::Error>> {
    
        Box::pin(async move{
            if let Some(tr) = &mut conn.transaction{
                tr.commit().await.map_err(|e| err_ydb_to_sqlx(e))?;
                conn.transaction = None;
                return Ok(());
            }
            Ok(())
        })
    }

    fn rollback(
        conn: &mut YdbConnection,
    ) -> futures_util::future::BoxFuture<'_, Result<(), sqlx_core::Error>> {
        Box::pin(async move{
            if let Some(tr) = &mut conn.transaction{
                tr.rollback().await.map_err(|e| err_ydb_to_sqlx(e))?;
                conn.transaction = None;
                return Ok(());
            }
            Ok(())
        })
    }

    fn start_rollback(conn: &mut YdbConnection) {
        let _ = Box::pin(async move{
            if let Some(tr) = &mut conn.transaction{
                let _ = tr.rollback().await.map_err(|e| err_ydb_to_sqlx(e)).map_err(|e|{
                    error!("{}",e);
                });
                conn.transaction = None;
            }
        });
    }
}
