use std::sync::Arc;
use crate::infrastructure::database::database_service::DatabaseDriver;
use crate::infrastructure::repositories::user::user_repository::UserRepository;

pub struct RepositoriesModule{
    pub user_repository: UserRepository
}


impl RepositoriesModule {

    pub fn new(driver: Arc< DatabaseDriver>)->Self{
        let user_repository = UserRepository::new(Arc::clone(&driver));

        Self{
            user_repository,
        }
    }

}