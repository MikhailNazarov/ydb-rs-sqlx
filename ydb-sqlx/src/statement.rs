use std::{borrow::Cow, sync::Arc};

use itertools::Either;
use sqlx_core::{column::ColumnIndex, impl_statement_query, statement::Statement, Error, HashMap};

use crate::{arguments::YdbArguments, column::YdbColumn, typeinfo::YdbTypeInfo};

use super::database::Ydb;

pub struct YdbStatement<'q> {
    pub(crate) sql: Cow<'q, str>,
    pub(crate) metadata: Arc<YdbStatementMetadata>,
}

#[derive(Debug, Default)]
pub(crate) struct YdbStatementMetadata {
    pub(crate) columns: Vec<YdbColumn>,
    // This `Arc` is not redundant; it's used to avoid deep-copying this map for the `Any` backend.
    // See `sqlx-postgres/src/any.rs`
    pub(crate) column_names: Arc<HashMap<String, usize>>,
    pub(crate) parameters: Vec<YdbTypeInfo>,
}

impl<'q> Statement<'q> for YdbStatement<'q> {
    type Database = Ydb;

    fn to_owned(&self) -> YdbStatement<'static> {
        YdbStatement::<'static> {
            sql: Cow::Owned(self.sql.clone().into_owned()),
            metadata: self.metadata.clone(),
        }
    }

    fn sql(&self) -> &str {
        &self.sql
    }

    fn parameters(&self) -> Option<itertools::Either<&[YdbTypeInfo], usize>> {
        Some(Either::Left(&self.metadata.parameters))
    }

    fn columns(&self) -> &[YdbColumn] {
        &self.metadata.columns
    }

    impl_statement_query!(YdbArguments);
}

impl ColumnIndex<YdbStatement<'_>> for &'_ str {
    fn index(&self, statement: &YdbStatement<'_>) -> Result<usize, Error> {
        statement
            .metadata
            .column_names
            .get(*self)
            .ok_or_else(|| Error::ColumnNotFound((*self).into()))
            .map(|v| *v)
    }
}
