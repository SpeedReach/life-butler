use serde::Serialize;
use crate::application::dto::task_dto::TaskDTO;

#[derive(Serialize)]
pub struct CreateTaskResponse {
    pub event: TaskDTO
}