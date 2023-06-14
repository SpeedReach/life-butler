use crate::driver;
use crate::infrastructure::database::entities::user::User;
use axum::response::IntoResponse;
use axum::Json;
use axum::{http::StatusCode, response::Response};
use error_stack::Context;
use serde::{Deserialize, Serialize};
use crate::driver::models::HttpResponse;


#[derive(Deserialize, Debug)]
pub struct RegisterUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterUserResponse {
    id: String,
    email: String,
    password: String
}

impl From<User> for RegisterUserResponse{
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            email: value.email,
            password: value.password
        }
    }
}


