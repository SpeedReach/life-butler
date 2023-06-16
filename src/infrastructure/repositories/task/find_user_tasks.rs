use axum::async_trait;
use bson::doc;
use error_stack::{IntoReport, Report, ResultExt};
use futures::StreamExt;
use crate::infrastructure::database::entities::task::Task;
use crate::infrastructure::error::DatabaseError;
use crate::infrastructure::repositories::task::TaskRepository;

#[async_trait]
pub trait FindUserTasksRepository{
    async fn find_tasks(&self,user_id: String) -> Result<Vec<Task>,Report<DatabaseError>>;
}

#[async_trait]
impl FindUserTasksRepository for TaskRepository {
    async fn find_tasks(&self,user_id: String) -> Result<Vec<Task>,Report<DatabaseError>> {
        let mut cursor = self.get_collection().find(doc! {"owner": user_id}, None)
            .await
            .into_report()
            .change_context(DatabaseError)?;
        let mut tasks = Vec::<Task>::new();
        while let Some(result) = cursor.next().await{
            let task = result.into_report().change_context(DatabaseError)?;
            tasks.push(task);
        }
        Ok(tasks)
    }
}