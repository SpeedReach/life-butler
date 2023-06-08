mod common;




#[cfg(test)]
mod user{
    use error_stack::{Report, ResultExt};
    use life_butler::application::use_case::user::delete_email_user::DeleteEmailUserError;
    use life_butler::application::use_case::user::register_user::RegisterUserError;
    use life_butler::driver::module::Modules;
    use life_butler::infrastructure::database::entities::user::User;
    use crate::common::{setup, TestError};

    static TEST_EMAIL: &str = "brian030128@gmail.com";

    pub async fn create_user(modules: &Modules) ->  Result<User, Report<RegisterUserError>>{
        return  modules.register_user_use_case.register_user(TEST_EMAIL.to_owned(),"123456".to_owned()).await;
    }

    pub async fn delete_user_email(modules: &Modules)->  Result<(), Report<DeleteEmailUserError>> {
        return modules.delete_email_user_use_case.delete_email_user(TEST_EMAIL.to_owned()).await;
    }

    #[tokio::test]
    pub async fn create_delete_test() -> Result<(), Report<TestError>>{
        let modules = setup().await;
        let _ = delete_user_email(modules).await.change_context(TestError{});
        
        create_user(&modules).await.change_context(TestError {})?;
        
        let create_res_2 = create_user(&modules).await.change_context(TestError{});
        
        match create_res_2 {
            Ok(_) => panic!("should not be able to create another user with same password"),
            Err(_) => {},
        }
        
        delete_user_email(&modules).await.change_context(TestError)?;
        
        Ok(())
    }

}