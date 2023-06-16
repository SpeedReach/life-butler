use std::sync::Arc;
use chrono::DateTime;
use mongodb::bson::DateTime as DT;
use chrono::format::Item::Fixed;
use error_stack::{IntoReport, Report, ResultExt};
use crate::application::dto::event_dto::EventDTO;
use crate::application::use_case::event::commands::create_event::create_event_error::CreateEventError;
use crate::application::use_case::event::commands::create_event::create_event_request::CreateEventRequest;
use crate::application::use_case::event::commands::create_event::create_event_response::CreateEventResponse;
use crate::infrastructure::database::entities::event::Event;
use crate::infrastructure::repositories::event::EventRepository;
use crate::infrastructure::repositories::event::insert_event::InsertEventRepository;
use crate::infrastructure::results::insert_result::InsertResult;

pub struct CreateEventUseCase{
    repository: Arc<dyn InsertEventRepository+Sync+Send>
}



impl CreateEventUseCase{
    pub fn new(repo: Arc<EventRepository>)->Self{
        Self{
            repository: repo
        }
    }
    
    pub async fn create_event(&self,request: CreateEventRequest) -> Result<CreateEventResponse,Report<CreateEventError>>{
        let start_time = DateTime::parse_from_rfc3339(request.start_time.as_str())
            .into_report()
            .change_context(CreateEventError::ParsingError(request.start_time))?;
        let end_time = DateTime::parse_from_rfc3339(request.end_time.as_str())
            .into_report()
            .change_context(CreateEventError::ParsingError(request.end_time))?;
        let event = Event::new(
            request.title,
            request.user_id,
            DT::from(start_time),
            DT::from(end_time),
            request.notes
        );
        let result = self.repository.insert_event(event)
            .await
            .change_context(CreateEventError::DatabaseError)?;
        match result {
            InsertResult::Success(event) => Ok(CreateEventResponse{event: EventDTO::from(event)}),
            _ => Err(Report::new(CreateEventError::DatabaseError))
        }
    }
}