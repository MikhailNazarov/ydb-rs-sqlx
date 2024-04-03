use sqlx_core::arguments::Arguments;

use super::database::Ydb;

#[derive(Default)]
pub struct YdbArguments {}

impl<'q> Arguments<'q> for YdbArguments {
    type Database = Ydb;

    fn reserve(&mut self, additional: usize, size: usize) {
        todo!()
    }

    fn add<T>(&mut self, value: T)
    where
        T: 'q
            + Send
            + sqlx_core::encode::Encode<'q, Self::Database>
            + sqlx_core::types::Type<Self::Database>,
    {
        todo!()
    }
}

pub struct YdbArgumentBuffer {}
