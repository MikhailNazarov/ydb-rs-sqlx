use sqlx_core::transaction::TransactionManager;

use super::database::Ydb;

pub struct YdbTransactionManager {}

impl TransactionManager for YdbTransactionManager {
    type Database = Ydb;

    fn begin(
        conn: &mut <Self::Database as sqlx_core::database::Database>::Connection,
    ) -> futures_util::future::BoxFuture<'_, Result<(), sqlx_core::Error>> {
        todo!()
    }

    fn commit(
        conn: &mut <Self::Database as sqlx_core::database::Database>::Connection,
    ) -> futures_util::future::BoxFuture<'_, Result<(), sqlx_core::Error>> {
        todo!()
    }

    fn rollback(
        conn: &mut <Self::Database as sqlx_core::database::Database>::Connection,
    ) -> futures_util::future::BoxFuture<'_, Result<(), sqlx_core::Error>> {
        todo!()
    }

    fn start_rollback(conn: &mut <Self::Database as sqlx_core::database::Database>::Connection) {
        todo!()
    }
}
