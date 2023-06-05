
#[cfg(test)]
mod user{
    use std::arch::asm;
    use std::sync::Arc;
    use error_stack::Report;
    use life_butler::driver::module::Modules;
    use life_butler::infrastructure::database::entities::user::User;
    use life_butler::infrastructure::error::OperationError;
    use life_butler::infrastructure::repositories::user::user_repository::UserRepository;
    use mongodb::results::DeleteResult;
    use crate::binary::setup;

    static TEST_EMAIL: &str = "brian030128@gmail.com";

    pub async fn create_user(user_repository: &UserRepository) ->  Result<User, Report<OperationError>>{
        return user_repository.create_user(TEST_EMAIL.to_owned(),"123456".to_owned()).await;
    }

    pub async fn delete_user_email(user_repository: &UserRepository)->  Result<DeleteResult, Report<OperationError>> {
        return user_repository.delete_user_with_email(TEST_EMAIL.to_owned()).await;
    }

    #[tokio::test]
    pub async fn create_delete(){
        let modules = setup().await;
        delete_user_email(&modules.repositories.user_repository).await;
        let create_res_1= create_user(&modules.repositories.user_repository).await;
        assert!(create_res_1.is_ok(), "failed creating user");
        let create_res_2 = create_user(&modules.repositories.user_repository).await;
        print!("{}",create_res_2.err().unwrap());
        let del_res_1=delete_user_email(&modules.repositories.user_repository).await;
        assert_eq!(del_res_1.unwrap().deleted_count,1);
    }

}