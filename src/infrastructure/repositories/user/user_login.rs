use error_stack::{IntoReport, Report, ResultExt};
use mongodb::bson::doc;
use crate::infrastructure::database::entities::user::User;
use crate::infrastructure::error::DatabaseError;
use crate::infrastructure::repositories::user::UserRepository;
use crate::infrastructure::results::login_result::LoginResult;


pub trait LoginRepository{
    async fn login(&self, email: String,password: String) -> Result<LoginResult, Report<DatabaseError>>;
}


impl LoginRepository for UserRepository {

    async fn login(&self, email: String, password: String) -> Result<LoginResult, Report<DatabaseError>>{
        let collection = self.get_collection();
        let filter = doc!{"email": email, "password": password};
        
        let find_result = collection.find_one(filter,None).await
            .into_report()
            .change_context(DatabaseError)?;
        match find_result {
            None => Ok(LoginResult::Failed),
            Some(user) => Ok(LoginResult::Success(user.id))
        }
    }
}