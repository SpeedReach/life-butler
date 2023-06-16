use std::sync::Arc;
use error_stack::{Report, ResultExt};
use crate::application::use_case::task::commands::update_task::update_task_error::UpdateTaskError;
use crate::application::use_case::task::commands::update_task::update_task_request::UpdateTaskRequest;
use crate::infrastructure::repositories::task::TaskRepository;
use crate::infrastructure::repositories::task::update_task_status::UpdateTaskStatusRepository;

pub struct UpdateTaskUseCase{
    repository: Arc<dyn UpdateTaskStatusRepository + Sync + Send>
}

impl UpdateTaskUseCase{

    pub fn new(repo: Arc<TaskRepository>)->Self{
        Self{
            repository: repo
        }
    }


    pub async fn update_task(&self,request: UpdateTaskRequest) -> Result<(), Report<UpdateTaskError>>{
        let result = self.repository.update_task_status(request.task_id,request.is_done)
            .await
            .change_context(UpdateTaskError::DatabaseError)?;
        Ok(())
    }

}