use sqlx_core::{
    error::{self, DatabaseError},
    rt::TimeoutError,
};
use thiserror::Error;
use ydb::YdbError;

#[derive(Error, Debug)]
pub enum WrappedError {
    #[error(transparent)]
    YdbError(#[from] YdbError),
    #[error(transparent)]
    TimeoutError(#[from] TimeoutError),
}

impl DatabaseError for WrappedError {
    fn message(&self) -> &str {
        self.message()
    }

    fn as_error(&self) -> &(dyn std::error::Error + Send + Sync + 'static) {
        self
    }

    fn as_error_mut(&mut self) -> &mut (dyn std::error::Error + Send + Sync + 'static) {
        self
    }

    fn into_error(self: Box<Self>) -> Box<dyn std::error::Error + Send + Sync + 'static> {
        self
    }

    fn kind(&self) -> sqlx_core::error::ErrorKind {
        self.kind()
    }
}
