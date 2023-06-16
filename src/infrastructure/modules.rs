use std::sync::Arc;
use crate::infrastructure::database::database_service::DatabaseDriver;
use crate::infrastructure::repositories::event::EventRepository;
use crate::infrastructure::repositories::task::TaskRepository;
use crate::infrastructure::repositories::user::UserRepository;

pub struct RepositoriesModule{
    pub user_repository: UserRepository,
    pub event_repository: EventRepository,
    pub task_repository: TaskRepository
}


impl RepositoriesModule {

    pub async fn new(driver: Arc< DatabaseDriver>)->Self{
        let user_repository = UserRepository::new(&driver).await;
        let event_repository = EventRepository::new(&driver).await;
        let task_repository = TaskRepository::new(&driver).await;
        Self{
            user_repository,
            event_repository,
            task_repository
        }
    }

}