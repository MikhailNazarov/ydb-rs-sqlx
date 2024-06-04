use std::{ops::Deref, time::Duration};

use sqlx_core::{
    decode::Decode,
    encode::{Encode, IsNull},
    error::BoxDynError,
    types::Type,
};
use ydb::{Bytes, YdbError};

use crate::{
    arguments::{NamedArgument, YdbArgumentBuffer},
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
                let value = ydb::Value::from(self.clone());
                let is_null = match &value {
                    ydb::Value::Null => IsNull::Yes,
                    _ => IsNull::No,
                };
                buf.push(value, YdbTypeInfo($ydb_type_first));
                is_null
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

ydb_type!(Interval, DataType::Interval);

ydb_type!(f32, DataType::Float);
ydb_type!(f64, DataType::Double, DataType::Float);
ydb_type!(
    std::time::SystemTime,
    DataType::Timestamp,
    DataType::Date,
    DataType::DateTime
);

ydb_type!(
    chrono::DateTime<chrono::Utc>,
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

impl<'q, T> Encode<'q, Ydb> for NamedArgument<T>
where
    T: Type<Ydb> + Clone + Into<ydb::Value>,
{
    fn encode_by_ref(&self, buf: &mut YdbArgumentBuffer) -> sqlx_core::encode::IsNull {
        let value = self.value().clone().into();
        let is_null = match &value {
            ydb::Value::Null => IsNull::Yes,
            _ => IsNull::No,
        };
        buf.push_named(self.name().to_owned(), value, T::type_info());
        is_null
    }
}

impl<'q> Encode<'q, Ydb> for &'q str {
    fn encode_by_ref(
        &self,
        buf: &mut <Ydb as sqlx_core::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> IsNull {
        buf.push(ydb::Value::from(*self), YdbTypeInfo(DataType::Text));
        IsNull::No
    }
}

pub enum Sign {
    Positive,
    Negative,
}
pub struct Interval {
    duration: Duration,
    sign: Sign,
}

impl From<&Interval> for ydb::Value {
    fn from(value: &Interval) -> Self {
        ydb::Value::Interval(ydb::SignedInterval {
            duration: value.duration,
            sign: match value.sign {
                Sign::Positive => ydb::Sign::Plus,
                Sign::Negative => ydb::Sign::Minus,
            },
        })
    }
}
impl TryFrom<ydb::Value> for Interval {
    type Error = YdbError;

    fn try_from(value: ydb::Value) -> Result<Self, Self::Error> {
        match value {
            ydb::Value::Interval(interval) => Ok(Interval {
                duration: interval.duration,
                sign: match interval.sign {
                    ydb::Sign::Plus => Sign::Positive,
                    ydb::Sign::Minus => Sign::Negative,
                },
            }),

            _ => Err(ydb::YdbError::Custom("Value is not an interval".to_owned())),
        }
    }
}
