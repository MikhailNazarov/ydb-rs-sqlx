use sqlx_core::column::Column;

use super::database::Ydb;

#[derive(Debug)]
pub struct YdbColumn {}

impl Column for YdbColumn {
    type Database = Ydb;

    fn ordinal(&self) -> usize {
        todo!()
    }

    fn name(&self) -> &str {
        todo!()
    }

    fn type_info(&self) -> &<Self::Database as sqlx_core::database::Database>::TypeInfo {
        todo!()
    }
}
