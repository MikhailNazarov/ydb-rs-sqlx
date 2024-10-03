

use std::sync::Once;
use driver::install_drivers;
use sqlx_core::describe::Describe;
use futures::{future::BoxFuture, stream::BoxStream};
use itertools::Either;
use sqlx_core::{
    any::{self, *}, connection::{ConnectOptions, Connection}, database::Database, executor::Executor, ext::ustr::UStr, Error, Result
};

sqlx_core::declare_driver_with_optional_migrate!(DRIVER = Ydb);

use crate::{
    column::YdbColumn, connection::{YdbConnectOptions, YdbConnection}, database::Ydb, query::YdbQueryResult, row::YdbRow, transaction::YdbTransactionManager, typeinfo::{DataType, YdbTypeInfo}
};
use sqlx_core::transaction::TransactionManager;

impl AnyConnectionBackend for YdbConnection {
    fn name(&self) -> &str {
        Ydb::NAME
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, Result<()>> {
        Connection::close(*self)
    }

    fn close_hard(self: Box<Self>) -> BoxFuture<'static, Result<()>> {
        Connection::close_hard(*self)
    }

    fn ping(&mut self) -> BoxFuture<'_, Result<()>> {
        Connection::ping(self)
    }

    fn begin(&mut self) -> BoxFuture<'_, Result<()>> {
        YdbTransactionManager::begin(self)
    }

    fn commit(&mut self) -> BoxFuture<'_, Result<()>> {
        YdbTransactionManager::commit(self)
    }

    fn rollback(&mut self) -> BoxFuture<'_, Result<()>> {
        YdbTransactionManager::rollback(self)
    }

    fn start_rollback(&mut self) {
        YdbTransactionManager::start_rollback(self)
    }

    fn shrink_buffers(&mut self) {
        Connection::shrink_buffers(self);
    }

    fn flush(&mut self) -> BoxFuture<'_, Result<()>> {
        Connection::flush(self)
    }

    fn should_flush(&self) -> bool {
        Connection::should_flush(self)
    }

    fn as_migrate(
        &mut self,
    ) -> sqlx_core::Result<&mut (dyn sqlx_core::migrate::Migrate + Send + 'static)> {
        Ok(self)
    }

    

    

    fn fetch_many<'q>(
        &'q mut self,
        query: &'q str,
        persistent: bool,
        arguments: Option<AnyArguments<'q>>,
    ) -> BoxStream<'q, Result<Either<AnyQueryResult, AnyRow>,sqlx_core::Error>>
    {
        // let persistent = arguments.is_some();
        // let args = arguments.as_ref().map(AnyArguments::convert_to);
        todo!()
        //Box::pin(
            
            // self.run(query, args, 0, persistent, None)
            //     .try_flatten_stream()
            //     .map(
            //         move |res: sqlx_core::Result<Either<YdbQueryResult, YdbRow>>| match res? {
            //             Either::Left(result) => Ok(Either::Left(map_result(result))),
            //             Either::Right(row) => Ok(Either::Right(AnyRow::try_from(&row)?)),
            //         },
            //     ),
        //)
    }

    fn fetch_optional<'q>(
        &'q mut self,
        query: &'q str,
        persistent: bool,
        arguments: Option<AnyArguments<'q>>,
    ) -> BoxFuture<'q, Result<Option<AnyRow>, sqlx_core::Error>>{
        //let persistent = arguments.is_some();
        //let args = arguments.as_ref().map(AnyArguments::convert_to);

        Box::pin(async move {
            todo!();
           
            // let stream = self.run(query, args, 1, persistent, None).await?;
            // futures_util::pin_mut!(stream);

            // if let Some(Either::Right(row)) = stream.try_next().await? {
            //     return Ok(Some(AnyRow::try_from(&row)?));
            // }

            //Ok(None)
        })
    }

    fn prepare_with<'c, 'q: 'c>(
        &'c mut self,
        sql: &'q str,
        _parameters: &[any::AnyTypeInfo],
    ) -> BoxFuture<'c, Result<AnyStatement<'q>>> {
        Box::pin(async move {
            let statement = Executor::prepare_with(self, sql, &[]).await?;
            AnyStatement::try_from_statement(
                sql,
                &statement,
                statement.metadata.column_names.clone(),
            )
        })
    }

    fn describe<'q>(&'q mut self, sql: &'q str) -> BoxFuture<'q, Result<Describe<Any>>> {
        Box::pin(async move {
            let describe = Executor::describe(self, sql).await?;

            let columns = describe
                .columns
                .iter()
                .map(AnyColumn::try_from)
                .collect::<Result<Vec<_>, _>>()?;

            let parameters = match describe.parameters {
                Some(Either::Left(parameters)) => Some(Either::Left(
                    parameters
                        .iter()
                        .enumerate()
                        .map(|(i, type_info)| {
                            AnyTypeInfo::try_from(type_info).map_err(|_| {
                                sqlx_core::Error::AnyDriverError(
                                    format!(
                                        "Any driver does not support type {type_info} of parameter {i}"
                                    )
                                    .into(),
                                )
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                )),
                Some(Either::Right(count)) => Some(Either::Right(count)),
                None => None,
            };

            Ok(Describe {
                columns,
                parameters,
                nullable: describe.nullable,
            })
        })
    
    }
}

impl<'a> TryFrom<&'a YdbTypeInfo> for AnyTypeInfo {
    type Error = Error;

    fn try_from(ydb_type: &'a YdbTypeInfo) -> Result<Self, Self::Error> {
        Ok(AnyTypeInfo {
            kind: match &ydb_type.0 {
               
                DataType::Null | DataType::Void => AnyTypeInfoKind::Null,
                DataType::Bool => AnyTypeInfoKind::Bool,
                DataType::Int8 | DataType::Uint8 => AnyTypeInfoKind::SmallInt,
                DataType::Int16 | DataType::Uint16 => AnyTypeInfoKind::SmallInt,
                DataType::Int32 | DataType::Uint32 => AnyTypeInfoKind::Integer,
                DataType::Int64 | DataType::Uint64 => AnyTypeInfoKind::BigInt,
                DataType::Float => AnyTypeInfoKind::Real,
                DataType::Double => AnyTypeInfoKind::Double,
                DataType::String => AnyTypeInfoKind::Blob,      
                DataType::Text | DataType::Json => AnyTypeInfoKind::Text,
                _ => {
                    return Err(Error::AnyDriverError(
                        format!("Any driver does not support the Ydb type {ydb_type:?}").into(),
                    ))
                }
            },
        })
    }
}

impl<'a> TryFrom<&'a YdbColumn> for AnyColumn {
    type Error = Error;

    fn try_from(col: &'a YdbColumn) -> Result<Self, Self::Error> {
        let type_info = AnyTypeInfo::try_from(&col.type_info).map_err(|e| Error::ColumnDecode {
            index: col.name.to_string(),
            source: e.into(),
        })?;

        Ok(AnyColumn {
            ordinal: col.ordinal,
            name: UStr::new(col.name.as_ref()),
            type_info,
        })
    }
}

impl<'a> TryFrom<&'a YdbRow> for AnyRow {
    type Error = Error;

    fn try_from(_row: &'a YdbRow) -> Result<Self, Self::Error> {
        // let columns = Arc::new(row.columns().iter()
        // .map(|c| (UStr::new(&c.name), c.ordinal)).collect());
        // AnyRow::map_from(row, 
        //     columns
        // )
        todo!()
    }
}

impl<'a> TryFrom<&'a AnyConnectOptions> for YdbConnectOptions {
    type Error = Error;

    fn try_from(value: &'a AnyConnectOptions) -> Result<Self, Self::Error> {
        let opts = YdbConnectOptions::from_url(&value.database_url)?;
        //todo:
        //opts.log_settings = value.log_settings.clone();
        Ok(opts)
    }
}

fn map_result(res: YdbQueryResult) -> AnyQueryResult {
    AnyQueryResult {
        rows_affected: res.rows_affected(),
        last_insert_id: None,
    }
}

pub fn install_driver() {
    static ONCE: Once = Once::new();

    ONCE.call_once(|| {
        install_drivers(&[
            crate::any::DRIVER,
        ])
        .expect("drivers already installed")
    });
}