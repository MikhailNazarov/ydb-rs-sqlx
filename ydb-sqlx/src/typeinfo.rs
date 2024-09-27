use std::fmt::Display;

use sqlx_core::type_info::TypeInfo;
use ydb::Bytes;

use crate::types::Interval;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct YdbTypeInfo(pub(crate) DataType);

impl YdbTypeInfo {
    pub(crate) fn new(value: &ydb::Value) -> Self {
        let data_type = value.into();

        YdbTypeInfo(data_type)
    }
}

impl TypeInfo for YdbTypeInfo {
    fn is_null(&self) -> bool {
        let is_null = matches!(self.0, DataType::Null);
        is_null
    }

    fn name(&self) -> &str {
        match self.0 {
            DataType::Unknown => "???",
            DataType::Void => "Void",
            DataType::Null => "Null",
            DataType::Bool => "Bool",
            DataType::Int8 => "Int8",
            DataType::Uint8 => "Uint8",
            DataType::Int16 => "Int16",
            DataType::Uint16 => "Uint16",
            DataType::Int32 => "Int32",
            DataType::Uint32 => "Uint32",
            DataType::Int64 => "Int64",
            DataType::Uint64 => "Uint64",
            DataType::Float => "Float",
            DataType::Double => "Double",
            DataType::Date => "Date",
            DataType::DateTime => "DateTime",
            DataType::Timestamp => "Timestamp",
            DataType::Interval => "Interval",
            DataType::String => "String",
            DataType::Text => "Text",
            DataType::Yson => "Yson",
            DataType::Json => "Json",
            DataType::JsonDocument => "JsonDocument",
            // DataType::Optional(_) => "Optional",
            DataType::List => "List",
            DataType::Struct => "Struct",
        }
    }
}

impl Display for YdbTypeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(self.name())
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum DataType {
    Unknown,
    Void,
    Null,
    Bool,
    Int8,
    Uint8,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Int64,
    Uint64,
    Float,
    Double,
    Date,
    DateTime,
    Timestamp,
    Interval,

    /// Store native bytes array, similary to binary/blob in other databases. It named string by history reason only.
    /// Use Utf8 type for store text.
    String,

    /// Text data, encoded to valid utf8
    Text,
    Yson,
    Json,
    JsonDocument,

    //Optional(Box<DataType>),
    List,
    Struct,
}

impl From<&YdbTypeInfo> for ydb::Value {
    fn from(value: &YdbTypeInfo) -> Self {
        match value.0 {
            DataType::Void => ydb::Value::Void,
            DataType::Null => ydb::Value::Null,
            DataType::Bool => ydb::Value::Bool(false),
            DataType::Int8 => ydb::Value::Int8(0),
            DataType::Uint8 => ydb::Value::Uint8(0),
            DataType::Int16 => ydb::Value::Int16(0),
            DataType::Uint16 => ydb::Value::Uint16(0),
            DataType::Int32 => ydb::Value::Int32(0),
            DataType::Uint32 => ydb::Value::Uint32(0),
            DataType::Int64 => ydb::Value::Int64(0),
            DataType::Uint64 => ydb::Value::Uint64(0),
            DataType::Float => ydb::Value::Float(0.0),
            DataType::Double => ydb::Value::Double(0.0),
            DataType::Date => todo!(),
            DataType::DateTime => todo!(),
            DataType::Timestamp => todo!(),
            DataType::Interval => todo!(),
            DataType::String => ydb::Value::Text(String::new()),
            DataType::Text => ydb::Value::Text(String::new()),
            DataType::Yson => ydb::Value::Yson(Bytes::default()),
            DataType::Json => ydb::Value::Json(String::new()),
            DataType::JsonDocument => ydb::Value::JsonDocument(String::new()),
            DataType::List => ydb::Value::Void,
            DataType::Struct => ydb::Value::Void,
            DataType::Unknown => ydb::Value::Void,
        }
    }
}

impl From<&ydb::Value> for DataType {
    fn from(value: &ydb::Value) -> Self {
        match value {
            ydb::Value::Void => DataType::Void,
            ydb::Value::Null => DataType::Null,
            ydb::Value::Bool(_) => DataType::Bool,
            ydb::Value::Int8(_) => DataType::Int8,
            ydb::Value::Uint8(_) => DataType::Uint8,
            ydb::Value::Int16(_) => DataType::Int16,
            ydb::Value::Uint16(_) => DataType::Uint16,
            ydb::Value::Int32(_) => DataType::Int32,
            ydb::Value::Uint32(_) => DataType::Uint32,
            ydb::Value::Int64(_) => DataType::Int64,
            ydb::Value::Uint64(_) => DataType::Uint64,
            ydb::Value::Float(_) => DataType::Float,
            ydb::Value::Double(_) => DataType::Double,
            ydb::Value::Date(_) => DataType::Date,
            ydb::Value::DateTime(_) => DataType::DateTime,
            ydb::Value::Timestamp(_) => DataType::Timestamp,
            ydb::Value::Interval(_) => DataType::Interval,
            ydb::Value::Bytes(_) => DataType::String,
            ydb::Value::Text(_) => DataType::Text,
            ydb::Value::Yson(_) => DataType::Yson,
            ydb::Value::Json(_) => DataType::Json,
            ydb::Value::JsonDocument(_) => DataType::JsonDocument,
            ydb::Value::Optional(t) => DataType::from(t.get_inner_type()),
            ydb::Value::List(_) => DataType::List,
            ydb::Value::Struct(_) => DataType::Struct,
            _ => DataType::Unknown,
        }
    }
}
#[cfg(test)]
mod test {
    use sqlx_core::decode::Decode;

    use crate::{
        database::Ydb,
        value::{YdbValue, YdbValueRef},
    };

    use super::{DataType, YdbTypeInfo};

    #[test]
    pub fn decode_null() {
        let value = YdbValue::new(
            ydb::Value::Optional(Default::default()),
            YdbTypeInfo(DataType::String),
        );
        let r = YdbValueRef::new(&value);
        let res = <Option<String> as Decode<Ydb>>::decode(r);

        assert!(!res.is_err());
    }
}
