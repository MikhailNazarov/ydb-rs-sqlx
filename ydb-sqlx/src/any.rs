use sqlx_core::any::{
    AnyColumn, AnyConnectOptions, AnyConnectionBackend, AnyQueryResult, AnyRow, AnyTypeInfo,
};

use crate::{
    column::YdbColumn,
    connection::{YdbConnectOptions, YdbConnection},
    query::YdbQueryResult,
    row::YdbRow,
    typeinfo::YdbTypeInfo,
};

impl AnyConnectionBackend for YdbConnection {
    fn name(&self) -> &str {
        todo!()
    }

    fn close(
        self: Box<Self>,
    ) -> futures::prelude::future::BoxFuture<'static, sqlx_core::Result<()>> {
        todo!()
    }

    fn close_hard(
        self: Box<Self>,
    ) -> futures::prelude::future::BoxFuture<'static, sqlx_core::Result<()>> {
        todo!()
    }

    fn ping(&mut self) -> futures::prelude::future::BoxFuture<'_, sqlx_core::Result<()>> {
        todo!()
    }

    fn begin(&mut self) -> futures::prelude::future::BoxFuture<'_, sqlx_core::Result<()>> {
        todo!()
    }

    fn commit(&mut self) -> futures::prelude::future::BoxFuture<'_, sqlx_core::Result<()>> {
        todo!()
    }

    fn rollback(&mut self) -> futures::prelude::future::BoxFuture<'_, sqlx_core::Result<()>> {
        todo!()
    }

    fn start_rollback(&mut self) {
        todo!()
    }

    fn shrink_buffers(&mut self) {
        todo!()
    }

    fn flush(&mut self) -> futures::prelude::future::BoxFuture<'_, sqlx_core::Result<()>> {
        todo!()
    }

    fn should_flush(&self) -> bool {
        todo!()
    }

    fn fetch_many<'q>(
        &'q mut self,
        query: &'q str,
        arguments: Option<sqlx_core::any::AnyArguments<'q>>,
    ) -> futures::prelude::stream::BoxStream<
        'q,
        sqlx_core::Result<
            itertools::Either<sqlx_core::any::AnyQueryResult, sqlx_core::any::AnyRow>,
        >,
    > {
        todo!()
    }

    fn fetch_optional<'q>(
        &'q mut self,
        query: &'q str,
        arguments: Option<sqlx_core::any::AnyArguments<'q>>,
    ) -> futures::prelude::future::BoxFuture<'q, sqlx_core::Result<Option<sqlx_core::any::AnyRow>>>
    {
        todo!()
    }

    fn prepare_with<'c, 'q: 'c>(
        &'c mut self,
        sql: &'q str,
        parameters: &[sqlx_core::any::AnyTypeInfo],
    ) -> futures::prelude::future::BoxFuture<'c, sqlx_core::Result<sqlx_core::any::AnyStatement<'q>>>
    {
        todo!()
    }

    fn describe<'q>(
        &'q mut self,
        sql: &'q str,
    ) -> futures::prelude::future::BoxFuture<'q, sqlx_core::Result<Describe<sqlx_core::any::Any>>>
    {
        todo!()
    }
}

impl<'a> TryFrom<&'a YdbTypeInfo> for AnyTypeInfo {
    type Error = sqlx_core::Error;

    fn try_from(ydb_type: &'a YdbTypeInfo) -> Result<Self, Self::Error> {
        Ok(AnyTypeInfo {
            kind: match &ydb_type.0 {
                crate::typeinfo::DataType::Unknown => todo!(),
                crate::typeinfo::DataType::Void => todo!(),
                crate::typeinfo::DataType::Null => todo!(),
                crate::typeinfo::DataType::Bool => todo!(),
                crate::typeinfo::DataType::Int8 => todo!(),
                crate::typeinfo::DataType::Uint8 => todo!(),
                crate::typeinfo::DataType::Int16 => todo!(),
                crate::typeinfo::DataType::Uint16 => todo!(),
                crate::typeinfo::DataType::Int32 => todo!(),
                crate::typeinfo::DataType::Uint32 => todo!(),
                crate::typeinfo::DataType::Int64 => todo!(),
                crate::typeinfo::DataType::Uint64 => todo!(),
                crate::typeinfo::DataType::Float => todo!(),
                crate::typeinfo::DataType::Double => todo!(),
                crate::typeinfo::DataType::Date => todo!(),
                crate::typeinfo::DataType::DateTime => todo!(),
                crate::typeinfo::DataType::Timestamp => todo!(),
                crate::typeinfo::DataType::Interval => todo!(),
                crate::typeinfo::DataType::String => todo!(),
                crate::typeinfo::DataType::Text => todo!(),
                crate::typeinfo::DataType::Yson => todo!(),
                crate::typeinfo::DataType::Json => todo!(),
                crate::typeinfo::DataType::JsonDocument => todo!(),
                crate::typeinfo::DataType::List => todo!(),
                crate::typeinfo::DataType::Struct => todo!(),
                /*
                PgType::Void => AnyTypeInfoKind::Null,
                PgType::Int2 => AnyTypeInfoKind::SmallInt,
                PgType::Int4 => AnyTypeInfoKind::Integer,
                PgType::Int8 => AnyTypeInfoKind::BigInt,
                PgType::Float4 => AnyTypeInfoKind::Real,
                PgType::Float8 => AnyTypeInfoKind::Double,
                PgType::Bytea => AnyTypeInfoKind::Blob,
                PgType::Text | PgType::Varchar => AnyTypeInfoKind::Text,
                PgType::DeclareWithName(UStr::Static("citext")) => AnyTypeInfoKind::Text,
                _ => {
                    return Err(sqlx_core::Error::AnyDriverError(
                        format!("Any driver does not support the Postgres type {pg_type:?}").into(),
                    ))
                }
                */
            },
        })
    }
}

impl<'a> TryFrom<&'a YdbColumn> for AnyColumn {
    type Error = sqlx_core::Error;

    fn try_from(col: &'a YdbColumn) -> Result<Self, Self::Error> {
        let type_info =
            AnyTypeInfo::try_from(&col.type_info).map_err(|e| sqlx_core::Error::ColumnDecode {
                index: col.name.to_string(),
                source: e.into(),
            })?;

        Ok(AnyColumn {
            ordinal: col.ordinal,
            name: col.name.clone(),
            type_info,
        })
    }
}

impl<'a> TryFrom<&'a YdbRow> for AnyRow {
    type Error = sqlx_core::Error;

    fn try_from(row: &'a YdbRow) -> Result<Self, Self::Error> {
        AnyRow::map_from(row, row.metadata.column_names.clone())
    }
}

impl<'a> TryFrom<&'a YdbConnectOptions> for YdbConnectOptions {
    type Error = sqlx_core::Error;

    fn try_from(value: &'a AnyConnectOptions) -> Result<Self, Self::Error> {
        let mut opts = YdbConnectOptions::parse_from_url(&value.database_url)?;
        opts.log_settings = value.log_settings.clone();
        Ok(opts)
    }
}

fn map_result(res: YdbQueryResult) -> AnyQueryResult {
    AnyQueryResult {
        rows_affected: res.rows_affected(),
        last_insert_id: None,
    }
}
