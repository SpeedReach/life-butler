use std::error::Error;
use std::fmt::{Display, Formatter, write};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use error_stack::Context;
use crate::application::OperationError;

#[derive(Debug, Clone)]
pub enum CreateTaskError {
    DatabaseError,
    ParsingError(String),
    AlreadyExpired,
}

impl Display for CreateTaskError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateTaskError::DatabaseError => write!(f, "資料庫出錯，請聯絡管理員"),
            CreateTaskError::ParsingError(date) => write!(f, "無法將 {} 轉成日期", date),
            CreateTaskError::AlreadyExpired => write!(f, "不能創建已經過期的活動")
        }
    }
}

impl Context for CreateTaskError {}



impl OperationError for CreateTaskError {
    fn status_code(&self) -> u16 {
        match self {
            CreateTaskError::DatabaseError => 500,
            CreateTaskError::ParsingError(_) => 406,
            CreateTaskError::AlreadyExpired => 406
        }
    }
}
