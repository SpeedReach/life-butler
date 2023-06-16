use chrono::{DateTime, FixedOffset, Utc};
use crate::infrastructure::database::entities::event::Event;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct EventDTO{
    pub id: String,
    pub title: String,
    pub owner: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub notes: String,
    pub state: String
}

// EXPIRED GOING_ON FUTURE

impl From<Event> for EventDTO{
    fn from(value: Event) -> Self {
        let now = Utc::now();
        let start_time = value.start_time.to_chrono();
        let end_time = value.end_time.to_chrono();
        let mut state = "";
        if end_time < now {
            state = "EXPIRED";
        }
        else if now < start_time{
            state = "FUTURE";
        }
        else{
            state = "GOING_ON";
        }
        Self{
            id: value.id,
            title: value.title,
            owner: value.owner,
            start_time,
            end_time,
            notes: value.notes,
            state: state.to_owned()
        }
    }
}