use std::sync::Arc;
use error_stack::{Report, ResultExt};
use futures::SinkExt;
use crate::application::dto::event_dto::EventDTO;
use crate::application::use_case::event::queries::get_recent_events::get_recent_event_error::GetRecentEventError;
use crate::application::use_case::event::queries::get_recent_events::get_recent_event_request::GetRecentEventRequest;
use crate::application::use_case::event::queries::get_recent_events::get_recent_event_response::GetRecentEventResponse;
use crate::infrastructure::database::entities::event::Event;
use crate::infrastructure::repositories::event::EventRepository;
use crate::infrastructure::repositories::event::find_recent_events::FindRecentEventRepository;

pub struct GetRecentEventUseCase{
    repository: Arc<dyn FindRecentEventRepository + Send + Sync> 
}


impl GetRecentEventUseCase {
    pub fn new(repo: Arc<EventRepository>) -> Self {
        Self {
            repository: repo
        }
    }
    
    pub async fn get_events(&self,request: GetRecentEventRequest) -> Result<GetRecentEventResponse,Report<GetRecentEventError>>{
        let result = self.repository.find_recent_events(request.user_id, request.skip, request.count)
            .await
            .change_context(GetRecentEventError::DatabaseError)?;
        let size = &result.len();
        
        let mut events = Vec::<EventDTO>::new();
        for e in result {
            events.push(EventDTO::from(e));
        }
        return Ok(GetRecentEventResponse{ retrieved: *size, events });
    }
    
}