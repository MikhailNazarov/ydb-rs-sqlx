use std::sync::Arc;

use sqlx_core::rt::timeout;
use ydb::AnonymousCredentials;


use crate::error::WrappedError;

use super::{YdbConnectOptions, YdbConnection};

impl YdbConnection {
    pub(crate) async fn establish(options: &YdbConnectOptions) -> Result<Self, WrappedError> {
        let builder =
            ydb::ClientBuilder::new_from_connection_string(options.connection_string.clone())?
                .with_credentials(options.credentials.clone().unwrap_or(Arc::new(Box::new(AnonymousCredentials::new())))); 

        let client = builder.client()?;

        timeout(options.connection_timeout, client.wait()).await??;
       
       
        Ok(YdbConnection { client, transaction: None })
    }
}
