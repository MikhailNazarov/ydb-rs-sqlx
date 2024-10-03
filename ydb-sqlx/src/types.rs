use std::{
    ops::Deref,
    time::{Duration, SystemTime},
};

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
            fn encode_by_ref(&self, buf: &mut YdbArgumentBuffer) -> Result<IsNull, BoxDynError> {
                let value = ydb::Value::from(self.clone());
                let is_null = match &value {
                    ydb::Value::Null => IsNull::Yes,
                    _ => IsNull::No,
                };
                buf.push(value, YdbTypeInfo($ydb_type_first));
                Ok(is_null)
            }
        }

        #[allow(unused)]
        impl Decode<'_, Ydb> for $native_type {
            fn decode(value: YdbValueRef<'_>) -> Result<Self, BoxDynError> {
                let value: ydb::Value = value.deref().clone();
                if let ydb::Value::Optional(x) = &value {}
                value.try_into().map_err(|e: YdbError| e.into())
            }
        }
    };
}

macro_rules! ydb_type_with_optional {
    ($native_type:ty, $ydb_type_first:path $(,$ydb_type:path)* $(,)?) => {
        #[allow(unused)]
        impl Type<Ydb> for $native_type {
            fn type_info() -> YdbTypeInfo {
                YdbTypeInfo($ydb_type_first)
            }
        }

        #[allow(unused)]
        impl Encode<'_, Ydb> for $native_type {
            fn encode_by_ref(&self, buf: &mut YdbArgumentBuffer) -> Result<IsNull, BoxDynError> {
                let value = ydb::Value::from(self.clone());
                let is_null = match &value {
                    ydb::Value::Null => IsNull::Yes,
                    _ => IsNull::No,
                };
                buf.push(value, YdbTypeInfo($ydb_type_first));
                Ok(is_null)
            }
        }

        #[allow(unused)]
        impl Decode<'_, Ydb> for $native_type {
            fn decode(value: YdbValueRef<'_>) -> Result<Self, BoxDynError> {
                let value: ydb::Value = value.deref().clone();
                match value {
                    ydb::Value::Optional(v) => {
                        if v.is_none() {
                            Err(Box::new(ydb::YdbError::Custom("decode error".to_string())))
                        } else {
                            Option::<$native_type>::try_from(ydb::Value::Optional(v))
                                .map_err(|e: YdbError| e.into())
                                .map(|x| x.unwrap())
                        }
                    }
                    value => value.try_into().map_err(|e: YdbError| e.into()),
                }
            }
        }
    };
}

ydb_type_with_optional!(bool, DataType::Bool);
ydb_type_with_optional!(i8, DataType::Int8);
ydb_type_with_optional!(u8, DataType::Uint8);
ydb_type_with_optional!(u16, DataType::Uint16, DataType::Uint8);
ydb_type_with_optional!(i16, DataType::Int16, DataType::Int8, DataType::Uint8);
ydb_type_with_optional!(
    i32,
    DataType::Int32,
    DataType::Int16,
    DataType::Uint16,
    DataType::Int8,
    DataType::Uint8
);
ydb_type_with_optional!(u32, DataType::Uint32, DataType::Uint16, DataType::Uint8);
ydb_type_with_optional!(
    i64,
    DataType::Int64,
    DataType::Int32,
    DataType::Uint32,
    DataType::Int16,
    DataType::Uint16,
    DataType::Int8,
    DataType::Uint8
);

ydb_type_with_optional!(
    u64,
    DataType::Uint64,
    DataType::Uint32,
    DataType::Uint16,
    DataType::Uint8
);

ydb_type!(Interval, DataType::Interval);

ydb_type_with_optional!(f32, DataType::Float);
ydb_type_with_optional!(f64, DataType::Double, DataType::Float);
ydb_type_with_optional!(std::time::SystemTime, DataType::Timestamp);

ydb_type_with_optional!(
    chrono::DateTime<chrono::Utc>,
    DataType::Date,
    DataType::DateTime
);

ydb_type_with_optional!(
    String,
    DataType::Text,
    DataType::Json,
    DataType::JsonDocument
);

ydb_type!(JsonDocument, DataType::JsonDocument);

ydb_type_with_optional!(
    Bytes,
    DataType::Bytes,
    DataType::Yson,
    DataType::Text,
    DataType::Json,
    DataType::JsonDocument
);

impl<'q, T> Encode<'q, Ydb> for NamedArgument<T>
where
    T: Type<Ydb> + Clone + Into<ydb::Value>,
{
    fn encode_by_ref(&self, buf: &mut YdbArgumentBuffer) -> Result<IsNull, BoxDynError> {
        let value = self.value().clone().into();
        let is_null = match &value {
            ydb::Value::Null => IsNull::Yes,
            _ => IsNull::No,
        };
        buf.push_named(self.name().to_owned(), value, T::type_info());
        Ok(is_null)
    }
}

impl<'q> Encode<'q, Ydb> for &'q str {
    fn encode_by_ref(
        &self,
        buf: &mut <Ydb as sqlx_core::database::Database>::ArgumentBuffer<'q>,
    ) -> Result<IsNull, BoxDynError> {
        buf.push(ydb::Value::from(*self), YdbTypeInfo(DataType::Text));
        Ok(IsNull::No)
    }
}

impl<'q, T> Encode<'q, Ydb> for (&'q str, T)
where
    T: Encode<'q, Ydb> + Type<Ydb> + Clone,
    ydb::Value: From<T>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <Ydb as sqlx_core::database::Database>::ArgumentBuffer<'q>,
    ) -> Result<IsNull, BoxDynError> {
        buf.push_named(
            self.0.to_owned(),
            ydb::Value::from(self.1.clone()),
            T::type_info(),
        );
        Ok(IsNull::No)
    }
}

impl Type<Ydb> for &str {
    fn type_info() -> YdbTypeInfo {
        YdbTypeInfo(DataType::Text)
    }
}

impl<T: Type<Ydb>> Type<Ydb> for (&str, T) {
    fn type_info() -> <Ydb as sqlx_core::database::Database>::TypeInfo {
        <T as Type<Ydb>>::type_info()
    }
}

impl Encode<'_, Ydb> for std::time::Instant {
    fn encode_by_ref(&self, buf: &mut YdbArgumentBuffer) -> Result<IsNull, BoxDynError> {
        let system_time = SystemTime::UNIX_EPOCH + Duration::from_secs(self.elapsed().as_secs());

        buf.push(
            ydb::Value::from(system_time),
            YdbTypeInfo(DataType::Timestamp),
        );
        Ok(IsNull::No)
    }
}

impl Type<Ydb> for std::time::Instant {
    fn type_info() -> YdbTypeInfo {
        YdbTypeInfo(DataType::Timestamp)
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

pub struct JsonDocument(pub String);

impl From<&JsonDocument> for ydb::Value {
    fn from(value: &JsonDocument) -> Self {
        ydb::Value::JsonDocument(value.0.clone())
    }
}

impl TryFrom<ydb::Value> for JsonDocument {
    type Error = YdbError;

    fn try_from(value: ydb::Value) -> Result<Self, Self::Error> {
        match value {
            ydb::Value::JsonDocument(json) => Ok(JsonDocument(json)),
            _ => Err(ydb::YdbError::Custom(
                "Value is not a json document".to_owned(),
            )),
        }
    }
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
