use sqlx_core::column::Column;

use crate::typeinfo::YdbTypeInfo;

use super::database::Ydb;

#[derive(Debug)]
pub struct YdbColumn {
    pub(crate) name: String,
    pub(crate) ordinal: usize,
    pub(crate) type_info: YdbTypeInfo,
}

impl Column for YdbColumn {
    type Database = Ydb;

    fn ordinal(&self) -> usize {
        self.ordinal
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn type_info(&self) -> &<Self::Database as sqlx_core::database::Database>::TypeInfo {
        &self.type_info
    }
}
