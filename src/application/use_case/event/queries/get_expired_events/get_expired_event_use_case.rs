use std::sync::Arc;
use error_stack::{Report, ResultExt};
use futures::SinkExt;
use crate::application::dto::event_dto::EventDTO;
use crate::application::use_case::event::queries::get_expired_events::get_expired_event_error::GetExpiredEventError;
use crate::application::use_case::event::queries::get_expired_events::get_expired_event_request::GetExpiredEventRequest;
use crate::application::use_case::event::queries::get_expired_events::get_expired_event_response::GetExpiredEventResponse;
use crate::application::use_case::event::queries::get_recent_events::get_recent_event_error::GetRecentEventError;
use crate::infrastructure::database::entities::event::Event;
use crate::infrastructure::repositories::event::EventRepository;
use crate::infrastructure::repositories::event::find_expired_events::FindExpiredEventRepository;

pub struct GetExpiredEventUseCase {
    repository: Arc<dyn FindExpiredEventRepository + Send + Sync>
}


impl GetExpiredEventUseCase {
    pub fn new(repo: Arc<EventRepository>) -> Self {
        Self {
            repository: repo
        }
    }
    
    pub async fn get_events(&self,request: GetExpiredEventRequest) -> Result<GetExpiredEventResponse,Report<GetExpiredEventError>>{
        let result = self.repository.find_expired_events(request.user_id, request.skip, request.count)
            .await
            .change_context(GetExpiredEventError::DatabaseError)?;
        let size = &result.len();
        
        let mut events = Vec::<EventDTO>::new();
        for e in result {
            events.push(EventDTO::from(e));
        }
        return Ok(GetExpiredEventResponse{ retrieved: *size, events });
    }
    
}