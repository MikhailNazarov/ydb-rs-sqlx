mod connection_impl;
mod executor;
pub mod schema_executor;

use std::fmt;
use std::ops::Deref;
use std::{str::FromStr, sync::Arc, time::Duration};

use self::schema_executor::YdbSchemaExecutor;

use super::database::Ydb;
use futures_util::future;
use sqlx_core::connection::{ConnectOptions, Connection, LogSettings};


use sqlx_core::transaction::Transaction;
use ydb::{AccessTokenCredentials, AnonymousCredentials, MetadataUrlCredentials, ServiceAccountCredentials, StaticCredentials};
use ydb::Credentials;

/// A connection to the YDB database.
#[allow(unused)]
pub struct YdbConnection{
    client: ydb::Client,
    pub(crate) transaction: Option<Box<dyn ydb::Transaction>>,
    pub(crate) log_settings: LogSettings,
    pub(crate) stats_mode: StatsMode
}
impl fmt::Debug for YdbConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("YdbConnection")
    }
}

impl Deref for YdbConnection {
    type Target = ydb::Client;
    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl YdbConnection{
    /// Get a schema executor
    pub fn schema(&self)->YdbSchemaExecutor{
        YdbSchemaExecutor::new(self)
    }
}


impl Connection for YdbConnection {
    type Database = Ydb;

    type Options = YdbConnectOptions;

    fn close(self) -> futures_util::future::BoxFuture<'static, Result<(), sqlx_core::Error>> {
        Box::pin(future::ready(Ok(())))
    }

    fn close_hard(self) -> futures_util::future::BoxFuture<'static, Result<(), sqlx_core::Error>> {
        Box::pin(future::ready(Ok(())))
    }

    fn ping(&mut self) -> futures_util::future::BoxFuture<'_, Result<(), sqlx_core::Error>> {
        
        //todo: validate connection
        // Box::pin(async{
        //     self.client.table_client().keepalive().await
        //     .map_err(|_|sqlx_core::error::Error::PoolClosed)
            
        // })
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
        Transaction::begin(self)
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
#[derive(Clone, Debug, Default)]
pub struct YdbConnectOptions {
    connection_string: String,
    connection_timeout: Duration,
    credentials: Option<Arc<Box<dyn Credentials>>>,
    log_settings:  LogSettings,
    stats_mode: StatsMode
}

#[derive(Clone, Debug, Default)]
pub enum StatsMode{    
    None,
    #[default]
    Basic,
    Full,
    Profile,
}

impl From<&StatsMode> for ydb::QueryStatsMode {
    fn from(mode: &StatsMode) -> Self {
        match mode {
            StatsMode::None => ydb::QueryStatsMode::None,
            StatsMode::Basic => ydb::QueryStatsMode::Basic,
            StatsMode::Full => ydb::QueryStatsMode::Full,
            StatsMode::Profile => ydb::QueryStatsMode::Profile,
        }
    }
}

impl YdbConnectOptions{

    pub fn from_env() -> Result<Self, sqlx_core::Error> {

        let ydb_conn_str = std::env::var("YDB_CONNECTION_STRING").ok();

        if let Some(ydb_conn_str) = ydb_conn_str {
            return Self::from_url(&url::Url::parse(&ydb_conn_str)
            .map_err(|e| sqlx_core::Error::Configuration(e.into()))?);
        }

        let db_url = std::env::var("DATABASE_URL").map_err(|e| sqlx_core::Error::Configuration(e.into()))?;

        
        Self::from_url(&url::Url::parse(&db_url)
            .map_err(|e| sqlx_core::Error::Configuration(e.into()))?)
    }

    pub fn  log_statements(mut self, level: tracing::log::LevelFilter) -> Self {
        self.log_settings.log_statements(level);
        self
    }

    pub fn log_slow_statements(
        mut self,
        level: tracing::log::LevelFilter,
        duration: std::time::Duration,
    ) -> Self {
        self.log_settings.log_slow_statements(level, duration);
        self
    }

    pub fn with_stats(mut self, mode: StatsMode) -> Self {
        self.stats_mode = mode;
        self
    }
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
    
    fn  log_statements(self, level: tracing::log::LevelFilter) -> Self {
        self.log_statements(level)
    }

    fn log_slow_statements(
        self,
        level: tracing::log::LevelFilter,
        duration: std::time::Duration,
    ) -> Self {
        self.log_slow_statements(level, duration)
    }

}


impl FromStr for YdbConnectOptions {
    type Err = sqlx_core::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut options = YdbConnectOptions{
            connection_timeout : Duration::from_secs(2),
            ..Default::default()
        };
        
        let url = sqlx_core::url::Url::parse(s)
            .map_err(|e| sqlx_core::Error::Configuration(e.into()))?;
        let mut user = None;
        let mut password = None;
        let mut database = None; 

        for (k,v) in url.query_pairs(){
            match k.as_ref(){
               
                "connection_timeout" => {
                    let timeout = v.parse::<u64>().map_err(|e| sqlx_core::Error::Configuration(e.into()))?;
                    options.connection_timeout = Duration::from_secs(timeout);
                },
                "sa-key" => {
                    let sa = ServiceAccountCredentials::from_file(v.as_ref())
                        .map_err(|e| sqlx_core::Error::Configuration(e.into()))?;
                    options.credentials = Some(Arc::new(Box::new(sa)));
                },
                "anonymous" =>{
                    let cred = AnonymousCredentials::new();
                    options.credentials = Some(Arc::new(Box::new(cred)));
                },
                "metadata" =>{
                    let cred = MetadataUrlCredentials::new();
                    options.credentials = Some(Arc::new(Box::new(cred)));
                }
                "token" =>{
                    let cred = AccessTokenCredentials::from(v.as_ref());
                    options.credentials = Some(Arc::new(Box::new(cred)));
                },
                "database" =>{
                    database = Some(v.into_owned());
                },
                "user" =>{
                    user = Some(v.into_owned());
                },
                "password"=>{
                    password = Some(v.into_owned());
                },
                _ => continue
            }
        }
        let database = database.unwrap_or("/".into()).to_string();

        

        
        let endpoint = format!("{}://{}:{}?database={}",url.scheme(),url.host().unwrap(),url.port().unwrap(),database);
        if let (Some(user), Some(password)) = (user, password) {
            let password = password.to_string();
            let uri = http::Uri::from_str(&endpoint).unwrap();
            let user = user.to_string();
            let cred = StaticCredentials::new(user, password, uri, database);
            options.credentials = Some(Arc::new(Box::new(cred)));
        }
        if options.credentials.is_none() {
            options.credentials = Some(Arc::new(Box::new(AnonymousCredentials::new())));
        }
        options.connection_string = endpoint;
        Ok(options)
    }

}

impl AsMut<YdbConnection> for YdbConnection {
    fn as_mut(&mut self) -> &mut YdbConnection {
        self
    }
}
