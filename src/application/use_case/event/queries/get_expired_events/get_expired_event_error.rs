use std::fmt::{Display, Formatter};
use error_stack::Context;
use crate::application::OperationError;

#[derive(Debug, Clone)]
pub enum GetExpiredEventError {
    DatabaseError
}

impl Display for GetExpiredEventError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GetExpiredEventError::DatabaseError => write!(f, "資料庫出錯，請聯絡管理員"),
        }
    }
}

impl Context for GetExpiredEventError {}



impl OperationError for GetExpiredEventError {
    fn status_code(&self) -> u16 {
        match self {
            GetExpiredEventError::DatabaseError => 500
        }
    }
}
