use sqlx_core::database::{Database, HasArguments, HasStatement};

use super::{
    arguments::{YdbArgumentBuffer, YdbArguments},
    column::YdbColumn,
    connection::YdbConnection,
    query::YdbQueryResult,
    row::YdbRow,
    statement::YdbStatement,
    transaction::YdbTransactionManager,
    typeinfo::YdbTypeInfo,
    value::YdbValue,
};

#[derive(Debug)]
pub struct Ydb {}

impl Database for Ydb {
    type Connection = YdbConnection;

    type TransactionManager = YdbTransactionManager;

    type Row = YdbRow;

    type QueryResult = YdbQueryResult;

    type Column = YdbColumn;

    type TypeInfo = YdbTypeInfo;

    type Value = YdbValue;

    const NAME: &'static str = "Ydb";

    const URL_SCHEMES: &'static [&'static str] = &["grpcs"];
}

impl<'q> HasStatement<'q> for Ydb {
    type Database = Ydb;

    type Statement = YdbStatement;
}

impl<'q> HasArguments<'q> for Ydb {
    type Database = Ydb;

    type Arguments = YdbArguments;

    type ArgumentBuffer = YdbArgumentBuffer;
}
