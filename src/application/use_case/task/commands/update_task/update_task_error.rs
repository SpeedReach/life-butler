use std::fmt::{Debug, Display, Formatter};
use error_stack::Context;
use serde::Serialize;
use crate::application::OperationError;

#[derive(Serialize,Clone, Debug)]
pub enum UpdateTaskError{
    DatabaseError
}

impl Display for UpdateTaskError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "資料庫出錯")
    }
}


impl Context for UpdateTaskError{

}

impl OperationError for UpdateTaskError{
    fn status_code(&self) -> u16 {
        500
    }
}