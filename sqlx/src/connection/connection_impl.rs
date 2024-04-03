use sqlx_core::rt::timeout;

use crate::error::WrappedError;

use super::{YdbConnectOptions, YdbConnection};

impl YdbConnection {
    pub(crate) async fn establish(options: &YdbConnectOptions) -> Result<Self, WrappedError> {
        let builder =
            ydb::ClientBuilder::new_from_connection_string(options.connection_string.clone())?;

        let client = builder.client()?;

        timeout(options.connection_timeout, client.wait()).await??;
        client.wait().await?;

        let _ = client.wait().await?;
        Ok(YdbConnection { client })
    }
}
