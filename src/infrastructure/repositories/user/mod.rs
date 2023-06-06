use crate::infrastructure::database::entities::user::User;
use std::error::{Error};
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use error_stack::{Report};
use mongodb::bson::doc;
use mongodb::IndexModel;
use mongodb::options::IndexOptions;
use crate::infrastructure::database::database_service::DatabaseDriver;

pub mod insert_user;
pub mod delete_email_user;


pub struct UserRepository {
    driver: Arc<DatabaseDriver>,
    database_id: String,
    collection_id: String,
}

impl UserRepository {
    pub async fn new(driver: &Arc<DatabaseDriver>) -> UserRepository {
        let options = IndexOptions::builder().unique(true).build();
        let model = IndexModel::builder()
            .keys(doc! {"email": 1})
            .options(options)
            .build();
        let create_result= driver
            .get_database()
            .collection::<User>("user")
            .create_index(model, None)
            .await.unwrap();
        
        UserRepository { 
            driver: Arc::clone(driver),
            database_id: driver.get_config().database_id,
            collection_id: String::from("user")
        }
    }
    
    
    fn get_collection(&self) -> mongodb::Collection<User> {
        self.driver
            .get_database()
            .collection::<User>(&self.collection_id)
    }
    
}

