use serde::{Deserialize, Serialize};

pub mod sleep_request;


#[derive(Deserialize,Clone)]
pub struct SleepRequest{
    pub sleep: bool,
    pub user_id: String
}