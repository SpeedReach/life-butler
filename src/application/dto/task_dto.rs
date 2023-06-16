use chrono::{DateTime, FixedOffset, Utc};
use serde::Serialize;
use crate::infrastructure::database::entities::task::Task;
use crate::shared::utils::date_time::utc_to_utc8;

#[derive(Serialize)]
pub struct TaskDTO{
    pub id: String,
    pub owner: String,
    pub title: String,
    pub due: DateTime<FixedOffset>,
    pub description: String,
    pub is_done: bool
}


impl From<Task> for TaskDTO{
    fn from(value: Task) -> Self {
        Self{
            id: value.id,
            owner: value.owner,
            title: value.title,
            due: utc_to_utc8(value.due.to_chrono()),
            description: value.description,
            is_done: value.is_done
        }
    }
}