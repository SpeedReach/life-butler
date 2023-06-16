use serde::Serialize;
use crate::application::dto::event_dto::EventDTO;

#[derive(Serialize)]
pub struct GetRecentEventResponse{
    pub retrieved: usize,
    pub events: Vec<EventDTO>
}