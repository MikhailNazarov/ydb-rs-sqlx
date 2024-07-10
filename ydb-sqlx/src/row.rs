use crate::{
    column::YdbColumn,
    value::{YdbValue, YdbValueRef},
};

use sqlx_core::{column::Column, column::ColumnIndex, row::Row, Error};
use tracing::info;

use super::database::Ydb;

pub struct YdbRow {
    values: Vec<YdbValue>,
    columns: Vec<YdbColumn>,
}

impl YdbRow {
    pub(crate) fn from(mut row: ydb::Row) -> Result<Self, Error> {
        let columns = row
            .columns()
            .into_iter()
            .map(|c| YdbColumn::new(c))
            .collect::<Vec<_>>();

        let mut values = vec![];
        for column in &columns {
            let value = row
                .remove_field(column.ordinal())
                .map_err(|e| Error::ColumnDecode {
                    index: column.name().to_owned(),
                    source: Box::new(e),
                })?;
            
            values.push(YdbValue::new(value, column.type_info().clone()));
        }

        Ok(Self { values, columns })
    }
}

impl Row for YdbRow {
    type Database = Ydb;

    fn columns(&self) -> &[YdbColumn] {
        &self.columns
    }

    fn try_get_raw<I>(&self, index: I) -> Result<YdbValueRef, sqlx_core::Error>
    where
        I: sqlx_core::column::ColumnIndex<Self>,
    {
        let index = index.index(self)?;
        let value = self.values.get(index).unwrap();
        Ok(YdbValueRef::new(value))
    }
}

impl ColumnIndex<YdbRow> for &'_ str {
    fn index(&self, row: &YdbRow) -> Result<usize, Error> {
        row.columns
            .iter()
            .find(|c| &c.name() == self)
            .map(|c| c.ordinal())
            .ok_or(Error::ColumnNotFound((*self).to_string()))
    }
}
