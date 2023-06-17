use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct UserRoutine{
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub user_id: String,
    pub sleeping: bool,
    pub last_wakeup: DateTime,
    pub last_eaten: DateTime
}

impl UserRoutine{
    pub fn new(user_id: String) -> Self{
        Self{
            id: ObjectId::new(),
            user_id,
            sleeping: false,
            last_wakeup: DateTime::now(),
            last_eaten: DateTime::now()
        }
    }
}