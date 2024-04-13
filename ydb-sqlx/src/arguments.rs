use std::ops::{Deref, DerefMut};

use sqlx_core::arguments::Arguments;

use crate::typeinfo::YdbTypeInfo;

use super::database::Ydb;

#[derive(Default)]
#[allow(unused)]
pub struct YdbArguments {
    // Types of each bind parameter
    pub(crate) types: Vec<YdbTypeInfo>,

    // Buffer of encoded bind parameters
    pub(crate) buffer: YdbArgumentBuffer,
}

impl<'q> Arguments<'q> for YdbArguments {
    type Database = Ydb;

    fn reserve(&mut self, _additional: usize, _size: usize) {
        //
    }

    fn add<T>(&mut self, _value: T)
    where
        T: 'q + Send + sqlx_core::encode::Encode<'q, Ydb> + sqlx_core::types::Type<Ydb>,
    {
        self.types.push(T::type_info());
    }
}

#[allow(unused)]
#[derive(Default)]
pub struct YdbArgumentBuffer {
    // buffer: Vec<u8>,

    // Number of arguments
    count: usize,

    // Whenever an `Encode` impl needs to defer some work until after we resolve parameter types
    // it can use `patch`.
    //
    // This currently is only setup to be useful if there is a *fixed-size* slot that needs to be
    // tweaked from the input type. However, that's the only use case we currently have.
    //
    patches: Vec<(
        usize, // offset
        usize, // argument index
        Box<dyn Fn(&mut [u8], &YdbTypeInfo) + 'static + Send + Sync>,
    )>,

    // Whenever an `Encode` impl encounters a `YdbTypeInfo` object that does not have an OID
    // It pushes a "hole" that must be patched later.
    //
    // The hole is a `usize` offset into the buffer with the type name that should be resolved
    // This is done for Records and Arrays as the OID is needed well before we are in an async
    // function and can just ask ydb.
    //
    type_holes: Vec<(usize, String)>, // Vec<{ offset, type_name }>
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
