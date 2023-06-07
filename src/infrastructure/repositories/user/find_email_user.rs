use error_stack::{IntoReport, ResultExt, Report};
use mongodb::bson::doc;
use crate::infrastructure::database::entities::user::User;
use crate::infrastructure::error::DatabaseError;
use crate::infrastructure::repositories::user::UserRepository;

pub trait FindEmailUserRepository{
    async fn find_email_user(&self,email: String)->Result<Option<User>,Report<DatabaseError>>;
}

impl FindEmailUserRepository for UserRepository {
    async fn find_email_user(&self,email: String) -> Result<Option<User>, Report<DatabaseError>> {
        let collection = self.get_collection();
        let filter = doc!{"email": email};

        let find_result = collection.find_one(filter,None).await
            .into_report()
            .change_context(DatabaseError)?;
        return Ok(find_result);
    }
}