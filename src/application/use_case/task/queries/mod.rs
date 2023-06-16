pub mod get_ongoing_tasks;
pub mod get_expired_tasks;
use std::fmt::{Display, Formatter};
use error_stack::Context;
use serde::Serialize;
use crate::application::OperationError;

#[derive(Debug, Clone)]
pub enum GetTasksError {
    DatabaseError
}

impl Display for GetTasksError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GetTasksError::DatabaseError => write!(f, "資料庫出錯，請聯絡管理員"),
        }
    }
}

impl Context for GetTasksError {}



impl OperationError for GetTasksError {
    fn status_code(&self) -> u16 {
        match self {
            GetTasksError::DatabaseError => 500
        }
    }
}
