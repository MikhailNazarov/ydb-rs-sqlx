use sqlx_core::{
    bytes::Bytes,
    database::HasValueRef,
    value::{Value, ValueRef},
};

use crate::typeinfo::YdbTypeInfo;

use super::database::Ydb;

impl<'r> HasValueRef<'r> for Ydb {
    type Database = Ydb;

    type ValueRef = YdbValueRef<'r>;
}

pub struct YdbValueRef<'r> {
    pub(crate) value: Option<&'r [u8]>,
    pub(crate) row: Option<&'r Bytes>,
    pub(crate) type_info: YdbTypeInfo,
    pub(crate) format: YdbValueFormat,
}

impl<'r> ValueRef<'r> for YdbValueRef<'r> {
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum YdbValueFormat {
    Text = 0,
    Binary = 1,
}

pub struct YdbValue {
    pub(crate) value: ydb::Value,
    pub(crate) type_info: YdbTypeInfo,
    pub(crate) format: YdbValueFormat,
}

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
        self.value == ydb::Value::Null
    }
}
