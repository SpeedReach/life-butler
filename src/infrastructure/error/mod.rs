use std::error::Error;
use std::fmt::{Display, Write};
use error_stack::{Context, Report};

#[derive(Debug, Clone)]
pub enum OperationError{
    InvalidOperation(String),
    InternalError
}


impl Display for OperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationError::InvalidOperation(info) => write!(f, "{}", info),
            OperationError::InternalError => f.write_str("An internal error occurred while executing this operation")
        }
    }
}

impl Context for OperationError {

}