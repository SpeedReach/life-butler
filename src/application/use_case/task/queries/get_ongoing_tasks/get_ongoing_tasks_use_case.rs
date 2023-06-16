use std::cmp::{min, Ordering};
use std::sync::Arc;
use error_stack::{Report, ResultExt};
use crate::application::dto::task_dto::TaskDTO;
use crate::application::use_case::task::queries::get_ongoing_tasks::get_ongoing_tasks_request::GetOnGoingTasksRequest;
use crate::application::use_case::task::queries::get_ongoing_tasks::get_ongoing_tasks_response::GetGoingTasksResponse;
use crate::application::use_case::task::queries::GetTasksError;
use crate::infrastructure::database::entities::task::Task;
use crate::infrastructure::repositories::task::find_user_tasks::FindUserTasksRepository;
use crate::infrastructure::repositories::task::TaskRepository;

pub struct GetGoingTasksUseCase {
    repository: Arc<dyn FindUserTasksRepository + Send + Sync>
}

impl GetGoingTasksUseCase {
    pub fn new(repo: Arc<TaskRepository>)->Self{
        Self{
            repository: repo
        }
    }
    pub async fn get_tasks(&self, request: GetOnGoingTasksRequest) -> Result<GetGoingTasksResponse, Report<GetTasksError>>{
        let result = self.repository.find_tasks(request.clone().user_id).await
            .change_context(GetTasksError::DatabaseError)?;

        let mut filtered = Vec::<Task>::new();
        for t in result {
            if t.is_done == request.done{
                filtered.push(t);
            }
        }
        filtered.sort_by(|t1,t2| {
            if t1.due > t2.due { Ordering::Greater }
            else if t1.due == t2.due {Ordering::Equal}
            else { Ordering::Less }
        });

        if filtered.is_empty() {
            return Ok(GetGoingTasksResponse{ retrieved: 0 , tasks: Vec::new()});
        }
        let mut tasks = Vec::<TaskDTO>::new();

        let start = min((&request).skip,(&filtered.len())-1);
        let end = min((&request).skip +(&request).count - 1,(&filtered).len()-1)+1;
        for t in &filtered[start..end] {
            tasks.push(TaskDTO::from(t.clone()));
        }
        Ok(GetGoingTasksResponse { retrieved: *(&tasks.len()), tasks })
    }
}