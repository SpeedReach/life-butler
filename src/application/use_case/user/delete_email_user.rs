
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use axum::http::{StatusCode};
use axum::response::{IntoResponse, Response};
use error_stack::{Context, Report, ResultExt};
use crate::application::use_case::user::delete_email_user::DeleteEmailUserError::NotFound;
use crate::driver::module::Modules;
use crate::infrastructure::repositories::user::delete_email_user::DeleteEmailUserRepository;
use crate::infrastructure::repositories::user::UserRepository;
use crate::infrastructure::results::delete_result::DeleteResult;


#[derive(Debug,Clone)]
pub enum DeleteEmailUserError{
    DatabaseError,
    NotFound(String)
}

pub struct DeleteEmailUserUseCase{
    repository: Arc<dyn DeleteEmailUserRepository+Sync+Send>
}

impl Display for DeleteEmailUserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeleteEmailUserError::DatabaseError => write!(f, "database error"),
            DeleteEmailUserError::NotFound(email) => write!(f, "user with email {}, can not be found ", email)
        }
    }
}

impl IntoResponse for DeleteEmailUserError{
    fn into_response(self) -> Response {
        match self {
            DeleteEmailUserError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response(),
            DeleteEmailUserError::NotFound(_) => (StatusCode::FORBIDDEN, self.to_string()).into_response()
        }
    }
}


impl Context for DeleteEmailUserError{

}


impl DeleteEmailUserUseCase{
    
    pub fn new(repo: Arc<UserRepository>)->Self{
        Self{
            repository: repo
        }
    }
    
    pub async fn delete_email_user(&self,email: String)->Result<(),Report<DeleteEmailUserError>>{
        let result =self.repository.delete_email_user(email.clone())
            .await
            .change_context(DeleteEmailUserError::DatabaseError)?;
        
        match result {
            DeleteResult::NotFound => Err(Report::new(DeleteEmailUserError::NotFound(email))),
            DeleteResult::Success(_) => Ok(())
        }
        
    }
    
}