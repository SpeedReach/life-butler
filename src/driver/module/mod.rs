use std::fmt::format;
use std::sync::Arc;
use crate::application::use_case::user::delete_email_user::DeleteEmailUserUseCase;
use crate::application::use_case::user::register_user::RegisterUserUseCase;
use crate::application::use_case::user::user_login::UserLoginUseCase;
use crate::infrastructure::database::database_service::{DatabaseConfig, DatabaseDriver};
use crate::infrastructure::modules::RepositoriesModule;

pub struct Modules{
    pub register_user_use_case: RegisterUserUseCase,
    pub user_login_use_case: UserLoginUseCase,
    pub delete_email_user_use_case: DeleteEmailUserUseCase
}

impl Modules{

    pub async fn new(password: &str)->Modules {
        let driver= Arc::new(DatabaseDriver::new(DatabaseConfig::new(format!("mongodb+srv://brian920128:{}@cluster0.hek6yds.mongodb.net/?retryWrites=true&w=majority",password), "life-butler"))
            .await
            .unwrap());
        let repositories = Arc::new(RepositoriesModule::new(driver).await);

        Self{
            register_user_use_case: RegisterUserUseCase::new(Arc::new((&repositories).user_repository.clone())),
            user_login_use_case: UserLoginUseCase::new(Arc::new((&repositories).user_repository.clone())),
            delete_email_user_use_case: DeleteEmailUserUseCase::new(Arc::new((&repositories).user_repository.clone()))
        }
    }


}