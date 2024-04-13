mod connection_impl;
mod executor;

use std::fmt;
use std::{str::FromStr, sync::Arc, time::Duration};

use super::database::Ydb;
use futures_util::future;
use sqlx_core::connection::{ConnectOptions, Connection};
use ydb::{AnonymousCredentials, FromEnvCredentials};
use ydb::{Credentials, YdbError};

#[allow(unused)]
pub struct YdbConnection {
    client: ydb::Client,
}
impl fmt::Debug for YdbConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("YdbConnection")
    }
}

impl Connection for YdbConnection {
    type Database = Ydb;

    type Options = YdbConnectOptions;

    fn close(self) -> futures_util::future::BoxFuture<'static, Result<(), sqlx_core::Error>> {
        Box::pin(future::ready(Ok(())))
    }

    fn close_hard(self) -> futures_util::future::BoxFuture<'static, Result<(), sqlx_core::Error>> {
        todo!()
    }

    fn ping(&mut self) -> futures_util::future::BoxFuture<'_, Result<(), sqlx_core::Error>> {
        //todo: validate connection
        Box::pin(future::ready(Ok(())))
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

    fn shrink_buffers(&mut self) {}

    fn flush(&mut self) -> futures_util::future::BoxFuture<'_, Result<(), sqlx_core::Error>> {
        Box::pin(future::ready(Ok(())))
    }

    fn should_flush(&self) -> bool {
        false
    }
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct YdbConnectOptions {
    connection_string: String,
    connection_timeout: Duration,
    credentials: Arc<Box<dyn Credentials>>,
}

impl ConnectOptions for YdbConnectOptions {
    type Connection = YdbConnection;

    fn from_url(url: &url::Url) -> Result<Self, sqlx_core::Error> {
        Self::from_str(url.as_str())
    }

    fn connect(
        &self,
    ) -> futures_util::future::BoxFuture<'_, Result<Self::Connection, sqlx_core::Error>>
    where
        Self::Connection: Sized,
    {
        Box::pin(async move {
            let connection = YdbConnection::establish(self).await?;
            Ok(connection)
        })
    }

    fn log_statements(self, _level: tracing::log::LevelFilter) -> Self {
        todo!()
    }

    fn log_slow_statements(
        self,
        _level: tracing::log::LevelFilter,
        _duration: std::time::Duration,
    ) -> Self {
        todo!()
    }
}

impl YdbConnectOptions {
    #[allow(unused)]
    fn with_credentials_from_env(mut self) -> Result<Self, YdbError> {
        let cred = FromEnvCredentials::new()?;
        self.credentials = Arc::new(Box::new(cred));
        Ok(self)
    }
}

impl FromStr for YdbConnectOptions {
    type Err = sqlx_core::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(YdbConnectOptions {
            connection_string: s.to_owned(),
            connection_timeout: Duration::from_secs(10),
            credentials: Arc::new(Box::new(AnonymousCredentials::new())),
        })
    }
}

impl AsMut<YdbConnection> for YdbConnection {
    fn as_mut(&mut self) -> &mut YdbConnection {
        self
    }
}
