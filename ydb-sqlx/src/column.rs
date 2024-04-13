use sqlx_core::column::Column;

use crate::typeinfo::YdbTypeInfo;

use super::database::Ydb;

#[derive(Debug)]
pub struct YdbColumn {
    name: String,
    ordinal: usize,
    type_info: YdbTypeInfo,
}
impl YdbColumn {
    pub(crate) fn new(column: ydb::Column) -> Self {
        Self {
            name: column.name,
            ordinal: column.ordinal,
            type_info: YdbTypeInfo::new(column.value_type),
        }
    }
}

impl Column for YdbColumn {
    type Database = Ydb;

    fn ordinal(&self) -> usize {
        self.ordinal
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn type_info(&self) -> &YdbTypeInfo {
        &self.type_info
    }
}
