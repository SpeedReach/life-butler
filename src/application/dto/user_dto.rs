use serde::{Deserialize, Serialize};
use crate::infrastructure::database::entities::user::User;


#[derive(Serialize,Deserialize)]
pub struct UserDTO{
    user_id: String,
    email: String
}


impl From<User> for UserDTO{
    fn from(value: User) -> Self {
        UserDTO{
            user_id: value.id,
            email: value.email,
        }
    }
}