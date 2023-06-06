
#[cfg(test)]
mod user{
    use std::arch::asm;
    use std::sync::Arc;
    use error_stack::Report;
    use life_butler::driver::module::Modules;
    use life_butler::infrastructure::database::entities::user::User;
    use life_butler::infrastructure::error::DatabaseError;
    use life_butler::infrastructure::repositories::user::UserRepository;
    use life_butler::infrastructure::repositories::user::create_user::CreateUserRepository;
    use life_butler::infrastructure::repositories::user::delete_email_user::DeleteEmailUserRepository;
    use life_butler::infrastructure::results::create_result::CreateResult;
    use life_butler::infrastructure::results::delete_result::DeleteResult;
    use crate::binary::setup;

    static TEST_EMAIL: &str = "brian030128@gmail.com";

    pub async fn create_user(user_repository: &UserRepository) ->  Result<CreateResult<User>, Report<DatabaseError>>{
        return user_repository.create_user(TEST_EMAIL.to_owned(),"123456".to_owned()).await;
    }

    pub async fn delete_user_email(user_repository: &UserRepository)->  Result<DeleteResult, Report<DatabaseError>> {
        return user_repository.delete_email_user(TEST_EMAIL.to_owned()).await;
    }

    #[tokio::test]
    pub async fn create_delete() -> Result<(), Report<DatabaseError>>{
        let modules = setup().await;
        delete_user_email(&modules.repositories.user_repository).await?;
        let create_res_1= create_user(&modules.repositories.user_repository).await?;
        match create_res_1 {
            CreateResult::AlreadyExists(email) => panic!("Could not create user {email} because it already exists"),
            CreateResult::Success(user) =>  assert!(user.email == TEST_EMAIL)
        }

        let create_res_2 = create_user(&modules.repositories.user_repository).await?;
        match create_res_2 {
            CreateResult::Success(_) => panic!("Creating user with existing email should fail"),
            CreateResult::AlreadyExists(email) => assert!(email == TEST_EMAIL)
        }

        let del_res_1=delete_user_email(&modules.repositories.user_repository).await?;
        match del_res_1 {
            DeleteResult::Success(amount) => assert!(amount == 1, "Should only delete 1 users with email {}, instead deleted {}",TEST_EMAIL,amount),
            DeleteResult::NotFound => panic!("Should delete 1 existing user with email {}",TEST_EMAIL)
        }
        Ok(())
    }

}