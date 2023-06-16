
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use error_stack::{Context, Report, ResultExt};
use tracing::log::Log;
use crate::application::OperationError;
use crate::application::use_case::user::user_login::UserLoginError::{DatabaseError, WrongEmailOrPassword};
use crate::infrastructure::database::entities::user::User;
use crate::infrastructure::repositories::user::find_email_user::FindEmailUserRepository;
use crate::infrastructure::repositories::user::find_id_user::FindIDUserRepository;
use crate::infrastructure::repositories::user::UserRepository;


pub struct UserLoginUseCase{
    repository: Arc<dyn FindEmailUserRepository +Sync+ Send>,
}

#[derive(Debug, Clone)]
pub enum UserLoginError {
    DatabaseError,
    WrongEmailOrPassword
}



impl Display for UserLoginError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UserLoginError::DatabaseError => write!(f, "資料庫出錯，請聯絡開發者"),
            UserLoginError::WrongEmailOrPassword => write!(f, "密碼或帳號錯誤")
        }
    }
}


impl OperationError for UserLoginError {
    fn status_code(&self) -> u16 {
        match self {
            UserLoginError::DatabaseError => 500,
            UserLoginError::WrongEmailOrPassword => 401
        }
    }
}


impl std::error::Error for UserLoginError{}

impl UserLoginUseCase{
    pub fn new(repository: Arc<UserRepository>)->Self{
        Self{
            repository
        }
    }

    pub async fn login(&self,email: String, password: String) -> Result<String,Report<UserLoginError>>{
        let result = self.repository.find_email_user(email)
            .await
            .change_context(UserLoginError::DatabaseError)?;

        if let Some(user) = result {
            if user.password == password {
                return Ok(user.id)
            }
        }

        return Err(Report::new(UserLoginError::WrongEmailOrPassword));

    }
}