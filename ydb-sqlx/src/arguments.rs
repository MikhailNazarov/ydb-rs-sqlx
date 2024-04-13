use std::collections::HashMap;

use rustring_builder::StringBuilder;
use sqlx_core::{
    arguments::Arguments,
    encode::{Encode, IsNull},
    type_info::TypeInfo,
};

use crate::typeinfo::YdbTypeInfo;

use super::database::Ydb;

#[derive(Default, Clone, Debug)]
#[allow(unused)]
pub struct YdbArguments {
    // Types of each bind parameter
    types: Vec<YdbTypeInfo>,

    // Buffer of encoded bind parameters
    buffer: YdbArgumentBuffer,
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
        _ = value.encode(&mut self.buffer);
        self.types.push(T::type_info());
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

impl YdbArgumentBuffer {
    pub(crate) fn new() -> Self {
        Self::default()
    }
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
            "DECLARE {} as {};\n",
            self.name,
            self.type_info.name()
        ));
    }

    pub(crate) fn add_to_params(&self, params: &mut HashMap<String, ydb::Value>) {
        params.insert(self.name(), self.value.clone());
    }

    pub(crate) fn name(&self) -> String {
        self.name.clone()
    }
}

impl YdbArgumentBuffer {
    pub(crate) fn push(&mut self, value: ydb::Value, type_info: YdbTypeInfo) {
        self.index = self.index + 1;
        self.arguments
            .push(Argument::new(self.index, value, type_info));
    }

    pub(crate) fn push_named(&mut self, name: String, value: ydb::Value, type_info: YdbTypeInfo) {
        self.index = self.index + 1;
        self.arguments
            .push(Argument::new(self.index, value, type_info));
    }
}

pub struct NamedArgument<T> {
    name: String,
    value: T,
}

pub fn with_name<T>(name: impl Into<String>, value: T) -> NamedArgument<T> {
    NamedArgument {
        name: name.into(),
        value,
    }
}

impl<'q, T> Encode<'q, Ydb> for NamedArgument<T>
where
    T: Clone,
    ydb::Value: From<T>,
{
    fn encode_by_ref(&self, buf: &mut YdbArgumentBuffer) -> sqlx_core::encode::IsNull {
        let value = ydb::Value::from(self.value.clone());
        let is_null = match &value {
            ydb::Value::Null => IsNull::Yes,
            _ => IsNull::No,
        };
        buf.push_named(self.name.clone(), value, YdbTypeInfo(self.value));
        is_null
    }
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
