use miette::Diagnostic;
use sqlmicro_execution::ExecutionError;
use sqlmicro_parser::error::FormattedError;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error(transparent)]
pub enum SqlMicroError<'a> {
    #[diagnostic(transparent)]
    ExecutionError(#[from] ExecutionError),
    #[diagnostic(transparent)]
    ParsingError(FormattedError<'a>),
}

impl<'a> From<FormattedError<'a>> for SqlMicroError<'a> {
    fn from(value: FormattedError<'a>) -> Self {
        Self::ParsingError(value)
    }
}
