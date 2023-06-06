use std::error::Error;
use std::fmt::{Display, Formatter};
use error_stack::{ IntoReport, Report, ResultExt};
use futures::future::err;
use mongodb::{ClientSession, Collection};
use mongodb::bson::doc;
use mongodb::error::{ErrorKind, WriteFailure};
use serde_json::error::Category::Data;
use tokio::fs::read_to_string;
use crate::infrastructure::database::entities::user::User;
use crate::infrastructure::error::DatabaseError;
use super::*;
use crate::infrastructure::repositories::user::UserRepository;
use crate::infrastructure::results::create_result::CreateResult;


pub trait CreateUserRepository{
    async fn create_user(&self, email: String, password: String) -> Result<CreateResult<User>,Report<DatabaseError>>;
}

impl CreateUserRepository for UserRepository {

    async fn create_user(&self, email: String, password: String) -> Result<CreateResult<User>, Report<DatabaseError>> {
        let user = User::new(email.clone(),password);

        let collection = self.get_collection();

        let result = collection.insert_one(&user, None).await;

        if (&result).is_err(){
            let err = result.unwrap_err();
            if let ErrorKind::Write(write_failure) = *err.clone().kind {
                if let WriteFailure::WriteError(error) = write_failure{
                    if error.code == 11000 {
                        return Ok(CreateResult::AlreadyExists(email.clone()));
                    }
                }
            }
            else {
                return Err(Report::new(err).change_context(DatabaseError));
            }
        }

        Ok(CreateResult::Success(user))
    }
}