use std::str::FromStr;

use sqlx_core::connection::{ConnectOptions, Connection};

use super::database::Ydb;

pub struct YdbConnection {}

impl Connection for YdbConnection {
    type Database = Ydb;

    type Options = YdbConnectOptions;

    fn close(self) -> futures_util::future::BoxFuture<'static, Result<(), sqlx_core::Error>> {
        todo!()
    }

    fn close_hard(self) -> futures_util::future::BoxFuture<'static, Result<(), sqlx_core::Error>> {
        todo!()
    }

    fn ping(&mut self) -> futures_util::future::BoxFuture<'_, Result<(), sqlx_core::Error>> {
        todo!()
    }

    fn begin(
        &mut self,
    ) -> futures_util::future::BoxFuture<
        '_,
        Result<sqlx_core::transaction::Transaction<'_, Self::Database>, sqlx_core::Error>,
    >
    where
        Self: Sized,
    {
        todo!()
    }

    fn shrink_buffers(&mut self) {
        todo!()
    }

    fn flush(&mut self) -> futures_util::future::BoxFuture<'_, Result<(), sqlx_core::Error>> {
        todo!()
    }

    fn should_flush(&self) -> bool {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct YdbConnectOptions {}

impl ConnectOptions for YdbConnectOptions {
    type Connection = YdbConnection;

    fn from_url(url: &url::Url) -> Result<Self, sqlx_core::Error> {
        todo!()
    }

    fn connect(
        &self,
    ) -> futures_util::future::BoxFuture<'_, Result<Self::Connection, sqlx_core::Error>>
    where
        Self::Connection: Sized,
    {
        todo!()
    }

    fn log_statements(self, level: tracing::log::LevelFilter) -> Self {
        todo!()
    }

    fn log_slow_statements(
        self,
        level: tracing::log::LevelFilter,
        duration: std::time::Duration,
    ) -> Self {
        todo!()
    }
}

impl FromStr for YdbConnectOptions {
    type Err = sqlx_core::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
