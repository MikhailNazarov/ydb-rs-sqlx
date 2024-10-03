use sqlx_core::{
    database::{Database, HasStatementCache},
    migrate::MigrateDatabase,
};

use crate::value::YdbValueRef;

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

    type ValueRef<'r> = YdbValueRef<'r>;

    type Arguments<'q> = YdbArguments;

    type ArgumentBuffer<'q> = YdbArgumentBuffer;

    type Statement<'q> = YdbStatement<'q>;
}

impl HasStatementCache for Ydb {}
