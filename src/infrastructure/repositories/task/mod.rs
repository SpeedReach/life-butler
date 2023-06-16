pub mod insert_task;

use std::sync::Arc;
use crate::infrastructure::database::database_service::DatabaseDriver;
use crate::infrastructure::database::entities::task::Task;

#[derive(Clone)]
pub struct TaskRepository{
    driver: Arc<DatabaseDriver>,
    database_id: String,
    collection_id: String
}

impl TaskRepository{
    pub async fn new(driver: &Arc<DatabaseDriver>) -> TaskRepository {

        Self {
            driver: Arc::clone(driver),
            database_id: driver.get_config().database_id,
            collection_id: String::from("task")
        }
    }


    fn get_collection(&self) -> mongodb::Collection<Task> {
        self.driver
            .get_database()
            .collection::<Task>(&self.collection_id)
    }

}