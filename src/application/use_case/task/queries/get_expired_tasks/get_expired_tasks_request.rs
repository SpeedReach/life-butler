use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Clone)]
pub struct GetExpiredTasksRequest{
    pub user_id: String,
    pub skip: usize,
    pub count: usize
}