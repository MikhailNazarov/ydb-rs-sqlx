use std::ops::Deref;

use sqlx_core::value::{Value, ValueRef};

use crate::typeinfo::YdbTypeInfo;

use super::database::Ydb;

#[allow(unused)]
pub struct YdbValueRef<'r> {
    value_ref: &'r YdbValue,
}
impl<'r> Deref for YdbValueRef<'r> {
    type Target = ydb::Value;

    fn deref(&self) -> &Self::Target {
        self.value_ref
    }
}

impl<'r> YdbValueRef<'r> {
    pub(crate) fn new(value: &'r YdbValue) -> Self {
        Self { value_ref: value }
    }

    // pub(crate) fn as_ref(&self) -> &'r YdbValue {
    //     self.value_ref
    // }
}

impl<'r> ValueRef<'r> for YdbValueRef<'r> {
    type Database = Ydb;

    fn to_owned(&self) -> YdbValue {
        self.value_ref.clone()
    }

    fn type_info(&self) -> std::borrow::Cow<'_, YdbTypeInfo> {
        self.value_ref.type_info()
    }

    fn is_null(&self) -> bool {
        self.value_ref.is_null()
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct YdbValue {
    value: ydb::Value,
    type_info: YdbTypeInfo,
}

impl YdbValue {
    pub(crate) fn new(value: ydb::Value, type_info: YdbTypeInfo) -> Self {
        Self { value, type_info }
    }
}

impl Deref for YdbValue {
    type Target = ydb::Value;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Value for YdbValue {
    type Database = Ydb;

    fn as_ref(&self) -> YdbValueRef {
        YdbValueRef { value_ref: self }
    }

    fn type_info(&self) -> std::borrow::Cow<'_, YdbTypeInfo> {
        std::borrow::Cow::Borrowed(&self.type_info)
    }

    fn is_null(&self) -> bool {
        if let ydb::Value::Optional(t) = &self.value {
            if t.is_none() {
                return true;
            }
        }
        self.value == ydb::Value::Null
    }
}
