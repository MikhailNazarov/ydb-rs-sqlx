use std::fmt::Display;

use sqlx_core::type_info::TypeInfo;

#[derive(PartialEq, Clone, Debug)]
pub struct YdbTypeInfo {}

impl TypeInfo for YdbTypeInfo {
    fn is_null(&self) -> bool {
        todo!()
    }

    fn name(&self) -> &str {
        todo!()
    }
}

impl Display for YdbTypeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
