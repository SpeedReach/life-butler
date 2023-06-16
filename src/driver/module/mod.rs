use std::fmt::format;
use std::sync::Arc;
use crate::application::use_case::event::commands::create_event::create_event_use_case::CreateEventUseCase;
use crate::application::use_case::event::queries::get_recent_events::get_recent_event_use_case::GetRecentEventUseCase;
use crate::application::use_case::user::delete_email_user::DeleteEmailUserUseCase;
use crate::application::use_case::user::register_user::RegisterUserUseCase;
use crate::application::use_case::user::user_login::UserLoginUseCase;
use crate::infrastructure::database::database_service::{DatabaseConfig, DatabaseDriver};
use crate::infrastructure::modules::RepositoriesModule;
use crate::infrastructure::repositories::event::find_recent_events::FindRecentEventRepository;

pub struct Modules{
    pub register_user_use_case: RegisterUserUseCase,
    pub user_login_use_case: UserLoginUseCase,
    pub delete_email_user_use_case: DeleteEmailUserUseCase,
    pub create_event_use_case: CreateEventUseCase,
    pub get_recent_events_use_case: GetRecentEventUseCase,
    
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
            delete_email_user_use_case: DeleteEmailUserUseCase::new(Arc::new((&repositories).user_repository.clone())),
            create_event_use_case: CreateEventUseCase::new(Arc::new((&repositories).event_repository.clone())),
            get_recent_events_use_case: GetRecentEventUseCase::new(Arc::new((&repositories).event_repository.clone()))
        }
    }


}