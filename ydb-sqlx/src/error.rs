use sqlx_core::{error::DatabaseError, rt::TimeoutError};
use thiserror::Error;
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
        YdbOrCustomerError::Customer(e) => todo!(),
    }
}

pub(crate) fn err_ydb_to_sqlx(e: YdbError) -> sqlx_core::error::Error {
    match e {
        YdbError::Custom(_) => todo!(),
        YdbError::Convert(_) => todo!(),
        YdbError::NoRows => todo!(),
        YdbError::InternalError(_) => todo!(),
        YdbError::TransportDial(_) => todo!(),
        YdbError::Transport(_) => todo!(),
        YdbError::TransportGRPCStatus(_) => todo!(),
        YdbError::YdbStatusError(_) => todo!(),
        _ => todo!(),
    }
}
