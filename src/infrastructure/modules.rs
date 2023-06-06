use std::sync::Arc;
use crate::infrastructure::database::database_service::DatabaseDriver;
use crate::infrastructure::repositories::user::UserRepository;

pub struct RepositoriesModule{
    pub user_repository: UserRepository
}


impl RepositoriesModule {

    pub async fn new(driver: Arc< DatabaseDriver>)->Self{
        let user_repository = UserRepository::new(&driver).await;

        Self{
            user_repository,
        }
    }

}