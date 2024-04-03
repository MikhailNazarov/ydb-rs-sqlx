use sqlx_core::row::Row;

use super::database::Ydb;

pub struct YdbRow {}

impl Row for YdbRow {
    type Database = Ydb;

    fn columns(&self) -> &[<Self::Database as sqlx_core::database::Database>::Column] {
        todo!()
    }

    fn try_get_raw<I>(
        &self,
        index: I,
    ) -> Result<<Self::Database as sqlx_core::database::HasValueRef<'_>>::ValueRef, sqlx_core::Error>
    where
        I: sqlx_core::column::ColumnIndex<Self>,
    {
        todo!()
    }
}
