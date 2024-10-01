use std::collections::HashMap;

use rustring_builder::StringBuilder;
use sqlx_core::{executor::Execute, Error};

use crate::database::Ydb;

#[derive(Default)]
pub struct YdbQueryResult {
    pub(crate) rows_affected: u64,
}

impl YdbQueryResult {
    pub fn rows_affected(&self) -> u64 {
        self.rows_affected
    }
}

impl Extend<YdbQueryResult> for YdbQueryResult {
    fn extend<T: IntoIterator<Item = YdbQueryResult>>(&mut self, iter: T) {
        for elem in iter {
            self.rows_affected += elem.rows_affected;
        }
    }
}
#[derive(Clone)]
pub(crate) struct ParsedQuery {
    sql: String,
    params: HashMap<String, ydb::Value>,
}

impl ParsedQuery {
    pub(crate) fn sql(&self) -> &str {
        &self.sql
    }
}

impl From<ParsedQuery> for ydb::Query {
    fn from(value: ParsedQuery) -> Self {
        let query = ydb::Query::new(value.sql);
        if !value.params.is_empty() {
            return query.with_params(value.params);
        }
        query
    }
}

pub(crate) fn build_query<'q, E: 'q>(mut query: E) -> Result<ParsedQuery, Error>
where
    E: Execute<'q, Ydb>,
{
    let mut sb = StringBuilder::new();
    let mut params = HashMap::new();

    if let Some(arguments) = query
        .take_arguments()
        .map_err(|e| sqlx_core::error::Error::AnyDriverError(e))?
    {
        for arg in arguments.into_iter() {
            arg.declare(&mut sb);
            arg.add_to_params(&mut params);
        }
        sb.append_line("");
    }

    sb.append(query.sql());

    let sql = sb.to_string();

    Ok(ParsedQuery { sql, params })
}
