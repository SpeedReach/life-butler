use std::sync::Arc;
use chrono::{DateTime, Utc};
use mongodb::bson::DateTime as DT;
use chrono::format::Item::Fixed;
use error_stack::{IntoReport, Report, ResultExt};
use crate::application::dto::task_dto::TaskDTO;
use crate::application::use_case::task::commands::create_task::create_task_error::CreateTaskError;
use crate::application::use_case::task::commands::create_task::create_task_request::CreateTaskRequest;
use crate::application::use_case::task::commands::create_task::create_task_response::CreateTaskResponse;
use crate::infrastructure::database::entities::task::Task;
use crate::infrastructure::repositories::task::insert_task::InsertTaskRepository;
use crate::infrastructure::repositories::task::TaskRepository;
use crate::infrastructure::results::insert_result::InsertResult;

pub struct CreateTaskUseCase {
    repository: Arc<dyn InsertTaskRepository+Sync+Send>
}



impl CreateTaskUseCase {
    pub fn new(repo: Arc<TaskRepository>)->Self{
        Self{
            repository: repo
        }
    }
    
    pub async fn create_task(&self,request: CreateTaskRequest) -> Result<CreateTaskResponse,Report<CreateTaskError>>{
        let due = DateTime::parse_from_rfc3339(request.due.as_str())
            .into_report()
            .change_context(CreateTaskError::ParsingError(request.due))?;
        if due < Utc::now() {
            return Err(Report::new(CreateTaskError::AlreadyExpired));
        }
        let task = Task::new(
            request.user_id,
            request.title,
            DT::from(due),
            request.description
        );
        let result = self.repository.insert_task(task)
            .await
            .change_context(CreateTaskError::DatabaseError)?;
        match result {
            InsertResult::Success(task) => Ok(CreateTaskResponse{event: TaskDTO::from(task)}),
            _ => Err(Report::new(CreateTaskError::DatabaseError))
        }
    }
}