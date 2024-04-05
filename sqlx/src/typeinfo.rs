use std::fmt::Display;

use sqlx_core::type_info::TypeInfo;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct YdbTypeInfo(pub(crate) DataType);

impl TypeInfo for YdbTypeInfo {
    fn is_null(&self) -> bool {
        matches!(self.0, DataType::Null)
    }

    fn name(&self) -> &str {
        match self.0 {
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
            DataType::Optional => "Optional",
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum DataType {
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

    Optional,
    List,
    Struct,
}
