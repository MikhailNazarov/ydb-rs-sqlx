use std::{collections::HashMap, default};

use rustring_builder::StringBuilder;
use sqlx_core::{
    arguments::Arguments,
    encode::{Encode, IsNull},
    type_info::TypeInfo,
    types::Type,
};
use tracing::{debug, info};
use ydb::ValueOptional;

use crate::typeinfo::{DataType, YdbTypeInfo};

use super::database::Ydb;

#[derive(Default, Clone, Debug)]
#[allow(unused)]
pub struct YdbArguments {
    // Types of each bind parameter
    types: Vec<YdbTypeInfo>,

    // Buffer of encoded bind parameters
    buffer: YdbArgumentBuffer,
}
trait EncodeEx<'q>
where
    Self: Encode<'q, Ydb> + Sized,
{
    fn encode_ex(&self, buffer: &mut YdbArgumentBuffer) -> IsNull {
        Encode::encode(self, buffer)
    }
}

impl<'q, T> EncodeEx<'q> for Option<T>
where
    T: 'q + Send + sqlx_core::encode::Encode<'q, Ydb> + sqlx_core::types::Type<Ydb>,
    ydb::Value: From<Option<T>>,
{
    fn encode_ex(&self, buffer: &mut YdbArgumentBuffer) -> IsNull {
        match self {
            Some(v) => Encode::encode(v, buffer),
            None => {
                let val: Option<T> = None;
                buffer.push(ydb::Value::from(val), T::type_info());
                IsNull::Yes
            }
        }
    }
}

impl<'q> Arguments<'q> for YdbArguments {
    type Database = Ydb;

    fn reserve(&mut self, _additional: usize, _size: usize) {
        //
    }

    fn add<T>(&mut self, value: T)
    where
        T: 'q + Send + sqlx_core::encode::Encode<'q, Ydb> + sqlx_core::types::Type<Ydb>,
    {
        //todo: NULL не добавляется в буфер
        //_ = value.encode(&mut self.buffer);
        let is_null = value.encode(&mut self.buffer);
        if let IsNull::Yes = is_null {
            self.buffer.push(ydb::Value::Null, T::type_info());
        }
        self.types.push(T::type_info());
        //debug!("Types: {:?}", self.types);
    }
}

impl YdbArguments {
    pub(crate) fn into_iter(self) -> impl Iterator<Item = Argument> {
        self.buffer.arguments.into_iter()
    }
}

#[allow(unused)]
#[derive(Default, Clone, Debug)]
pub struct YdbArgumentBuffer {
    arguments: Vec<Argument>,
    index: i32,
}

#[derive(Clone, Debug)]
pub(crate) struct Argument {
    name: String,
    value: ydb::Value,
    type_info: YdbTypeInfo,
}

impl Argument {
    pub(crate) fn new(name: String, value: ydb::Value, type_info: YdbTypeInfo) -> Self {
        Self {
            name,
            value,
            type_info,
        }
    }

    pub(crate) fn declare(&self, sb: &mut StringBuilder) {
        sb.append(format!(
            "DECLARE {} as {}{};\n",
            self.name,
            self.type_info.name(),
            if let ydb::Value::Null = self.value {
                "?"
            } else {
                ""
            }
        ));
    }

    pub(crate) fn add_to_params(self, params: &mut HashMap<String, ydb::Value>) {
        if let ydb::Value::Null = self.value {
            params.insert(
                self.name(),
                ydb::Value::optional_from(ydb::Value::from(&self.type_info), None).unwrap(),
            );
        } else {
            params.insert(self.name(), self.value);
        }
    }

    pub(crate) fn name(&self) -> String {
        self.name.clone()
    }
}

impl YdbArgumentBuffer {
    pub(crate) fn push(&mut self, value: ydb::Value, type_info: YdbTypeInfo) {
        self.index = self.index + 1;
        self.arguments.push(Argument::new(
            format!("$arg_{}", self.index),
            value,
            type_info,
        ));
        //debug!("Arguments: {:?}", self.arguments);
    }

    pub(crate) fn push_named(&mut self, name: String, value: ydb::Value, type_info: YdbTypeInfo) {
        self.arguments.push(Argument::new(name, value, type_info));
        //debug!("Arguments: {:?}", self.arguments);
    }
}

pub struct NamedArgument<T>
where
    T: Type<Ydb>,
{
    name: String,
    value: T,
}

impl<T> Type<Ydb> for NamedArgument<T>
where
    T: Type<Ydb>,
{
    fn type_info() -> YdbTypeInfo {
        T::type_info()
    }
}

impl<T: Type<Ydb>> NamedArgument<T> {
    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub fn with_name<T: Type<Ydb>>(name: &str, value: T) -> NamedArgument<T> {
    let name = if name.starts_with('$') {
        name.into()
    } else {
        format!("${}", name)
    };
    NamedArgument::<T> { name, value }
}

/*

impl Deref for YdbArgumentBuffer {
    type Target = Vec<u8>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl DerefMut for YdbArgumentBuffer {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
} */
