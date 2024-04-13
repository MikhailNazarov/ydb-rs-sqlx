use std::ops::Deref;

use sqlx_core::{
    decode::Decode,
    encode::{Encode, IsNull},
    error::BoxDynError,
    types::Type,
};
use ydb::{Bytes, YdbError};

use crate::{
    arguments::YdbArgumentBuffer,
    database::Ydb,
    typeinfo::{DataType, YdbTypeInfo},
    value::YdbValueRef,
};

macro_rules! ydb_type {
    ($native_type:ty, $ydb_type_first:path $(,$ydb_type:path)* $(,)?) => {
        #[allow(unused)]
        impl Type<Ydb> for $native_type {
            fn type_info() -> YdbTypeInfo {
                YdbTypeInfo($ydb_type_first)
            }
        }

        #[allow(unused)]
        impl Encode<'_, Ydb> for $native_type {
            fn encode_by_ref(&self, buf: &mut YdbArgumentBuffer) -> IsNull {
                todo!()
            }
        }

        #[allow(unused)]
        impl Decode<'_, Ydb> for $native_type {
            fn decode(value: YdbValueRef<'_>) -> Result<Self, BoxDynError> {
                let value: ydb::Value = value.deref().clone();
                value.try_into().map_err(|e: YdbError| e.into())
            }
        }
    };
}

ydb_type!(bool, DataType::Bool);
ydb_type!(i8, DataType::Int8);
ydb_type!(u8, DataType::Uint8);
ydb_type!(u16, DataType::Uint16, DataType::Uint8);
ydb_type!(i16, DataType::Int16, DataType::Int8, DataType::Uint8);
ydb_type!(
    i32,
    DataType::Int32,
    DataType::Int16,
    DataType::Uint16,
    DataType::Int8,
    DataType::Uint8
);
ydb_type!(u32, DataType::Uint32, DataType::Uint16, DataType::Uint8);
ydb_type!(
    i64,
    DataType::Int64,
    DataType::Int32,
    DataType::Uint32,
    DataType::Int16,
    DataType::Uint16,
    DataType::Int8,
    DataType::Uint8
);

ydb_type!(
    u64,
    DataType::Uint64,
    DataType::Uint32,
    DataType::Uint16,
    DataType::Uint8
);

ydb_type!(f32, DataType::Float);
ydb_type!(f64, DataType::Double, DataType::Float);
ydb_type!(
    std::time::SystemTime,
    DataType::Timestamp,
    DataType::Date,
    DataType::DateTime
);

ydb_type!(
    String,
    DataType::Text,
    DataType::Json,
    DataType::JsonDocument
);

ydb_type!(
    Bytes,
    DataType::String,
    DataType::Yson,
    DataType::Text,
    DataType::Json,
    DataType::JsonDocument
);
