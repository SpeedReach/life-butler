use axum::async_trait;
use error_stack::{IntoReport, Report, ResultExt};
use mongodb::bson::doc;
use crate::infrastructure::database::entities::user::User;
use crate::infrastructure::error::DatabaseError;
use crate::infrastructure::repositories::user::UserRepository;

#[async_trait]
pub trait FindIDUserRepository{
    async fn find_id_user(&self, user_id: String) -> Result<Option<User>, Report<DatabaseError>>;
}

#[async_trait]
impl FindIDUserRepository for UserRepository {

    async fn find_id_user(&self, user_id: String) -> Result<Option<User>, Report<DatabaseError>> {
        let collection = self.get_collection();
        let filter = doc!{"_id": user_id};

        let find_result = collection.find_one(filter,None).await
            .into_report()
            .change_context(DatabaseError)?;
        return Ok(find_result);
    }

}