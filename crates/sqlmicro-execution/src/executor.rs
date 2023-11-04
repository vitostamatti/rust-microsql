use std::collections::HashMap;

use derive_more::Display;
use sqlmicro_parser::query::SqlQuery;

use crate::{error::ExecutionError, row::Row, table::Table};

#[derive(Debug, Display)]
pub enum ExecutionResponse<'a> {
    #[display(fmt = "{_0:?}")]
    Select(Vec<Row<'a>>),
    Insert,
    Create,
}

#[derive(Debug, Default)]
pub struct Executor {
    tables: HashMap<String, Table>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }
    pub fn run(&mut self, query: SqlQuery) -> Result<ExecutionResponse, ExecutionError> {
        match query {
            SqlQuery::Select(select) => {
                let table = select.table;
                let table = self
                    .tables
                    .get(&table)
                    .ok_or(ExecutionError::TableNotFound(table))?;

                let rows = table.iter().collect();
                Ok(ExecutionResponse::Select(rows))
            }
            SqlQuery::Insert(insert) => {
                let table = self
                    .tables
                    .get_mut(&insert.table)
                    .ok_or(ExecutionError::TableNotFound(insert.table))?;

                // TODO: convert inside insert ?
                table.insert(insert.values.into_iter().map(|v| v.to_string()).collect());

                Ok(ExecutionResponse::Insert)
            }
            SqlQuery::Create(create) => {
                let table = Table::new(create.columns);

                self.tables.insert(create.table, table);

                Ok(ExecutionResponse::Create)
            }
        }
    }
}
