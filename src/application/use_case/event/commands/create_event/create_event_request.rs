use std::sync::Arc;
use serde::Deserialize;
use crate::application::dto::event_dto::EventDTO;
use crate::infrastructure::database::entities::event::Event;
use crate::infrastructure::repositories::event::insert_event::InsertEventRepository;

#[derive(Deserialize)]
pub struct CreateEventRequest{
    pub user_id: String,
    pub title: String,
    pub start_time: String,
    pub end_time: String,
    pub notes: String
}


