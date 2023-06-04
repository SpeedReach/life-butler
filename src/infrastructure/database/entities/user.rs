use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User{
    #[serde(rename = "_id")]
    pub id: String,
    pub email: String,
    pub password: String
}

impl User{

    pub fn new(email: String,password: String)->User{
        return User{
            id: Uuid::new_v4().to_string(),
            email,
            password
        }
    }

}