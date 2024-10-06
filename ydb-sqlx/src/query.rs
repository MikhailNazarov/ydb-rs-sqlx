use std::collections::HashMap;

use rustring_builder::StringBuilder;
use sqlx_core::{executor::Execute, Error};

use crate::{arguments::YdbArguments, database::Ydb};

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

pub(crate) fn build_query<'q, E: 'q + Execute<'q, Ydb>>(
    mut query: E,
) -> Result<ParsedQuery, Error> {
    let arguments = query
        .take_arguments()
        .map_err(sqlx_core::error::Error::AnyDriverError)?;

    build_query_from_parts(query.sql(), arguments)
}

pub(crate) fn build_query_from_parts(
    sql: &str,
    arguments: Option<YdbArguments>,
) -> Result<ParsedQuery, Error> {
    let mut sb = StringBuilder::new();
    let mut params = HashMap::new();

    if let Some(arguments) = arguments {
        if !arguments.is_empty() {
            for arg in arguments.into_iter() {
                arg.declare(&mut sb);
                arg.add_to_params(&mut params);
            }
            sb.append_line("");
        }
    }

    sb.append(sql);

    let sql = sb.to_string();

    Ok(ParsedQuery { sql, params })
}
