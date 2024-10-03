use std::{error::Error, fmt::Display};

use sqlx_core::{
    error::{DatabaseError, ErrorKind},
    rt::TimeoutError,
};
use thiserror::Error;
use tracing::error;
use ydb::{YdbError, YdbOrCustomerError};

#[derive(Error, Debug)]
pub enum WrappedError {
    #[error(transparent)]
    YdbError(#[from] YdbError),
    #[error(transparent)]
    YdbOrCustomerError(#[from] YdbOrCustomerError),
    #[error(transparent)]
    TimeoutError(#[from] TimeoutError),

    #[error(transparent)]
    Sqlx(#[from] sqlx_core::error::Error),
}

impl From<WrappedError> for sqlx_core::error::Error {
    fn from(value: WrappedError) -> Self {
        match value {
            WrappedError::YdbError(e) => err_ydb_to_sqlx(e),
            WrappedError::YdbOrCustomerError(e) => err_ydb_or_customer_to_sqlx(e),
            WrappedError::TimeoutError(_) => sqlx_core::Error::PoolTimedOut,
            WrappedError::Sqlx(e) => e,
        }
    }
}

pub(crate) fn err_ydb_or_customer_to_sqlx(e: YdbOrCustomerError) -> sqlx_core::error::Error {
    match e {
        YdbOrCustomerError::YDB(e) => err_ydb_to_sqlx(e),
        YdbOrCustomerError::Customer(_e) => todo!(),
    }
}

pub(crate) fn err_ydb_to_sqlx(e: YdbError) -> sqlx_core::error::Error {
    sqlx_core::error::Error::AnyDriverError(Box::new(e))

    // sqlx_core::error::Error::frm(e)
    // match e {
    //     YdbError::Custom(_) => sqlx_core::error::DatabaseError(),
    //     YdbError::Convert(_) => todo!(),
    //     YdbError::NoRows => todo!(),
    //     YdbError::InternalError(_) => todo!(),
    //     YdbError::TransportDial(_) => todo!(),
    //     YdbError::Transport(_) => todo!(),
    //     YdbError::TransportGRPCStatus(_) => todo!(),
    //     YdbError::YdbStatusError(e) => {
    //         sqlx_core::Error::Database(Box::new(InternalError { message: e.message }))
    //     }
    //     _ => todo!(),
    // }
}

#[derive(Debug)]
pub struct InternalError {
    pub message: String,
}
impl Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.message))
    }
}
impl Error for InternalError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl DatabaseError for InternalError {
    fn message(&self) -> &str {
        &self.message
    }

    fn as_error(&self) -> &(dyn Error + Send + Sync + 'static) {
        self
    }

    fn as_error_mut(&mut self) -> &mut (dyn Error + Send + Sync + 'static) {
        self
    }

    fn into_error(self: Box<Self>) -> Box<dyn Error + Send + Sync + 'static> {
        self
    }

    fn kind(&self) -> sqlx_core::error::ErrorKind {
        ErrorKind::Other
    }
}
