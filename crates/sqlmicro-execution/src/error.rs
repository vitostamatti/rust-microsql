use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("Execution Error")]
pub enum ExecutionError {
    #[error("Table {0} was not found")]
    TableNotFound(String),
    #[error("Table {0} already exists")]
    TableAlreadyExists(String),
    #[error("Column {0} does not exists")]
    ColumnDoesNotExists(String),
}
