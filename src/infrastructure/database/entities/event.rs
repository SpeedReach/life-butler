
use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Event{
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
    pub owner: String,
    pub start_time: DateTime,
    pub end_time: DateTime,
    pub notes: String
}

impl Event{
    pub fn new(
        title: String,
        owner: String,
        start_time: DateTime,
        end_time: DateTime,
        notes: String
    ) -> Self{
        Self{
            id: ObjectId::new().to_string(),
            title,
            owner,
            start_time,
            end_time,
            notes
        }
    }
}