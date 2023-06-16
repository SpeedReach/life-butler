use axum::async_trait;
use bson::doc;
use error_stack::{IntoReport, Report, ResultExt};
use crate::infrastructure::error::DatabaseError;
use crate::infrastructure::repositories::task::TaskRepository;

#[async_trait]
pub trait UpdateTaskStatusRepository {
    async fn update_task_status(&self, task_id: String,status: bool) -> Result<(), Report<DatabaseError>>;
}

#[async_trait]
impl UpdateTaskStatusRepository for TaskRepository{
    async fn update_task_status(&self, task_id: String, status: bool) -> Result<(), Report<DatabaseError>> {
        let result =self.get_collection().update_one(doc! {"_id": task_id}, doc! {"$set": { "is_done": status }}, None)
            .await
            .into_report()
            .change_context(DatabaseError)?;
        Ok(())
    }
}