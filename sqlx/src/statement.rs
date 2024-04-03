use sqlx_core::statement::Statement;

use super::database::Ydb;

pub struct YdbStatement {}

impl<'q> Statement<'q> for YdbStatement {
    type Database = Ydb;

    fn to_owned(
        &self,
    ) -> <Self::Database as sqlx_core::database::HasStatement<'static>>::Statement {
        todo!()
    }

    fn sql(&self) -> &str {
        todo!()
    }

    fn parameters(
        &self,
    ) -> Option<
        itertools::Either<&[<Self::Database as sqlx_core::database::Database>::TypeInfo], usize>,
    > {
        todo!()
    }

    fn columns(&self) -> &[<Self::Database as sqlx_core::database::Database>::Column] {
        todo!()
    }

    fn query(
        &self,
    ) -> sqlx_core::query::Query<
        '_,
        Self::Database,
        <Self::Database as sqlx_core::database::HasArguments<'_>>::Arguments,
    > {
        todo!()
    }

    fn query_with<'s, A>(&'s self, arguments: A) -> sqlx_core::query::Query<'s, Self::Database, A>
    where
        A: sqlx_core::arguments::IntoArguments<'s, Self::Database>,
    {
        todo!()
    }

    fn query_as<O>(
        &self,
    ) -> sqlx_core::query_as::QueryAs<
        '_,
        Self::Database,
        O,
        <Self::Database as sqlx_core::database::HasArguments<'_>>::Arguments,
    >
    where
        O: for<'r> sqlx_core::from_row::FromRow<
            'r,
            <Self::Database as sqlx_core::database::Database>::Row,
        >,
    {
        todo!()
    }

    fn query_as_with<'s, O, A>(
        &'s self,
        arguments: A,
    ) -> sqlx_core::query_as::QueryAs<'s, Self::Database, O, A>
    where
        O: for<'r> sqlx_core::from_row::FromRow<
            'r,
            <Self::Database as sqlx_core::database::Database>::Row,
        >,
        A: sqlx_core::arguments::IntoArguments<'s, Self::Database>,
    {
        todo!()
    }

    fn query_scalar<O>(
        &self,
    ) -> sqlx_core::query_scalar::QueryScalar<
        '_,
        Self::Database,
        O,
        <Self::Database as sqlx_core::database::HasArguments<'_>>::Arguments,
    >
    where
        (O,): for<'r> sqlx_core::from_row::FromRow<
            'r,
            <Self::Database as sqlx_core::database::Database>::Row,
        >,
    {
        todo!()
    }

    fn query_scalar_with<'s, O, A>(
        &'s self,
        arguments: A,
    ) -> sqlx_core::query_scalar::QueryScalar<'s, Self::Database, O, A>
    where
        (O,): for<'r> sqlx_core::from_row::FromRow<
            'r,
            <Self::Database as sqlx_core::database::Database>::Row,
        >,
        A: sqlx_core::arguments::IntoArguments<'s, Self::Database>,
    {
        todo!()
    }
}
