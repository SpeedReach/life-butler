
use std::fmt::{Display, Formatter, write};
use std::sync::Arc;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use error_stack::{Context, Report, ResultExt};
use crate::infrastructure::database::entities::user::User;
use crate::infrastructure::repositories::user::insert_user::InsertUserRepository;
use crate::infrastructure::repositories::user::UserRepository;
use crate::infrastructure::results::create_result::InsertResult;


pub struct RegisterUserUseCase{
    repository: Arc<UserRepository>,
}

#[derive(Debug,Clone)]
pub enum RegisterUserError{
    DatabaseError,
    EmailAlreadyExists(String)
}

impl Display for RegisterUserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RegisterUserError::DatabaseError => write!(f, "database error"),
            RegisterUserError::EmailAlreadyExists(email) => write!(f, "email: {}  has already been registered", email)
        }
    }
}

impl IntoResponse for RegisterUserError{
    fn into_response(self) -> Response {
        match self {
            RegisterUserError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response(),
            RegisterUserError::EmailAlreadyExists(_) => (StatusCode::FORBIDDEN, self.to_string()).into_response()
        }
    }
}

impl Context for RegisterUserError{

}


impl RegisterUserUseCase{

    pub fn new(repository: Arc<UserRepository>)->Self{
        Self{
            repository
        }
    }

    pub async fn register_user(&self,email: String,password: String)->Result<User,Report<RegisterUserError>>{
        let result = self.repository.insert_user(User::new(email,password))
            .await
            .change_context(RegisterUserError::DatabaseError)?;
        match result {
            InsertResult::AlreadyExists(email) => Err(Report::new(RegisterUserError::EmailAlreadyExists(email))),
            InsertResult::Success(user) => Ok(user)
        }
    }

}