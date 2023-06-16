use chrono::{DateTime as dt, FixedOffset, Utc};
use crate::infrastructure::database::entities::event::Event;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct EventDTO{
    pub id: String,
    pub title: String,
    pub owner: String,
    pub start_time: dt<Utc>,
    pub end_time: dt<Utc>,
    pub notes: String
}

impl From<Event> for EventDTO{
    fn from(value: Event) -> Self {
        Self{
            id: value.id,
            title: value.title,
            owner: value.owner,
            start_time: value.start_time.to_chrono(),
            end_time: value.end_time.to_chrono(),
            notes: value.notes
        }
    }
}