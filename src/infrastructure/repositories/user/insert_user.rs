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
use crate::infrastructure::results::create_result::InsertResult;


pub trait InsertUserRepository {
    async fn insert_user(&self, user: User) -> Result<InsertResult<User>,Report<DatabaseError>>;
}

impl InsertUserRepository for UserRepository {

    async fn insert_user(&self, user: User) -> Result<InsertResult<User>, Report<DatabaseError>> {

        let collection = self.get_collection();

        let result = collection.insert_one(&user, None).await;

        if (&result).is_err(){
            let err = result.unwrap_err();
            if let ErrorKind::Write(write_failure) = *err.clone().kind {
                if let WriteFailure::WriteError(error) = write_failure{
                    if error.code == 11000 {
                        return Ok(InsertResult::AlreadyExists(user.email.clone()));
                    }
                }
            }
            else {
                return Err(Report::new(err).change_context(DatabaseError));
            }
        }

        Ok(InsertResult::Success(user))
    }
}