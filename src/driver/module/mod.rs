use std::fmt::format;
use std::sync::Arc;
use crate::infrastructure::database::database_service::{DatabaseConfig, DatabaseDriver};
use crate::infrastructure::modules::RepositoriesModule;

pub struct Modules{
    pub repositories: RepositoriesModule
}

impl Modules{

    pub async fn new(password: &str)->Modules {
        let driver= Arc::new(DatabaseDriver::new(DatabaseConfig::new(format!("mongodb+srv://brian920128:{}@cluster0.hek6yds.mongodb.net/?retryWrites=true&w=majority",password), "life-butler"))
            .await
            .unwrap());
        let repositories = RepositoriesModule::new(driver).await;
        
        Self{
            repositories,
        }
    }
}