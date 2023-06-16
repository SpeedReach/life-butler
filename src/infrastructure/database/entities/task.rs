use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Task{
    #[serde(rename = "_id")]
    pub id: String,
    pub owner: String,
    pub title: String,
    pub due: DateTime,
    pub description: String,
    pub is_done: bool
}

impl Task{
    pub fn new(
        owner: String,
        title: String,
        due: DateTime,
        description: String,
    ) -> Self{
        Self{
            id: ObjectId::new().to_string(),
            owner,
            title,
            due,
            description,
            is_done: false
        }
    }
}