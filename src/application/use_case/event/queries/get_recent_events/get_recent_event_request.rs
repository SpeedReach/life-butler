use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetRecentEventRequest{
    pub user_id: String,
    pub skip: u64,
    pub count: i64
}