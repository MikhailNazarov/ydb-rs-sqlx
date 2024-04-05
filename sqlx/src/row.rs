use std::sync::Arc;

use sqlx_core::{column::ColumnIndex, row::Row, Error, HashMap};

use crate::{
    column::YdbColumn,
    statement::YdbStatementMetadata,
    value::{YdbValue, YdbValueFormat, YdbValueRef},
};

use super::database::Ydb;

pub struct YdbRow {
    //pub(crate) data: DataRow,
    pub(crate) format: YdbValueFormat,
    pub(crate) metadata: Arc<YdbStatementMetadata>,
}

impl Row for YdbRow {
    type Database = Ydb;

    fn columns(&self) -> &[<Self::Database as sqlx_core::database::Database>::Column] {
        &self.metadata.columns
    }

    fn try_get_raw<I>(
        &self,
        index: I,
    ) -> Result<<Self::Database as sqlx_core::database::HasValueRef<'_>>::ValueRef, sqlx_core::Error>
    where
        I: sqlx_core::column::ColumnIndex<Self>,
    {
        todo!()
        // let index = index.index(self)?;
        // let column = &self.metadata.columns[index];
        // let value = self.data.get(index);

        // Ok(YdbValueRef {
        //     format: self.format,
        //     row: Some(&self.data.storage),
        //     type_info: column.type_info.clone(),
        //     value,
        // })
    }
}

impl ColumnIndex<YdbRow> for &'_ str {
    fn index(&self, row: &YdbRow) -> Result<usize, Error> {
        row.metadata
            .column_names
            .get(*self)
            .ok_or_else(|| Error::ColumnNotFound((*self).into()))
            .map(|v| *v)
    }
}
