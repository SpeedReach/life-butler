use std::slice::from_raw_parts_mut;
use crate::{
    infrastructure::error::OperationError,
};
use async_trait::async_trait;
use mongodb::{Client, ClientSession, Collection};
use std::sync::Arc;
use error_stack::{IntoReport, Report, ResultExt};
use futures::future::BoxFuture;
use futures::FutureExt;
use mongodb::bson::{Bson, doc};
use mongodb::change_stream::event::OperationType::Insert;
use mongodb::options::{InsertOneOptions, SessionOptions};
use mongodb::results::DeleteResult;
use crate::infrastructure::database::database_service::DatabaseDriver;
use crate::infrastructure::database::entities::user::User;

pub struct UserRepository {
    driver: Arc<DatabaseDriver>,
}


impl UserRepository {
    pub fn new(driver: Arc<DatabaseDriver>) -> UserRepository {
        UserRepository { driver }
    }

    pub async fn create_user(
        &self,
        email: String,
        password: String,
    ) -> Result<User, Report<OperationError>> {

        let client = self.driver.get_client();
        let db_config = self.driver.get_config();

        let mut session = client.start_session(None).await
            .into_report()
            .change_context(OperationError::InternalError)?;

        async fn callback(session: &mut ClientSession,email: String, password: String, database_id: String) -> Result<Result<User,Report<OperationError>>,mongodb::error::Error>{
            let collection: Collection<User> = session.client().database(database_id.as_str()).collection("user");
            let filter = doc! { "email": &email };
            let mut cursor = collection.find(filter, None).await?;

            if cursor.advance().await? {
                session.abort_transaction();
                return Ok(Err(Report::new(OperationError::InvalidOperation("此email已經註冊過".to_owned()))));
            }

            let user = User::new(email,password);
            let result = collection.insert_one(&user, None).await?;

            print!("{}", &user.id);
            print!("{}", result.inserted_id);

            return  Ok(Ok(user));
        }

        let result = session.with_transaction(
            (),
            |session,_| callback(session, (&email).to_owned(),(&password).to_owned(), (&db_config.database_id).to_owned()).boxed(),
            None
        ).await;

        match result {
            Ok(r)=> r,
            Err(err)=> Err(Report::new(OperationError::InternalError).attach_printable(err))
        }
    }


    pub async fn delete_user_with_email(&self,email: String)->Result<DeleteResult,Report<OperationError>> {
        let database=self.driver.get_database();
        let collection: Collection<User> =database.collection("user");
        return Ok(collection.delete_one(doc! {"email": email},None)
            .await
            .into_report()
            .change_context(OperationError::InternalError)?)
    }

}
