use serde::Serialize;
use crate::application::dto::task_dto::TaskDTO;

#[derive(Serialize)]
pub struct GetGoingTasksResponse {
    pub retrieved: usize,
    pub tasks: Vec<TaskDTO>
}