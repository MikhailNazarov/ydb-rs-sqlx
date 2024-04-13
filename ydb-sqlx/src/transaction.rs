use sqlx_core::transaction::TransactionManager;

use crate::connection::YdbConnection;

use super::database::Ydb;

pub struct YdbTransactionManager {}

impl TransactionManager for YdbTransactionManager {
    type Database = Ydb;

    fn begin(
        _conn: &mut YdbConnection,
    ) -> futures_util::future::BoxFuture<'_, Result<(), sqlx_core::Error>> {
        todo!()
    }

    fn commit(
        _conn: &mut YdbConnection,
    ) -> futures_util::future::BoxFuture<'_, Result<(), sqlx_core::Error>> {
        todo!()
    }

    fn rollback(
        _conn: &mut YdbConnection,
    ) -> futures_util::future::BoxFuture<'_, Result<(), sqlx_core::Error>> {
        todo!()
    }

    fn start_rollback(_conn: &mut YdbConnection) {
        todo!()
    }
}
