use std::error::Error;
use std::fmt::{Display, Formatter};
use error_stack::{IntoReport, Report, ResultExt};
use mongodb::bson::doc;
use mongodb::Collection;
use crate::infrastructure::database::entities::user::User;
use crate::infrastructure::error::DatabaseError;
use crate::infrastructure::repositories::user::UserRepository;
use crate::infrastructure::results::delete_result::DeleteResult;


pub trait DeleteEmailUserRepository {
    async fn delete_email_user(&self, email: String) -> Result<DeleteResult, Report<DatabaseError>>;
}

impl DeleteEmailUserRepository for UserRepository{
    async fn delete_email_user(&self, email: String) -> Result<DeleteResult, Report<DatabaseError>> {
        let database=self.driver.get_database();
        let collection: Collection<User> =database.collection("user");
        let result = collection.delete_one(doc! {"email": email},None)
            .await
            .into_report()
            .change_context(DatabaseError)?;
        return Ok(DeleteResult::new(result));
    }
}