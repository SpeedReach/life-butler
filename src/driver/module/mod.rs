use std::sync::Arc;
use crate::infrastructure::database::database_service::{DatabaseConfig, DatabaseDriver};
use crate::infrastructure::modules::RepositoriesModule;

pub struct Modules{
    pub repositories: RepositoriesModule
}

impl Modules{

    pub async fn new()->Modules {
        let driver = Arc::new(DatabaseDriver::new(DatabaseConfig::new("mongodb+srv://brian920128:c177267w@cluster0.hek6yds.mongodb.net/?retryWrites=true&w=majority".to_owned(), "life-butler"))
            .await
            .unwrap());
        let repositories = RepositoriesModule::new(driver);

        Self{
            repositories,
        }
    }

}