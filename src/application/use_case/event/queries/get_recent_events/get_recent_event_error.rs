use std::fmt::{Display, Formatter};
use error_stack::Context;
use crate::application::OperationError;

#[derive(Debug, Clone)]
pub enum GetRecentEventError {
    DatabaseError
}

impl Display for GetRecentEventError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GetRecentEventError::DatabaseError => write!(f, "資料庫出錯，請聯絡管理員"),
        }
    }
}

impl Context for GetRecentEventError {}



impl OperationError for GetRecentEventError {
    fn status_code(&self) -> u16 {
        match self {
            GetRecentEventError::DatabaseError => 500
        }
    }
}
