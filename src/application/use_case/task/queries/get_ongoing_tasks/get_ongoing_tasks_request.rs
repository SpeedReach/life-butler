use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Clone)]
pub struct GetOnGoingTasksRequest {
    pub user_id: String,
    pub skip: usize,
    pub count: usize,
    pub done: bool
}