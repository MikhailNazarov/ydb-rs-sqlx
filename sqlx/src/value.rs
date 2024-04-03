use sqlx_core::{
    database::HasValueRef,
    value::{Value, ValueRef},
};

use super::database::Ydb;

impl<'r> HasValueRef<'r> for Ydb {
    type Database = Ydb;

    type ValueRef = YdbValueRef;
}

pub struct YdbValueRef {}

impl<'r> ValueRef<'r> for YdbValueRef {
    type Database = Ydb;

    fn to_owned(&self) -> <Self::Database as sqlx_core::database::Database>::Value {
        todo!()
    }

    fn type_info(
        &self,
    ) -> std::borrow::Cow<'_, <Self::Database as sqlx_core::database::Database>::TypeInfo> {
        todo!()
    }

    fn is_null(&self) -> bool {
        todo!()
    }
}

pub struct YdbValue {}

impl Value for YdbValue {
    type Database = Ydb;

    fn as_ref(&self) -> <Self::Database as sqlx_core::database::HasValueRef<'_>>::ValueRef {
        todo!()
    }

    fn type_info(
        &self,
    ) -> std::borrow::Cow<'_, <Self::Database as sqlx_core::database::Database>::TypeInfo> {
        todo!()
    }

    fn is_null(&self) -> bool {
        todo!()
    }
}
