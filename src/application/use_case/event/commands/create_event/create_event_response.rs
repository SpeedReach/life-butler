use serde::Serialize;
use crate::application::dto::event_dto::EventDTO;

#[derive(Serialize)]
pub struct CreateEventResponse{
    pub event: EventDTO
}