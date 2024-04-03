use super::database::Ydb;
use sqlx_core::describe::Describe;
use sqlx_core::executor::Executor;

#[derive(Debug)]
pub struct YdbExecutor {}

impl<'c> Executor<'c> for YdbExecutor {
    type Database = Ydb;

    fn fetch_many<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> futures_util::stream::BoxStream<
        'e,
        Result<
            itertools::Either<
                <Self::Database as sqlx_core::database::Database>::QueryResult,
                <Self::Database as sqlx_core::database::Database>::Row,
            >,
            sqlx_core::Error,
        >,
    >
    where
        'c: 'e,
        E: sqlx_core::executor::Execute<'q, Self::Database>,
    {
        todo!()
    }

    fn fetch_optional<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> futures_util::future::BoxFuture<
        'e,
        Result<Option<<Self::Database as sqlx_core::database::Database>::Row>, sqlx_core::Error>,
    >
    where
        'c: 'e,
        E: sqlx_core::executor::Execute<'q, Self::Database>,
    {
        todo!()
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        parameters: &'e [<Self::Database as sqlx_core::database::Database>::TypeInfo],
    ) -> futures_util::future::BoxFuture<
        'e,
        Result<
            <Self::Database as sqlx_core::database::HasStatement<'q>>::Statement,
            sqlx_core::Error,
        >,
    >
    where
        'c: 'e,
    {
        todo!()
    }

    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> futures_util::future::BoxFuture<'e, Result<Describe<Self::Database>, sqlx_core::Error>>
    where
        'c: 'e,
    {
        todo!()
    }
}
