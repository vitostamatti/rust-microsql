use std::rc::Rc;

use crate::{
    table::{ColumnInfo, StoredRow},
    ExecutionError,
};

#[derive(Debug)]
pub struct Row<'a> {
    id: usize,
    columns: Rc<ColumnInfo>,
    data: &'a StoredRow,
}

impl<'a> Row<'a> {
    pub fn new(columns: Rc<ColumnInfo>, id: usize, data: &'a StoredRow) -> Self {
        Self { id, columns, data }
    }

    pub fn columns(&self) -> &ColumnInfo {
        &self.columns.as_ref()
    }

    pub fn get(&self, column: &String) -> String {
        self.try_get(column).unwrap()
    }

    pub fn try_get(&self, column: &String) -> Result<String, ExecutionError> {
        self.data.get(column).map_or_else(
            || Err(ExecutionError::ColumnDoesNotExists(column.to_owned())),
            |val| Ok(val.to_string()),
        )
    }

    pub fn id(&self) -> usize {
        self.id
    }
}
