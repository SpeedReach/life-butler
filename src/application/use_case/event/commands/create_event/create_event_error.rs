use std::error::Error;
use std::fmt::{Display, Formatter, write};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use error_stack::Context;
use crate::application::OperationError;

#[derive(Debug, Clone)]
pub enum CreateEventError{
    DatabaseError,
    ParsingError(String),
    AlreadyExpired,
    StartTimeBigger
}

impl Display for CreateEventError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateEventError::DatabaseError => write!(f,"資料庫出錯，請聯絡管理員"),
            CreateEventError::ParsingError(date) => write!(f,"無法將 {} 轉成日期",date),
            CreateEventError::AlreadyExpired => write!(f,"不能創建已經過期的活動"),
            CreateEventError::StartTimeBigger => write!(f,"開始日期必需小於結束日期")
        }
    }
}

impl Context for CreateEventError {}



impl OperationError for CreateEventError{
    fn status_code(&self) -> u16 {
        match self {
            CreateEventError::DatabaseError => 500,
            CreateEventError::ParsingError(_) => 406,
            CreateEventError::AlreadyExpired => 406,
            CreateEventError::StartTimeBigger=>406
        }
    }
}
