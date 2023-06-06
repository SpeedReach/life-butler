
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use error_stack::{Context, Report, ResultExt};
use tracing::log::Log;
use crate::application::use_case::user::user_login::UserLoginError::{DatabaseError, WrongEmailOrPassword};
use crate::infrastructure::repositories::user::user_login::LoginRepository;
use crate::infrastructure::repositories::user::UserRepository;
use crate::infrastructure::results::login_result::LoginResult;


pub struct UserLoginUseCase{
    repository: Arc<UserRepository>,
}

#[derive(Debug, Clone)]
pub enum UserLoginError {
    DatabaseError,
    WrongEmailOrPassword
}

impl Display for UserLoginError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UserLoginError::DatabaseError => write!(f, "database error"),
            UserLoginError::WrongEmailOrPassword => write!(f, "Login failed, wrong email or password")
        }
    }
}

impl IntoResponse for UserLoginError {
    fn into_response(self) -> Response {
        match self {
            UserLoginError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response(),
            UserLoginError::WrongEmailOrPassword => (StatusCode::UNAUTHORIZED, self.to_string()).into_response()
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
        let result = self.repository.login(email,password)
            .await
            .change_context(UserLoginError::DatabaseError)?;
        match result {
            LoginResult::Success(user_id) => Ok(user_id),
            LoginResult::Failed => Err(Report::new(UserLoginError::WrongEmailOrPassword))
        }

    }
}