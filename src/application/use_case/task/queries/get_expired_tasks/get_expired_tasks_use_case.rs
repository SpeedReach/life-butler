use std::cmp::{min, Ordering};
use std::sync::Arc;
use chrono::Utc;
use error_stack::{Report, ResultExt};
use crate::application::dto::task_dto::TaskDTO;
use crate::application::use_case::task::queries::get_expired_tasks::get_expired_tasks_request::GetExpiredTasksRequest;
use crate::application::use_case::task::queries::get_expired_tasks::get_expired_tasks_response::GetExpiredTasksResponse;
use crate::application::use_case::task::queries::GetTasksError;
use crate::infrastructure::database::entities::task::Task;
use crate::infrastructure::repositories::task::find_user_tasks::FindUserTasksRepository;
use crate::infrastructure::repositories::task::TaskRepository;

pub struct GetExpiredTasksUseCase{
    repository: Arc<dyn FindUserTasksRepository + Send + Sync>
}

impl GetExpiredTasksUseCase{

    pub fn new(repo: Arc<TaskRepository>)->Self{
        Self{
            repository: repo
        }
    }

    pub async fn get_tasks(&self, request: GetExpiredTasksRequest) -> Result<GetExpiredTasksResponse, Report<GetTasksError>>{
        let result = self.repository.find_tasks(request.clone().user_id).await
            .change_context(GetTasksError::DatabaseError)?;

        let mut filtered = Vec::<Task>::new();
        let now = Utc::now();
        for t in result {
            if t.due.to_chrono() < now && !t.is_done {
                filtered.push(t);
            }
        }
        if filtered.is_empty(){
            return Ok(GetExpiredTasksResponse{retrieved: 0, tasks: Vec::new()});
        }
        filtered.sort_by(|t1,t2| {
            if t1.due > t2.due { Ordering::Less }
            else if t1.due == t2.due {Ordering::Equal}
            else { Ordering::Greater}
        });
        let mut tasks = Vec::<TaskDTO>::new();
        let start = min((&request).skip,(&filtered.len())-1);
        let end = min((&request).skip +(&request).count - 1,(&filtered).len()-1)+1;
        for t in &filtered[start..end] {
            tasks.push(TaskDTO::from(t.clone()));
        }
        Ok(GetExpiredTasksResponse{ retrieved: *(&tasks.len()), tasks })
    }
}