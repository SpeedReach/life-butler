use chrono::{DateTime, FixedOffset, Utc};
use serde::Serialize;
use crate::infrastructure::database::entities::task::Task;
use crate::shared::utils::date_time::utc_to_utc8;

#[derive(Serialize,Clone)]
pub struct TaskDTO{
    pub id: String,
    pub owner: String,
    pub title: String,
    pub due: DateTime<FixedOffset>,
    pub description: String,
    pub is_done: bool,
    pub expired: bool
}


impl From<Task> for TaskDTO{
    fn from(value: Task) -> Self {
        let due = utc_to_utc8(value.due.to_chrono());
        Self{
            id: value.id,
            owner: value.owner,
            title: value.title,
            due,
            description: value.description,
            is_done: value.is_done,
            expired: false,
        }
    }
}