use axum::async_trait;
use bson::doc;
use error_stack::{IntoReport, Report, ResultExt};
use mongodb::Collection;
use crate::infrastructure::database::entities::task::Task;
use crate::infrastructure::error::DatabaseError;
use crate::infrastructure::repositories::task::TaskRepository;
use crate::infrastructure::results::insert_result::InsertResult;

#[async_trait]
pub trait InsertTaskRepository {
    async fn insert_task(&self, task: Task) -> Result<InsertResult<Task>, Report<DatabaseError>>;
}

#[async_trait]
impl InsertTaskRepository for TaskRepository{
    async fn insert_task(&self, task: Task) -> Result<InsertResult<Task>, Report<DatabaseError>> {
        let collection: Collection<Task> = self.get_collection();
        let result = collection.insert_one(task.clone(),None)
            .await
            .into_report()
            .change_context(DatabaseError)?;
        return Ok(InsertResult::Success(task));
    }
}