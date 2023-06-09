use crate::infrastructure::database::entities::user::User;
use crate::infrastructure::repositories::user::insert_user::InsertUserRepository;
use crate::infrastructure::repositories::user::UserRepository;
use crate::infrastructure::results::insert_result::InsertResult;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use error_stack::{Context, Report, ResultExt};
use std::fmt::{write, Display, Formatter};
use std::sync::Arc;
use crate::application::OperationError;

pub struct RegisterUserUseCase {
    repository: Arc<dyn InsertUserRepository + Sync + Send>,
}

#[derive(Debug, Clone)]
pub enum RegisterUserError {
    DatabaseError,
    EmailAlreadyExists(String),
}

impl Display for RegisterUserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RegisterUserError::DatabaseError => write!(f, "資料庫出錯，請聯絡開發者"),
            RegisterUserError::EmailAlreadyExists(email) => {
                write!(f, "此Email已被註冊： {}", email)
            }
        }
    }
}


impl Context for RegisterUserError {}

impl OperationError for RegisterUserError{
    fn status_code(&self) -> u16 {
        match self {
            RegisterUserError::DatabaseError => 500,
            RegisterUserError::EmailAlreadyExists(_) => 406
        }
    }
}

impl RegisterUserUseCase {
    pub fn new(repository: Arc<UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn register_user(
        &self,
        email: String,
        password: String,
    ) -> Result<User, Report<RegisterUserError>> {
        let result = self
            .repository
            .insert_user(User::new(email, password))
            .await
            .change_context(RegisterUserError::DatabaseError)?;
        match result {
            InsertResult::AlreadyExists(email) => {
                Err(Report::new(RegisterUserError::EmailAlreadyExists(email)))
            }
            InsertResult::Success(user) => Ok(user),
        }
    }
}
