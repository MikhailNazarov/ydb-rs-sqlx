use std::ops::Deref;

use sqlx_core::{
    decode::Decode,
    encode::{Encode, IsNull},
    error::BoxDynError,
    types::Type,
};
use ydb::{Value, YdbError};

use crate::{
    arguments::YdbArgumentBuffer,
    database::Ydb,
    typeinfo::YdbTypeInfo,
    value::{YdbValue, YdbValueRef},
};

impl Type<Ydb> for i16 {
    fn type_info() -> YdbTypeInfo {
        YdbTypeInfo(crate::typeinfo::DataType::Int16)
    }
}

impl Type<Ydb> for i32 {
    fn type_info() -> YdbTypeInfo {
        YdbTypeInfo(crate::typeinfo::DataType::Int32)
    }
}

impl Encode<'_, Ydb> for i32 {
    fn encode_by_ref(&self, buf: &mut YdbArgumentBuffer) -> IsNull {
        buf.extend(&self.to_be_bytes());

        IsNull::No
    }
}
impl Decode<'_, Ydb> for i32 {
    fn decode(value: YdbValueRef<'_>) -> Result<Self, BoxDynError> {
        let value: ydb::Value = value.deref().clone();
        value.try_into().map_err(|e: YdbError| e.into())
    }
}

impl Type<Ydb> for i64 {
    fn type_info() -> YdbTypeInfo {
        YdbTypeInfo(crate::typeinfo::DataType::Int64)
    }
}

impl Encode<'_, Ydb> for i64 {
    fn encode_by_ref(&self, buf: &mut YdbArgumentBuffer) -> IsNull {
        buf.extend(&self.to_be_bytes());

        IsNull::No
    }
}

impl Decode<'_, Ydb> for i64 {
    fn decode(value: YdbValueRef<'_>) -> Result<Self, BoxDynError> {
        todo!()
    }
}
