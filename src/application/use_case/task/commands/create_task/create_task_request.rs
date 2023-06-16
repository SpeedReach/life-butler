use serde::Deserialize;
use crate::application::dto::event_dto::EventDTO;
use crate::infrastructure::database::entities::event::Event;
use crate::infrastructure::repositories::event::insert_event::InsertEventRepository;

#[derive(Deserialize)]
pub struct CreateTaskRequest{
    pub user_id: String,
    pub title: String,
    pub due: String,
    pub description: String
}


