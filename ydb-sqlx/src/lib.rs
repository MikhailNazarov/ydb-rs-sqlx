use arguments::YdbArguments;
use connection::YdbConnection;
use database::Ydb;
use row::YdbRow;
use sqlx_core::executor::Executor;

use sqlx_core::{
    impl_acquire, impl_column_index_for_row, impl_column_index_for_statement,
    impl_encode_for_option, impl_into_arguments_for_arguments,
};
use statement::YdbStatement;

pub mod arguments;
pub mod column;
pub mod connection;
pub mod database;
pub mod error;
mod migration;
pub mod query;
pub mod row;
pub mod statement;
pub mod transaction;
pub mod typeinfo;
pub mod types;
pub mod value;
/// An alias for [`Pool`][crate::pool::Pool], specialized for Ydb.
pub type YdbPool = sqlx_core::pool::Pool<Ydb>;

/// An alias for [`PoolOptions`][crate::pool::PoolOptions], specialized for Ydb.
pub type YdbPoolOptions = sqlx_core::pool::PoolOptions<Ydb>;

/// An alias for [`Executor<'_, Database = Ydb>`][Executor].
pub trait YdbExecutor<'c>: Executor<'c, Database = Ydb> {}
impl<'c, T: Executor<'c, Database = Ydb>> YdbExecutor<'c> for T {}

impl_into_arguments_for_arguments!(YdbArguments);
impl_acquire!(Ydb, YdbConnection);
impl_column_index_for_row!(YdbRow);
impl_column_index_for_statement!(YdbStatement);
impl_encode_for_option!(Ydb);
pub use arguments::with_name;
