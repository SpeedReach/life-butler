use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateTaskRequest{
    pub task_id: String,
    pub is_done: bool
}

